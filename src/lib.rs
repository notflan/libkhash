#![allow(dead_code)]
use std::{
    io::{
	Read,
    },
    fmt::Write,
};

type HASHER =hash::Crc64Checksum;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() -> Result<(), error::Error>
    {
	let input = b"hello world!!";
	let kana = generate(input)?;
	println!("kana: {}", kana);
	panic!("uhh")
    }
}

pub const BUFFER_SIZE: usize = 4096;

mod array;
mod reinterpret;
mod group;
mod sixteen;
use sixteen::Bit16IterExt;
mod def;
mod map;
mod hash;
mod provider;
mod mnemonic;
mod error;

#[macro_use]
mod ffi;
use ffi::*;

fn compute<T: Read, Digest: provider::ByteProvider>(mut from: T) -> Result<(usize, String), error::Error>
{
    let (read, hash) = provider::compute::<_, Digest>(&mut from)?;

    println!("hash ({}): {}", read, hash);
    let mut output = String::with_capacity(128);
    for element in hash.bytes().iter()
	.into_16()
	.map(|bytes| mnemonic::Digest::new(unsafe{reinterpret::bytes(&bytes)}))
    {
	write!(output, "{}", element)?;
    }

    Ok((read,output))
}

pub fn generate<T: AsRef<[u8]>>(bytes: T) -> Result<String, error::Error>
{
    let bytes = bytes.as_ref();
    let mut nbytes = bytes;
    let (ok, string) = compute::<_, HASHER>(&mut nbytes)?;
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

#[no_mangle]
pub unsafe extern "C" fn _kana_length(bin: *const c_void, sz: size_t, out_len: *mut size_t) -> i32
{
    no_unwind!{
	try error::Error::Unknown;
	let bin = HeapArray::<u8>::from_raw_copied(bin as *const u8, usize::from(sz));
	let string = c_try!(generate(&bin));
	*out_len = (string.bytes().len()+1).into();

	GENERIC_SUCCESS
    }
}
#[no_mangle]
pub unsafe extern "C" fn _kana_do(bin: *const c_void, sz: size_t, out_str: *mut c_char, str_len: size_t) -> i32
{
    no_unwind!{
	try error::Error::Unknown;
	let bin = HeapArray::<u8>::from_raw_copied(bin as *const u8, usize::from(sz));
	let string: Vec<u8> = c_try!(generate(&bin)).bytes().collect();
	
	libc::memcpy(out_str as *mut c_void, &string[0] as *const u8 as *const c_void, std::cmp::min(str_len, string.len()));
	
	GENERIC_SUCCESS
    }
}
