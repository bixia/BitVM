pub mod add;
pub mod bits;
pub mod cmp;
pub mod inv;
pub mod mul;
pub mod std;
pub mod sub;

pub struct BigIntImpl<const N_BITS: u32> {}

pub const MAX_U30: u32 = 1 << 30;

impl<const N_BITS: u32> BigIntImpl<N_BITS> {
    pub const N_BITS: u32 = N_BITS;
    pub const N_LIMBS: u32 = (N_BITS + 30 - 1) / 30;
    pub const HEAD: u32 = N_BITS - (Self::N_LIMBS - 1) * 30;
    pub const HEAD_OFFSET: u32 = 1u32 << Self::HEAD;
}

pub type U254 = BigIntImpl<254>;
