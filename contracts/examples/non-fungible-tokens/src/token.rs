use elrond_wasm::{Vec, Address};
use elrond_wasm::api::BigUintApi;

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Token<BigUint: BigUintApi> {
     pub price: BigUint,
     pub init_price: BigUint,
     pub uri: Vec<u8>,
     pub secret:Vec<u8>,
     pub state:u8,
     pub min_price:BigUint,
     pub max_price:BigUint,
     pub sellers:Vec<Address>,
     pub percents:Vec<u16>,
     pub seller_owner:u8
}

//seller_owner=1 ou 3:le propriétaire peut offrir
//seller_owner=2 ou 3:le propriétaire peut vendre



