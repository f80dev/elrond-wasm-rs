use elrond_wasm::elrond_codec::*;

imports!();

pub struct LotteryInfo<BigUint:BigUintApi> {
    pub ticket_price: BigUint, 
    pub tickets_left: u32, 
    pub deadline: u64,
    pub max_entries_per_user: u32,
    pub prize_distribution: Vec<u8>,
    pub whitelist: Vec<Address>, 
    pub current_ticket_number: u32,
    pub prize_pool: BigUint,
    pub queued_tickets: u32
}

impl<BigUint:BigUintApi> NestedEncode for LotteryInfo<BigUint> {
    fn dep_encode_to<O: OutputBuffer>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.ticket_price.dep_encode_to(dest)?;
        self.tickets_left.dep_encode_to(dest)?;
        self.deadline.dep_encode_to(dest)?;
        self.max_entries_per_user.dep_encode_to(dest)?;
        self.prize_distribution.dep_encode_to(dest)?;
        self.whitelist.dep_encode_to(dest)?;
        self.current_ticket_number.dep_encode_to(dest)?;
        self.prize_pool.dep_encode_to(dest)?;
        self.queued_tickets.dep_encode_to(dest)?;

        core::result::Result::Ok(())
    }
}

impl<BigUint:BigUintApi> TopEncode for LotteryInfo<BigUint> {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_slice_u8(dep_encode_to_vec(self)?.as_slice());
        Result::Ok(())
    }
}


impl<BigUint:BigUintApi> NestedDecode for LotteryInfo<BigUint> {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        core::result::Result::Ok(LotteryInfo {
            ticket_price: BigUint::dep_decode(input)?,
            tickets_left: u32::dep_decode(input)?,
            deadline: u64::dep_decode(input)?,
            max_entries_per_user: u32::dep_decode(input)?,
            prize_distribution: Vec::<u8>::dep_decode(input)?,
            whitelist: Vec::<Address>::dep_decode(input)?,
            current_ticket_number: u32::dep_decode(input)?,
            prize_pool: BigUint::dep_decode(input)?,
            queued_tickets: u32::dep_decode(input)?,
        })
    }
}

impl<BigUint:BigUintApi> TopDecode for LotteryInfo<BigUint> {
    fn top_decode<I: TopDecodeInput>(mut input: I) -> Result<Self, DecodeError> {
        dep_decode_from_byte_slice(input.get_slice_u8())
    }
}