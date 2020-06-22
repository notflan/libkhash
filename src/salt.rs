use crate::*;
use malloc_array::*;
use getrandom::{
    getrandom,
    Error,
};
use hex_literal::hex;

use std::{
    io::{
	self,
	Write,
    },
};

pub const SIZE: usize = 32;

const STATIC_SALT: &[u8; SIZE] = &hex!("6787f049791466d5a31a3aa6f7138d8fbb907fd1785758298b5c97b0f3fb31ff");

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub enum Salt
{
    None,
    Static(&'static [u8; SIZE]),
    Fixed([u8; SIZE]),
    Dynamic(Box<[u8]>),
}

impl Default for Salt
{
    fn default() -> Self
    {
	Self::Static(STATIC_SALT)
    }
}

impl Salt
{
    pub fn fixed(array: [u8; SIZE]) -> Self
    {
	Self::Fixed(array)
    }
    pub fn unfixed<T>(slice: &T) -> Self
    where T: AsRef<[u8]> + ?Sized
    {
	let slice = slice.as_ref();
	assert!(slice.len() > 0, "Salt expects at least one byte.");
	Self::Dynamic(Vec::from(slice).into_boxed_slice())
    }
    pub fn none() -> Self
    {
	Self::None
    }
    pub fn random() -> Result<Self, Error>
    {
	let mut buffer = [0u8; SIZE];
	getrandom(&mut buffer[..])?;
	Ok(Self::Fixed(buffer))
    }
    pub const fn internal() -> Self
    {
	Self::Static(STATIC_SALT)
    }

    pub fn bytes(&self) -> &[u8]
    {
	match &self {
	    Self::Fixed(ar) => &ar[..],
	    Self::Dynamic(vec) => &vec[..],
	    Self::Static(s) => &s[..],
	    _ => &[],
	}
    }

    /// Append salt bytes to a stream.
    pub fn append<W: Write+?Sized>(&self, to: &mut W) -> io::Result<usize>
    {
	to.write(self.bytes())
    }
}

#[derive(Copy,Clone,Debug)]
#[repr(C)]
#[repr(packed)]
pub struct FFI
{
    size: usize,
    body: *mut u8,
}

/// We won't try to copy more than this much data.
const MAX_FFI_SALT_SIZE: usize = 1024;
/// Clone a new `Salt` from an `FFI` salt.
pub unsafe fn clone_from_raw(ptr: *const FFI) -> Salt
{
    if ptr.is_null() {
	Salt::default()
    } else {
	let ptr = &*ptr;
	if ptr.size == 0 || ptr.body.is_null() {
	    return Salt::None;
	}
	let size = std::cmp::min(ptr.size, MAX_FFI_SALT_SIZE);
	Salt::Dynamic(HeapArray::from_raw_copied(ptr.body, size).into_unsafe().into_boxed_slice())
    }
}
/// Consume an `FFI` salt and return a `Salt`.
pub unsafe fn from_raw(ptr: *mut FFI) -> Salt
{
    if ptr.is_null() {
	Salt::default()
    } else {
	let ptr = {
	    let mut ptr = HeapArray::from_raw_parts(ptr, 1);
	    let rval = ptr[0].clone();
	    ptr.set_memory(0);
	    rval
	};
	if ptr.size == 0 || ptr.body.is_null() {
	    return Salt::None;
	}
	let body = HeapArray::from_raw_parts(ptr.body, ptr.size);
	Salt::Dynamic(body.into_boxed_slice())
    }
}

/// Consume a `Salt` and output a newly allocated `FFI` salt.
pub unsafe fn into_raw(salt: Salt) -> *mut FFI
{
    unsafe fn genffi(bytes: &[u8]) -> *mut FFI
    {
	if bytes.len() == 0 {
	    let (ffi, _) = heap![FFI{size:0,body:0 as *mut u8}].into_raw_parts();
	    ffi
	} else {
	    let mut array = heap![unsafe u8; bytes.len()];
	    array.memory_from_raw(&bytes[0] as *const u8, bytes.len());
	    let (body, size) = array.into_raw_parts();
	    let (ffi, _) = heap![FFI{size,body}].into_raw_parts();
	    ffi
	}
    }
    match salt {
	Salt::Static(STATIC_SALT) => 0 as *mut FFI,
	Salt::Static(&other) | Salt::Fixed(other) => genffi(&other[..]),
	Salt::Dynamic(other) => genffi(&other[..]),
	_ => genffi(salt.bytes()),
    }
}

