use elrond_wasm::elrond_codec::*;
use elrond_wasm::Box;

const ARRAY_SIZE: usize = 512;

pub struct LargeBoxedByteArray(Box<[u8; ARRAY_SIZE]>);

impl NestedEncode for LargeBoxedByteArray {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.0.dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.0.dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for LargeBoxedByteArray {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.0.top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.0.top_encode_or_exit(output, c, exit);
	}
}

impl NestedDecode for LargeBoxedByteArray {
	#[inline]
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(LargeBoxedByteArray(Box::<[u8; ARRAY_SIZE]>::dep_decode(
			input,
		)?))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		LargeBoxedByteArray(Box::<[u8; ARRAY_SIZE]>::dep_decode_or_exit(input, c, exit))
	}
}

impl TopDecode for LargeBoxedByteArray {
	#[inline]
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(LargeBoxedByteArray(Box::<[u8; ARRAY_SIZE]>::top_decode(
			input,
		)?))
	}

	#[inline]
	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		LargeBoxedByteArray(Box::<[u8; ARRAY_SIZE]>::top_decode_or_exit(input, c, exit))
	}
}
