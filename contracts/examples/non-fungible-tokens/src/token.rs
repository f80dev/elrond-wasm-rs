use elrond_wasm::elrond_codec::*;
use elrond_wasm::{ BigUintApi, Vec};


pub struct Token<BigUint: BigUintApi> {
     pub price: BigUint,
     pub uri: Vec<u8>,
     pub state:u8,
}


impl<BigUint: BigUintApi> NestedEncode for Token<BigUint> {
     fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
          self.price.dep_encode(dest)?;
          self.uri.dep_encode(dest)?;
          self.state.dep_encode(dest)?;
          Ok(())
     }

     fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
          &self,
          dest: &mut O,
          c: ExitCtx,
          exit: fn(ExitCtx, EncodeError) -> !,
     ) {
          self.price.dep_encode_or_exit(dest, c.clone(), exit);
          self.uri.dep_encode_or_exit(dest, c.clone(), exit);
          self.state.dep_encode_or_exit(dest, c.clone(), exit);
     }
}

impl<BigUint: BigUintApi> TopEncode for Token<BigUint> {
     #[inline]
     fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
          top_encode_from_nested(self, output)
     }

     #[inline]
     fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
          &self,
          output: O,
          c: ExitCtx,
          exit: fn(ExitCtx, EncodeError) -> !,
     ) {
          top_encode_from_nested_or_exit(self, output, c, exit);
     }
}


impl<BigUint: BigUintApi> NestedDecode for Token<BigUint> {
     fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
          Ok(Token {
               price: BigUint::dep_decode(input)?,
               uri: Vec::<u8>::dep_decode(input)?,
               state: u8::dep_decode(input)?
          })
     }

     fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
          input: &mut I,
          c: ExitCtx,
          exit: fn(ExitCtx, DecodeError) -> !,
     ) -> Self {
          Token {
               price: BigUint::dep_decode_or_exit(input, c.clone(), exit),
               uri: Vec::<u8>::dep_decode_or_exit(input, c.clone(), exit),
               state: u8::dep_decode_or_exit(input, c.clone(), exit)
          }
     }
}


impl<BigUint: BigUintApi> TopDecode for Token<BigUint> {
     fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
          top_decode_from_nested(input)
     }

     fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
          input: I,
          c: ExitCtx,
          exit: fn(ExitCtx, DecodeError) -> !,
     ) -> Self {
          top_decode_from_nested_or_exit(input, c, exit)
     }
}

