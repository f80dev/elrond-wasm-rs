#![no_std]
#![allow(clippy::too_many_arguments)]

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
	fn mint(&self,
			count: u64,
			new_token_uri: &Vec<u8>,
			secret: &Vec<u8>,
			new_token_price: BigUint,
			min_price: BigUint,
			max_price:BigUint,
			owner_seller:u8
	) -> SCResult<u64> {
		let caller=self.get_caller();
		require!(count>0,"At least one token must be mined");
		require!(new_token_uri.len() > 0,"URI can't be empty");

		let token_id=self.perform_mint(count,&caller,new_token_uri,secret,new_token_price,min_price,max_price,owner_seller);

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

	// fn decrypt(&self, encrypted:&Vec<u8>) -> Vec<u8> {
	// 	let a:u8=12;
	// 	for u in encrypted {
	// 		u = &(u ^ a);
	// 	}
	// 	return encrypted.to_vec();
	// }


	#[endpoint]
	fn open(&self, token_id: u64) -> SCResult<Vec<u8>> {
		require!(token_id < self.get_total_minted(), "Token does not exist!");

		let caller = self.get_caller();
		let token_owner = self.get_token_owner(token_id);

		let token=self.get_token(token_id);
		let secret=token.secret.to_vec();
		//secret=self.decrypt(&secret);


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
		let token=self.get_token(token_id);

		require!(token.seller_owner==1 || token.seller_owner==3,"Ce token ne peut être offert");


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
	fn perform_mint(&self, count:u64,
					new_token_owner: &Address,
					new_token_uri: &Vec<u8>,
					secret: &Vec<u8>,
					new_token_price: BigUint,
					min_price: BigUint,max_price: BigUint,
					owner_seller:u8) -> u64 {
		let new_owner_current_total = self.get_token_count(new_token_owner);
		let total_minted = self.get_total_minted();
		let first_new_id = total_minted;
		let last_new_id = total_minted + count;

		for id in first_new_id..last_new_id {
			let token = Token {
				init_price:new_token_price.clone(),
				price:new_token_price.clone(),
				uri:new_token_uri.to_vec(),
				secret:secret.to_vec(),
				state:0 as u8,
				min_price:min_price.clone(),
				max_price:max_price.clone(),
				sellers:Vec::new(),
				percents:Vec::new(),
				seller_owner:owner_seller
			};

			self.set_token(id, &token);
			self.set_token_owner(id,new_token_owner);
			self.set_token_miner(id,new_token_owner);
		}

		self.set_total_minted(total_minted + count);
		self.set_token_count(new_token_owner, new_owner_current_total + count);
		return last_new_id;
	}




	//
	// fn u64_to_str(&self,val:u64) -> String {
	// 	let mut rc=String::new();
	// 	for i in 10..0 {
	// 		let chiffre=val/(10^i);
	// 		let a=match chiffre {
	// 			0 => '0',
	// 			1 => '1',
	// 			2 => '2',
	// 			3 => '3',
	// 			4 => '4',
	// 			5 => '5',
	// 			6 => '6',
	// 			7 => '7',
	// 			8 => '8',
	// 			9 => '9',
	// 			_ => ' '
	// 		};
	// 		rc.push(a);
	// 	}
	// 	return rc;
	// }


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


	//Seul le propriétaire du token peut le remettre en vente
	#[endpoint]
	fn setstate(&self,  token_id: u64,new_state:u8) -> SCResult<()> {
		let mut token = self.get_token(token_id);
		let caller=self.get_caller();

		require!(self.get_token_owner(token_id) == caller,"Only token owner or reseller can change the state");
		require!(token.seller_owner==2 || token.seller_owner==3,"Le créateur du token n'autorise pas le propriétaire à le vendre");

		token.state=new_state;
		self.set_token(token_id,&token);

		Ok(())
	}



	#[endpoint]
	fn add_dealer(&self,  token_id: u64, addr: Address, percent:u16) -> SCResult<()> {
		let caller=self.get_caller();
		let owner=self.get_token_owner(token_id);

		require!(owner == caller,"Only token owner can add dealer");
		//TODO: ajouter un require pour s'assurer que le developpeur n'est pas déjà dans la liste

		let mut token = self.get_token(token_id);
		token.sellers.push(addr);
		token.percents.push(percent);
		self.set_token(token_id,&token);

		Ok(())
	}

	#[endpoint]
	fn clear_dealer(&self,  token_id: u64) -> SCResult<()> {
		let mut token = self.get_token(token_id);
		let caller=self.get_caller();

		require!(self.get_token_owner(token_id) == caller,"Only token owner can clear the dealer list");

		token.sellers=Vec::new();

		self.set_token(token_id,&token);

		Ok(())
	}





	#[endpoint]
	fn price(&self, token_id: u64, new_price: BigUint) -> SCResult<()> {
		let mut token = self.get_token(token_id);

		let owner = self.get_token_owner(token_id);
		let caller = self.get_caller();
		require!(owner == caller, "Seul le propriétaire du token peut modifier le prix");
		require!(&new_price <= &token.max_price,"Vous ne pouvez augmenter autant le prix");
		require!(&new_price >= &token.min_price,"Vous ne pouvez augmenter autant le prix");

		token.price = new_price;
		self.set_token(token_id,&token);

		return Ok(())
	}


	fn vector_as_u8_8_array(&self,vector: Vec<u8>) -> [u8;8] {
		let mut arr = [0u8;8];
		for (place, element) in arr.iter_mut().zip(vector.iter()) {
			*place = *element;
		}
		arr
	}

	#[payable("EGLD")]
	#[endpoint]
	fn buy(&self, #[payment] payment: BigUint, token_id: u64,seller:Address) -> SCResult<&str> {
		let mut token = self.get_token(token_id);
		let owner=self.get_token_owner(token_id);
		let caller=self.get_caller();

		let idx_seller = token.sellers.iter().position(|x| x == &seller).unwrap_or(1000);

		require!(seller==Address::zero() || idx_seller<1000 ,"Le revendeur n'est pas autorisé");
		require!(owner != caller,"Ce token vous appartient déjà");
		require!(token.state == 0,"Ce token n'est pas en vente");
		require!(payment >= token.price,"Montant inferieur au prix");

		//Le token n'est plus a vendre
		token.state=1;

		self.set_token(token_id,&token);
		self.perform_transfer(token_id,&owner,&caller);

		//Versement au vendeur
		let mut payment_for_owner=payment.clone();
		let mut message:&str=stringify!("Reglement du owner uniquement");

		if seller!=Address::zero() {
			//Transaction issue d'un revendeur

			let percent_for_dealer=token.percents[idx_seller] as u64;

			if percent_for_dealer>0 {
				let payment_for_dealer=&payment/&BigUint::from(10000u64/&percent_for_dealer);
				payment_for_owner=&payment-&payment_for_dealer;

				self.send().direct_egld(
					&token.sellers[idx_seller],
					&payment_for_dealer,
					b"Reglement du seller"
				);
				message="reglement du seller et du owner";
			}
		}

		if payment_for_owner>0 {
			let mes_owner: &str=stringify!("Reglement du owner");
			// let x = &[0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
			// let consume_and_return_x = 10 || x;
			// using_encoded_number(pay,16,false,true,consume_and_return_x);

			self.send().direct_egld(
				&owner,
				&payment_for_owner,
				mes_owner.as_bytes()
			);
		}

		return Ok(message);
	}



	
	#[view(tokens)]
	fn get_tokens(&self,seller_filter: Address,owner_filter: Address, miner_filter: Address) -> Vec<Vec<u8>> {
		let mut rc=Vec::new();

		let total_minted = self.get_total_minted();
		for i in 0..total_minted {
			let mut token=self.get_token(i);
			let owner=self.get_token_owner(i);
			let miner=self.get_token_miner(i);

			let idx = token.sellers.iter().position(|x| x == &seller_filter).unwrap_or(1000);

			if (owner_filter == Address::zero() || owner_filter == owner)
				&& (miner_filter == Address::zero() || miner_filter == miner)
				&& (seller_filter == Address::zero() || idx<1000) {
				let mut item:Vec<u8>=Vec::new();

				//On commence par inscrire la taille de token_price, uri
				//doc sur le conversion :https://docs.rs/elrond-wasm/0.10.3/elrond_wasm/
				item.append(&mut token.uri.len().to_be_bytes().to_vec());

				//Puis on ajoute les informations
				item.append(&mut token.price.to_bytes_be_pad_right(10).unwrap_or(Vec::new()));
				item.append(&mut owner.to_vec());
				item.push(token.state);

				item.append(&mut i.to_be_bytes().to_vec());
				item.append(&mut token.uri);

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
	#[storage_get("token")]
	fn get_token(&self,  token_id: u64) -> Token<BigUint>;
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
