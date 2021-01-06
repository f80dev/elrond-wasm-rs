#![no_std]

imports!();

mod token;
use token::Token;

#[elrond_wasm_derive::contract(NonFungibleTokensImpl)]
pub trait NonFungibleTokens {
	#[init]
	fn init(&self) {
		let owner = self.get_caller();
		self.set_owner(&owner);
		self.set_total_minted(0);
	}

	// endpoints

	/// Creates new tokens and sets their ownership to the specified account.
	/// Only the contract owner may call this function.
	#[endpoint]
	fn mint(&self, count: u64, new_token_owner: Address, new_token_uri: &Vec<u8>,secret: &Vec<u8>, new_token_price: BigUint) -> SCResult<u64> {
		require!(self.get_caller() == self.get_owner(),"Only owner can mint new tokens!");
		require!(count>0,"At least one token must be mined");
		require!(new_token_uri.len() > 0,"URI can't be empty");

		let token_id=self.perform_mint(count, &new_token_owner, new_token_uri, secret, new_token_price);
		Ok(token_id)
	}


	/// Approves an account to transfer the token on behalf of its owner.<br>
	/// Only the owner of the token may call this function.
	#[endpoint]
	fn approve(&self, token_id: u64, approved_address: Address) -> SCResult<()> {
		require!(token_id < self.get_total_minted(), "Token does not exist!");
		require!(self.get_caller() == self.get_token_owner(token_id),"Only the token owner can approve!");

		self.set_approval(token_id, &approved_address);

		Ok(())
	}



	/// Revokes approval for the token.<br>  
	/// Only the owner of the token may call this function.
	#[endpoint]
	fn revoke(&self, token_id: u64) -> SCResult<()> {
		require!(token_id < self.get_total_minted(), "Token does not exist!");
		require!(self.get_caller() == self.get_token_owner(token_id),"Only the token owner can revoke approval!");

		if !self.approval_is_empty(token_id) {
			self.perform_revoke_approval(token_id);
		}

		Ok(())
	}


	#[endpoint]
	fn open(&self, token_id: u64) -> SCResult<Vec<u8>> {
		require!(token_id < self.get_total_minted(), "Token does not exist!");

		let caller = self.get_caller();
		let token_owner = self.get_token_owner(token_id);

		let token=self.get_mut_token(token_id);
		let secret=token.secret.to_vec();

		if caller == token_owner {
			return Ok(secret);
		} else if !self.approval_is_empty(token_id) {
			let approved_address = self.get_approval(token_id);
			if caller == approved_address {
				return Ok(secret);
			}
		}

		sc_error!("You are not the owner of this token")
	}


	/// Transfer ownership of the token to a new account.
	#[endpoint]
	fn transfer(&self, token_id: u64, to: Address) -> SCResult<()> {
		require!(token_id < self.get_total_minted(), "Token does not exist!");

		let caller = self.get_caller();
		let token_owner = self.get_token_owner(token_id);

		if caller == token_owner {
			self.perform_transfer(token_id, &token_owner, &to);
			return Ok(());
		} else if !self.approval_is_empty(token_id) {
			let approved_address = self.get_approval(token_id);

			if caller == approved_address {
				self.perform_transfer(token_id, &token_owner, &to);
				return Ok(());
			}
		}

		sc_error!("Only the owner or the approved account may transfer the token!")
	}




	// private methods
	fn perform_mint(&self, count:u64, new_token_owner: &Address, new_token_uri: &Vec<u8>, secret: &Vec<u8>, new_token_price: BigUint) -> u64 {
		let new_owner_current_total = self.get_token_count(new_token_owner);
		let total_minted = self.get_total_minted();
		let first_new_id = total_minted;
		let last_new_id = total_minted + count;

		for id in first_new_id..last_new_id {
			let token = Token {
				price:new_token_price.clone(),
				uri:new_token_uri.to_vec(),
				secret:secret.to_vec(),
				state:0 as u8,
			};
			self.set_token(id, &token);
			self.set_token_owner(id,new_token_owner);
			self.set_token_miner(id,new_token_owner);
		}

		self.set_total_minted(total_minted + count);
		self.set_token_count(new_token_owner, new_owner_current_total + count);
		return last_new_id;
	}




	fn perform_revoke_approval(&self, token_id: u64) {
		// clear at key "''approval|token_id"
		self.clear_approval(token_id);
	}



	fn perform_transfer(&self, token_id: u64, from: &Address, to: &Address) {
		let prev_owner_token_count = self.get_token_count(from);
		let new_owner_token_count = self.get_token_count(to);

		// new ownership revokes approvals by previous owner
		self.perform_revoke_approval(token_id);

		self.set_token_count(from, prev_owner_token_count - 1);
		self.set_token_count(to, new_owner_token_count + 1);
		self.set_token_owner(token_id, to);
	}


