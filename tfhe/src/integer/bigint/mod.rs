mod algorithms;
pub mod i256;
pub mod i512;
pub mod static_signed;
pub mod static_unsigned;
pub mod u256;
pub mod u512;

pub use static_signed::StaticSignedBigInt;
pub use static_unsigned::StaticUnsignedBigInt;

pub use i256::I256;
pub use i512::I512;
pub use u256::U256;
pub use u512::U512;

pub type U1024 = static_unsigned::StaticUnsignedBigInt<16>;
pub type U2048 = static_unsigned::StaticUnsignedBigInt<32>;
pub type U4096 = static_unsigned::StaticUnsignedBigInt<64>;

pub type I1024 = static_signed::StaticSignedBigInt<16>;
pub type I2048 = static_signed::StaticSignedBigInt<32>;
pub type I4096 = static_signed::StaticSignedBigInt<64>;

#[cfg(test)]
fn u64_with_odd_bits_set() -> u64 {
    let mut v = 0u64;

    for i in (1..=63).step_by(2) {
        v |= 1u64 << i;
    }

    v
}
#[cfg(test)]
fn u64_with_even_bits_set() -> u64 {
    let mut v = 0u64;

    // bit index are from 0 to 63
    for i in (0..=62).step_by(2) {
        v |= 1u64 << i;
    }

    v
}
