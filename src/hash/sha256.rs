use super::*;
use sha2::{Sha256, Digest};
use std::{
    io::{
	self,
	Read,
    },
};

pub const SHA256_SIZE: usize = 32;
#[repr(C)]
#[repr(packed)]
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub struct Sha256Hash
{
    hash: [u8; SHA256_SIZE],
}


fn compute_stream<T: Read +?Sized, D: Digest>(input: &mut T, output: &mut D) -> io::Result<usize>
{
    let mut buffer = [0u8; BUFFER_SIZE];

    let mut read;
    let mut done=0;
    while (read = input.read(&mut buffer[..])?, read!=0).1
    {
	output.update(&buffer[..read]);
	done+=read;
    }
    Ok(done)
}

impl Sha256Hash
{
    /// Compute a hash from a stream.
    pub fn compute<T: Read + ?Sized>(input: &mut T, salt: &salt::Salt) -> io::Result<(usize, Self)>
    {
	let mut hash = [0u8; SHA256_SIZE];

	let mut hasher = Sha256::new();

	let ok = compute_stream(input, &mut hasher)?;
	hasher.update(salt.bytes());

	assert_eq!(array::copy_slice(&mut hash, hasher.finalize()), SHA256_SIZE);
	Ok((ok, Self{hash}))
    }

    pub fn bytes(&self) -> &[u8; SHA256_SIZE]
    {
	&self.hash
    }
}

use std::fmt;
impl fmt::Display for Sha256Hash
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "Sha256hash (")?;
	for byte in self.hash.iter()
	{
	    write!(f, "{:02x}", *byte)?;
	}
	write!(f, ")")
    }
}


impl provider::ByteProvider for hash::Sha256Hash
{
    fn bytes(&self) -> &[u8]
    {
	&self.bytes()[..]
    }
    
    fn compute<T: Read + ?Sized>(input: &mut T, salt: &salt::Salt, done: &mut usize) -> Result<Self, error::Error>
    {
	let (ok, this) = Self::compute(input, salt)?;
	*done = ok;
	Ok(this)
    }
}