	// Transfer ownership of the token to a new account.
	#[endpoint]
	fn burn(&self, token_id: u64) -> SCResult<()> {
		require!(token_id < self.get_total_minted(), "Token does not exist!");

		let caller = self.get_caller();
		let token_owner = self.get_token_owner(token_id);
		let token_miner = self.get_token_miner(token_id);

		if caller == token_owner || caller == token_miner {
			let zero=Address::zero();
			self.set_token_miner(token_id,&zero);
			self.perform_transfer(token_id, &token_owner, &zero);
			return Ok(());
		}

		sc_error!("Only the owner account can burn token!")
	}


	#[endpoint]
	fn setstate(&self,  token_id: u64,new_state:u8) -> SCResult<()> {
		require!(self.get_token_owner(token_id) == self.get_caller(),"Only token owner can change the state");

		let mut token = self.get_mut_token(token_id);
		token.state=new_state;
		self.set_token(token_id,&token);

		Ok(())
	}


	#[payable]
	#[endpoint]
	fn buy(&self, #[payment] payment: BigUint, token_id: u64) -> SCResult<()> {
		let mut token = self.get_mut_token(token_id);
		let owner=self.get_token_owner(token_id);
		let caller=self.get_caller();

		require!(owner != caller,"Ce token vous appartient déjà");
		require!(token.state == 0,"Ce token n'est pas en vente");
		require!(payment >= token.price,"Montant inferieur au prix");


		token.state=1;

		self.set_token(token_id,&token);

		self.perform_transfer(token_id,&owner,&caller);

		self.send_tx(
			&owner,
			&payment,
			"Reglement".as_bytes(),
		);

		return Ok(());
	}



	
	#[view(tokens)]
	fn get_tokens(&self,sep:u8,cr:u8,owner_filter: Address, miner_filter: Address) -> Vec<Vec<u8>> {
		let mut rc=Vec::new();

		//TODO: fonctionnement non viable sur les séparateur, risque de mauvaises séparation
		let total_minted = self.get_total_minted();
		for i in 0..total_minted {
			let mut token=self.get_mut_token(i);
			let owner=self.get_token_owner(i);
			let miner=self.get_token_miner(i);

			if (owner_filter == Address::zero() || owner_filter == owner) && (miner_filter == Address::zero() || miner_filter == miner) {
				let mut item=token.price.to_bytes_be();
				for _i in 0..4 {item.push(sep);}

				item.append(&mut owner.to_vec());
				item.push(token.state);
				item.append(&mut token.uri);

				//Separator
				for _i in 0..4 {item.push(cr);}

				rc.push(item);
			}

		}
		return rc;
	}


	#[view(contractOwner)]
	#[storage_get("owner")]
	fn get_owner(&self) -> Address;
	#[storage_set("owner")]
	fn set_owner(&self, owner: &Address);



	#[view(tokenOwner)]
	#[storage_get("tokenOwner")]
	fn get_token_owner(&self, token_id: u64) -> Address;
	#[storage_set("tokenOwner")]
	fn set_token_owner(&self, token_id: u64, owner: &Address);


	#[view(totalMinted)]
	#[storage_get("totalMinted")]
	fn get_total_minted(&self) -> u64;

	#[storage_set("totalMinted")]
	fn set_total_minted(&self, total_minted: u64);


	#[view(tokenCount)]
	#[storage_get("tokenCount")]
	fn get_token_count(&self, owner: &Address) -> u64;
	#[storage_set("tokenCount")]
	fn set_token_count(&self, owner: &Address, token_count: u64);


	#[view(getToken)]
	#[storage_get_mut("token")]
	fn get_mut_token(&self,  token_id: u64) -> mut_storage!(Token<BigUint>);
	#[storage_set("token")]
	fn set_token(&self, token_id: u64, token: &Token<BigUint>);



	#[storage_is_empty("approval")]
	fn approval_is_empty(&self, token_id: u64) -> bool;


	#[view(approval)]
	#[storage_get("approval")]
	fn get_approval(&self, token_id: u64) -> Address;
	#[storage_set("approval")]
	fn set_approval(&self, token_id: u64, approved_address: &Address);

	#[view(tokenMiner)]
	#[storage_get("tokenMiner")]
	fn get_token_miner(&self, token_id: u64) -> Address;
	#[storage_set("tokenMiner")]
	fn set_token_miner(&self, token_id: u64, miner_address: &Address);

	#[storage_clear("approval")]
	fn clear_approval(&self, token_id: u64);
}
