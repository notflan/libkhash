//#![feature(const_generics)]
#![allow(dead_code)]
use std::{
    io::{
	Read,
    },
    fmt::Write,
};

type HASHER = hash::Crc64Checksum;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() -> Result<(), error::Error>
    {
	let input = b"lolis are super ultra mega cute";
	let kana = generate(input, salt::Salt::default())?;
	println!("kana: {}", kana);
	assert_eq!(kana, "ワイトひはっトと");
	Ok(())
    }
    #[test]
    fn ffi() -> Result<(), Box<dyn std::error::Error>>
    {

	Ok(())
    }
}

pub const BUFFER_SIZE: usize = 4096;

mod array;
mod reinterpret;
mod ext;
use ext::*;
mod group; //unused
mod sixteen;
use sixteen::Bit16IterExt;
mod def;
mod map;
mod salt;
mod hash;
mod provider;
mod mnemonic;
mod error;

#[macro_use]
mod ffi;
use ffi::*;

fn compute<T: Read, Digest: provider::ByteProvider>(mut from: T, salt: salt::Salt) -> Result<(usize, String), error::Error>
{
    let (read, hash) = provider::compute::<_, Digest>(&mut from, salt)?;

    let mut output = String::with_capacity(128);
    for element in hash.bytes().iter()
	.into_16()
	.map(|bytes| mnemonic::Digest::new(unsafe{reinterpret::bytes(&bytes)}))
    {
	write!(output, "{}", element)?;
    }

    Ok((read,output))
}

pub fn generate<T: AsRef<[u8]>>(bytes: T, salt: salt::Salt) -> Result<String, error::Error>
{
    let bytes = bytes.as_ref();
    let mut nbytes = bytes;
    let (ok, string) = compute::<_, HASHER>(&mut nbytes,salt)?;
    if ok == bytes.len() {
	Ok(string)
    } else {
	return Err(error::Error::Length{expected: bytes.len(), got: ok});
    }
}

use std::ffi::c_void;
use libc::{
    size_t,
    c_char,
};

use malloc_array::{
    HeapArray,
};

// FFI section

/// Calculate the length in bytes of a kana hash output.
///
/// # Note
/// Does not consume `salt`
#[no_mangle]
pub unsafe extern "C" fn khash_length(bin: *const c_void, sz: size_t, salt: *const salt::FFI, out_len: *mut size_t) -> i32
{
    no_unwind!{
	try error::Error::Unknown;
	let bin = HeapArray::<u8>::from_raw_copied(bin as *const u8, usize::from(sz));
	let string = c_try!(generate(&bin, salt::clone_from_raw(salt)));
	*out_len = string.bytes().len().into();

	GENERIC_SUCCESS
    }
}

/// Compute and write a kana hash output to a string.
///
/// # Note
/// Consumes `salt`
#[no_mangle]
pub unsafe extern "C" fn khash_do(bin: *const c_void, sz: size_t, salt: *mut salt::FFI, out_str: *mut c_char, str_len: size_t) -> i32
{
    no_unwind!{
	try error::Error::Unknown;
	let bin = HeapArray::<u8>::from_raw_copied(bin as *const u8, usize::from(sz));
	let string: Vec<u8> = c_try!(generate(&bin, salt::from_raw(salt))).bytes().collect();
	
	libc::memcpy(out_str as *mut c_void, &string[0] as *const u8 as *const c_void, std::cmp::min(str_len, string.len()));
	
	GENERIC_SUCCESS
    }
}

/// Free a salt allocated with `khash_new_salt`
#[no_mangle]
pub unsafe extern "C" fn khash_free_salt(salt: *mut salt::FFI) -> i32
{
    no_unwind!{
	drop(salt::from_raw(salt));
	GENERIC_SUCCESS
    }
}

/// Create a new salt
#[no_mangle]
pub unsafe extern "C" fn khash_new_salt(salt_type: u8, bin: *const c_void, sz: size_t, nptr: *mut salt::FFI) -> i32
{
    no_unwind!{
	try error::Error::Unknown;
	match salt_type {
	    salt::SALT_TYPE_SPECIFIC => {
		let bin = HeapArray::<u8>::from_raw_copied(bin as *const u8, usize::from(sz));
		*nptr = salt::into_raw(salt::Salt::unfixed(&bin[..]));
	    },
	    salt::SALT_TYPE_DEFAULT => {
		*nptr = salt::into_raw(salt::Salt::default());
	    },
	    salt::SALT_TYPE_RANDOM => {
		*nptr = salt::into_raw(match salt::Salt::random() {
		    Ok(v) => v,
		    Err(e) => return i32::from(error::Error::RNG(e)),
		})
	    },
	    _ => {
		*nptr = salt::into_raw(salt::Salt::None);
	    },
	}
	GENERIC_SUCCESS
    }
}

#[no_mangle]
pub unsafe extern "C" fn khash_clone_salt(salt: *const salt::FFI, out: *mut salt::FFI) -> i32
{
    no_unwind!{
	*out = salt::into_raw(salt::clone_from_raw(salt));
	GENERIC_SUCCESS
    }   
}


//TODO:
/*
mod ctx;

#[no_mangle]
pub unsafe extern "C" fn khash_new_context(salt: *mut salt::FFI, ctx: *mut ctx::CContext) -> i32
{

}
*/
