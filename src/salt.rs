#[cfg(feature="ffi")] use malloc_array::*;

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
    convert::{TryInto,TryFrom},
};

/// The static salt size
pub const SIZE: usize = 32;

/// The default static salt
const STATIC_SALT: &[u8; SIZE] = &hex!("6787f049791466d5a31a3aa6f7138d8fbb907fd1785758298b5c97b0f3fb31ff");

/// A salt to use for the kana-hash algorithm
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
    /// A fixed size salt of [SIZE]
    pub fn fixed(array: [u8; SIZE]) -> Self
    {
	Self::Fixed(array)
    }
    /// A salt from a slice
    pub fn unfixed<T>(slice: &T) -> Self
    where T: AsRef<[u8]> + ?Sized
    {
	let slice = slice.as_ref();
	assert!(slice.len() > 0, "Salt expects at least one byte.");
	Self::Dynamic(Vec::from(slice).into_boxed_slice())
    }
    /// No salt
    pub const fn none() -> Self
    {
	Self::None
    }
    /// Try to create a random salt
    pub fn random() -> Result<Self, Error>
    {
	let mut buffer = [0u8; SIZE];
	getrandom(&mut buffer[..])?;
	Ok(Self::Fixed(buffer))
    }
    /// The default internal salt
    pub const fn internal() -> Self
    {
	Self::Static(STATIC_SALT)
    }

    /// Get the raw bytes of this salt
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
pub(crate) struct FFI
{
    salt_type: u8,
    size: u32,
    body: *mut u8,
}

pub(crate) const SALT_TYPE_NONE: u8 = 0;
pub(crate) const SALT_TYPE_DEFAULT: u8 = 1;
pub(crate) const SALT_TYPE_SPECIFIC: u8 = 2;
pub(crate) const SALT_TYPE_RANDOM: u8 = 3;

/// We won't try to copy more than this much data.
const MAX_FFI_SALT_SIZE: usize = 1024;

/// Clone a new `Salt` from an `FFI` salt.
#[cfg(feature="ffi")] pub(crate) unsafe fn clone_from_raw(ptr: *const FFI) -> Salt
{
    let ffi = &*ptr;
    match ffi.salt_type {
	SALT_TYPE_SPECIFIC => {
	    Salt::Dynamic(HeapArray::from_raw_copied(ffi.body as *const u8, usize::try_from(ffi.size).unwrap()).into_boxed_slice())
	},
	SALT_TYPE_DEFAULT => {
	    Salt::default()
	},
	_ => Salt::None,
    }
}
/// Consume an `FFI` salt and return a `Salt`.
#[cfg(feature="ffi")]  pub(crate) unsafe fn from_raw(ptr: *mut FFI) -> Salt
{
    let ffi = &mut *ptr;
    let out = match ffi.salt_type {
	SALT_TYPE_SPECIFIC => {
	    Salt::Dynamic(HeapArray::from_raw_parts(ffi.body as *mut u8, usize::try_from(ffi.size).unwrap()).into_boxed_slice())
	},
	SALT_TYPE_DEFAULT => {
	    Salt::default()
	},
	_ => Salt::None,
    };
    ffi.salt_type = SALT_TYPE_NONE;
    ffi.size = 0;
    ffi.body = 0 as *mut u8;
    out
}

/// Consume a `Salt` and output a new `FFI` salt.
#[cfg(feature="ffi")] pub(crate) unsafe fn into_raw(salt: Salt) -> FFI
{
    unsafe fn allocate(slice: impl AsRef<[u8]>) -> FFI
    {
	let (body, size) = box_with_malloc(slice);
	FFI {
	    salt_type: SALT_TYPE_SPECIFIC,
	    size: size.try_into().unwrap(),
	    body
	}
    }
    
    match &salt {
	Salt::None => FFI {
	    salt_type: SALT_TYPE_NONE,
	    size: 0,
	    body: 0 as *mut u8,
	},
	Salt::Static(STATIC_SALT) => FFI {
	    salt_type: SALT_TYPE_DEFAULT,
	    size: 0,
	    body: 0 as *mut u8,
	},
	Salt::Dynamic(bytes) => allocate(&bytes),
	Salt::Fixed(bytes) | &Salt::Static(bytes) => allocate(&bytes),
    }
}

#[cfg(feature="ffi")] fn box_with_malloc(slice: impl AsRef<[u8]>) -> (*mut u8, usize)
{
    unsafe { HeapArray::from_slice_copied(slice) }.into_raw_parts()
}
