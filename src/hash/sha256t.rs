use super::*;
use crate::{
    array,
    provider::ByteProvider,
};

pub const SHA256_TRUNCATE: usize = 8;
mod __static_assert
{
    use super::*;
    const _IS_LESS_THAN_OR_EQ_SHA256_SIZE: &'static [()] = &[(); SHA256_SIZE - SHA256_TRUNCATE];
}


use std::fmt;
impl fmt::Display for Sha256Truncated
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "Sha256Truncated[->{}] (", SHA256_TRUNCATE)?;
	for byte in self.hash.iter()
	{
	    write!(f, "{:02x}", *byte)?;
	}
	write!(f, ")")
    }
}

#[repr(C)]
#[repr(packed)]
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub struct Sha256Truncated
{
    hash: [u8; SHA256_TRUNCATE],
}


impl ByteProvider for hash::Sha256Truncated
{
    fn bytes(&self) -> &[u8]
    {
	&self.hash[..]
    }
    
    fn compute<T: Read + ?Sized>(input: &mut T, salt: &salt::Salt, done: &mut usize) -> Result<Self, error::Error>
    {
	let (ok, sha) = Sha256Hash::compute(input, salt)?;
	let mut hash = [0u8; SHA256_TRUNCATE];
	array::copy_slice(&mut hash, sha.bytes());
	*done = ok;
	Ok(Self{hash})
    }
}
