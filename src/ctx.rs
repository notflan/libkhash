//! Contains contexts used for algorithms
use crate::*;
use std::{
    io::{
	Read,
    },
};

/// An algorithm to use for the context.
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Algorithm
{
    Crc32,
    Crc64,
    Sha256,
    Sha256Truncated,
}

impl Default for Algorithm
{
    #[inline] fn default() -> Self
    {
	Self::Sha256Truncated
    }
}

/// A kana-hash context containing it's salt and algorithm.
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub struct Context
{
    algo: Algorithm,
    salt: salt::Salt,
}

impl Context
{
    /// Create a new kana-hash context with an algorithm and a salt
    pub fn new(algo: Algorithm, salt: impl Into<salt::Salt>) -> Self
    {
	Self {
	    algo,
	    salt: salt.into(),
	}
    }

    /// The algorithm used
    pub fn get_algorithm(&self) -> &Algorithm
    {
	&self.algo
    }
    /// The salt used
    pub fn get_salt(&self) -> &salt::Salt
    {
	&self.salt
    }
    
    pub(crate) fn compute<R: Read>(&self, mut from: R) -> Result<(usize, Box<[u8]>), error::Error>
    {
	fn provide<P,R>(input: &mut R, salt: &salt::Salt, output: &mut usize) -> Result<Vec<u8>, error::Error>
	where P: provider::ByteProvider,
	      R: Read + ?Sized
	{
	    let this = P::compute(input, &salt, output)?;
	    Ok(Vec::from(this.bytes()))
	}

	let mut output = 0usize;
	let bytes = match self.algo
	{
	    Algorithm::Crc32 => provide::<hash::Crc32Checksum, _>(&mut from, &self.salt, &mut output)?,
	    Algorithm::Crc64 => provide::<hash::Crc64Checksum, _>(&mut from, &self.salt, &mut output)?,
	    Algorithm::Sha256 => provide::<hash::Sha256Hash, _>(&mut from, &self.salt, &mut output)?,
	    Algorithm::Sha256Truncated => provide::<hash::Sha256Truncated, _>(&mut from, &self.salt, &mut output)?,
	}.into_boxed_slice();

	Ok((output, bytes))
    }

    pub(crate) unsafe fn into_raw(self) -> CContext
    {
	CContext{ 
	    algo: u8::from(self.algo),
	    salt: salt::into_raw(self.salt),
	    flags: Default::default(),
	}
    }
    
    pub(crate) unsafe fn clone_from_raw(from: *const CContext) -> Self
    {
	let from = &*from;
	Self {
	    algo: from.algo.into(),
	    salt: salt::clone_from_raw(&from.salt as *const salt::FFI),
	}
    }

    pub(crate) unsafe fn from_raw(from: *mut CContext) -> Self
    {
	let from = &mut *from;
	let output = Self{
	    algo: from.algo.into(),
	    salt: salt::from_raw(&mut from.salt as *mut salt::FFI),
	};
	from.algo = 0;
	output
    }
}

impl Default for Context
{
    fn default() -> Self
    {
	Self {
	    algo: Default::default(),
	    salt: Default::default(),
	}
    }
}

pub(crate) const ALGO_DEFAULT: u8 = 0;
pub(crate) const ALGO_CRC32: u8 = 1;
pub(crate) const ALGO_CRC64: u8 = 2;
pub(crate) const ALGO_SHA256: u8 = 3;
pub(crate) const ALGO_SHA256_TRUNCATED: u8 = 4;

/// FFI context
#[derive(Debug)]
#[repr(C)]
pub(crate) struct CContext
{
    algo: u8,
    flags: u64, //nothing yet, might be flags later idk
    salt: salt::FFI,
    
}

impl From<Algorithm> for u8
{
    fn from(al: Algorithm) -> Self
    {
	match al {
	    Algorithm::Crc32 => ALGO_CRC32,
	    Algorithm::Crc64 => ALGO_CRC64,
	    Algorithm::Sha256 => ALGO_SHA256,
	    Algorithm::Sha256Truncated => ALGO_SHA256_TRUNCATED,
	}
    }
}
impl From<u8> for Algorithm
{
    fn from(al: u8) -> Self
    {
	match al {
	    ALGO_CRC32 => Algorithm::Crc32,
	    ALGO_CRC64 => Algorithm::Crc64,
	    ALGO_SHA256 => Algorithm::Sha256,
	    ALGO_SHA256_TRUNCATED => Algorithm::Sha256Truncated,
	    _ => Self::default(),
	}
    }
}
