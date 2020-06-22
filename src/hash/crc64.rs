use super::*;
use crc::{Hasher64, crc64};

pub struct Crc64Checksum
{
    hash: u64,
}

impl provider::ByteProvider for Crc64Checksum
{
    fn bytes(&self) -> &[u8]
    {
	unsafe{reinterpret::bytes(&self.hash)}
    }
    
    fn compute<T: Read + ?Sized>(input: &mut T, salt: &salt::Salt, done: &mut usize) -> Result<Self, error::Error>
    {
	let mut buffer = [0u8; BUFFER_SIZE];
	let mut hasher = crc64::Digest::new(crc64::ECMA);
	let mut read;
	while (read = input.read(&mut buffer[..])?, read!=0).1
	{
	    hasher.write(&buffer[..read]);
	    *done += read;
	}
	hasher.write(salt.bytes());
	Ok(Self{hash: hasher.sum64()})
    }
}

use std::fmt;
impl fmt::Display for Crc64Checksum
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "Crc64checksum (")?;
	for b in provider::ByteProvider::bytes(self) {
	    write!(f, "{:02x}", *b)?;
	}
	write!(f, ")")
    }
}
