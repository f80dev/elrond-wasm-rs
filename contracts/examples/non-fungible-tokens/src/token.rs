use elrond_wasm::{BigUintApi, Vec};

derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Token<BigUint: BigUintApi> {
     pub price: BigUint,
     pub uri: Vec<u8>,
     pub secret:Vec<u8>,
     pub state:u8,
     pub update_price:u32,
}



