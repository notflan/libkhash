//! Contains contexts used for the digest algorithms.
//!
//! These contexts contain the digest name and the salt to be used with the digest.
//!
//! # Defaults
//! Both the digest name (`Algorithm`) and the `Context` itself implement `Default`.
//! The default algorithm is `SHA256Truncated`, and the default `Context` uses this algorithm along with the default salt (which is the library's hard-coded static salt.)

use crate::*;
use std::{
    io::{
	Read,
    },
};

/// An algorithm to use for the context.
///
/// # CRC
/// `CRC32` and `CRC64` are only available if compiled with the default "crc" feature enabled.
/// If the library is compiled without this feature, but with the "ffi" feature (i.e. generates native libraries), then FFI requests for the CRC family of digests will instead use the default (`Sha256Truncated`).
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Algorithm
{
    #[cfg(feature="crc")]
    /// The 32 bit CRC checksum (requires default feature `crc`)
    Crc32,
    #[cfg(feature="crc")]
    /// The 64 bit CRC checksum (requires default feature `crc`)
    Crc64,
    /// The SHA256 hash
    Sha256,
    /// The SHA256 hash truncated to the first 64 bits
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
///
/// # Default
/// The default context contains the `SHA256Truncated` digest algorithm and the library's hard-coded static salt.
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
	    #[cfg(feature="crc")] Algorithm::Crc32 => provide::<hash::Crc32Checksum, _>(&mut from, &self.salt, &mut output)?,
	    #[cfg(feature="crc")] Algorithm::Crc64 => provide::<hash::Crc64Checksum, _>(&mut from, &self.salt, &mut output)?,
	    Algorithm::Sha256 => provide::<hash::Sha256Hash, _>(&mut from, &self.salt, &mut output)?,
	    Algorithm::Sha256Truncated => provide::<hash::Sha256Truncated, _>(&mut from, &self.salt, &mut output)?,
	}.into_boxed_slice();

	Ok((output, bytes))
    }

    #[cfg(feature="ffi")] 
    pub(crate) unsafe fn into_raw(self) -> CContext
    {
	CContext{ 
	    algo: u8::from(self.algo),
	    salt: salt::into_raw(self.salt),
	    flags: Default::default(),
	}
    }
    
    #[cfg(feature="ffi")] 
    pub(crate) unsafe fn clone_from_raw(from: *const CContext) -> Self
    {
	let from = &*from;
	Self {
	    algo: from.algo.into(),
	    salt: salt::clone_from_raw(&from.salt as *const salt::FFI),
	}
    }
    
    #[cfg(feature="ffi")] 
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
	    #[cfg(feature="crc")] Algorithm::Crc32 => ALGO_CRC32,
	    #[cfg(feature="crc")] Algorithm::Crc64 => ALGO_CRC64,
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
	    #[cfg(feature="crc")] ALGO_CRC32 => Algorithm::Crc32,
	    #[cfg(feature="crc")] ALGO_CRC64 => Algorithm::Crc64,
	    ALGO_SHA256 => Algorithm::Sha256,
	    ALGO_SHA256_TRUNCATED => Algorithm::Sha256Truncated,
	    _ => Self::default(),
	}
    }
}
