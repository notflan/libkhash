use super::*;
use crc::{Hasher32, crc32};

pub struct Crc32Checksum
{
    hash: u32,
}

impl provider::ByteProvider for Crc32Checksum
{
    fn bytes(&self) -> &[u8]
    {
	unsafe{reinterpret::bytes(&self.hash)}
    }
    
    fn compute<T: Read + ?Sized>(input: &mut T, salt: &salt::Salt, done: &mut usize) -> Result<Self, error::Error>
    {
	let mut buffer = [0u8; BUFFER_SIZE];
	let mut hasher = crc32::Digest::new(crc32::IEEE);
	let mut read;
	while (read = input.read(&mut buffer[..])?, read!=0).1
	{
	    hasher.write(&buffer[..read]);
	    *done += read;
	}
	hasher.write(salt.bytes());
	Ok(Self{hash: hasher.sum32()})
    }
}

use std::fmt;
impl fmt::Display for Crc32Checksum
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "Crc32checksum (")?;
	for b in provider::ByteProvider::bytes(self) {
	    write!(f, "{:02x}", *b)?;
	}
	write!(f, ")")
    }
}
