//! FFI exported functions
use super::*;


/// Calculate the length in bytes of a kana hash output.
///
/// # Note
/// Does not consume `salt`
#[no_mangle]
pub unsafe extern "C" fn khash_length(context: *const c_void, bin: *const c_void, sz: size_t, out_len: *mut size_t) -> i32
{
    let context = context as *const ctx::CContext;
    no_unwind!{
	try error::Error::Unknown;
	let context = ctx::Context::clone_from_raw(context);
	let bin = HeapArray::<u8>::from_raw_copied(bin as *const u8, usize::from(sz));
	let string = c_try!(generate(&context, &bin));
	*out_len = string.bytes().len().into();

	GENERIC_SUCCESS
    }
}

/// Compute and write a kana hash output to a string.
///
/// # Note
/// Consumes `salt`
#[no_mangle]
pub unsafe extern "C" fn khash_do(context: *mut c_void, bin: *const c_void, sz: size_t, out_str: *mut c_char, str_len: size_t) -> i32
{
    let context = context as *mut ctx::CContext;
    no_unwind!{
	try error::Error::Unknown;
	
	let context = ctx::Context::from_raw(context);
	let bin = HeapArray::<u8>::from_raw_copied(bin as *const u8, usize::from(sz));
	let string: Vec<u8> = c_try!(generate(&context, &bin)).bytes().collect();
	
	libc::memcpy(out_str as *mut c_void, &string[0] as *const u8 as *const c_void, std::cmp::min(str_len, string.len()));
	
	GENERIC_SUCCESS
    }
}

/// Free a context
#[no_mangle]
pub unsafe extern "C" fn khash_free_context(context: *mut c_void) -> i32
{
    let context = context as *mut ctx::CContext;
    no_unwind!{
	drop(ctx::Context::from_raw(context));
	GENERIC_SUCCESS
    }
}

/// Create a new context
#[no_mangle]
pub unsafe extern "C" fn khash_new_context(algo: u8, salt_type: u8, bin: *const c_void, sz: size_t, nptr: *mut c_void) -> i32
{
    let nptr = nptr as *mut ctx::CContext;
    no_unwind!{
	try error::Error::Unknown;
	let salt = match salt_type {
	    salt::SALT_TYPE_SPECIFIC => {
		let bin = HeapArray::<u8>::from_raw_copied(bin as *const u8, usize::from(sz));
		salt::Salt::unfixed(&bin[..])
	    },
	    salt::SALT_TYPE_DEFAULT => {
		salt::Salt::default()
	    },
	    salt::SALT_TYPE_RANDOM => {
		match salt::Salt::random() {
		    Ok(v) => v,
		    Err(e) => return i32::from(error::Error::RNG(e)),
		}
	    },
	    _ => {
		salt::Salt::None
	    },
	};
	let context = ctx::Context::new(algo.into(), salt);
	*nptr = context.into_raw();
	GENERIC_SUCCESS
    }
}


/// Clone a context
#[no_mangle]
pub unsafe extern "C" fn khash_clone_context(raw: *const c_void, out: *mut c_void) -> i32
{
    let raw = raw as *const ctx::CContext;
    let out = out as *mut ctx::CContext;
    no_unwind!{
	*out = ctx::Context::clone_from_raw(raw).into_raw();
	GENERIC_SUCCESS
    }   
}

/// Free a salt allocated with `khash_new_salt`
#[no_mangle]
pub unsafe extern "C" fn khash_free_salt(salt: *mut c_void) -> i32
{
    let salt = salt as *mut salt::FFI;
    no_unwind!{
	drop(salt::from_raw(salt));
	GENERIC_SUCCESS
    }
}

/// Create a new salt
#[no_mangle]
pub unsafe extern "C" fn khash_new_salt(salt_type: u8, bin: *const c_void, sz: size_t, nptr: *mut c_void) -> i32
{
    let nptr = nptr as *mut salt::FFI;
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

/// Clone a salt
#[no_mangle]
pub unsafe extern "C" fn khash_clone_salt(salt: *const c_void, out: *mut c_void) -> i32
{
    let salt = salt as *const salt::FFI;
    let out = out as *mut salt::FFI;
    no_unwind!{
	*out = salt::into_raw(salt::clone_from_raw(salt));
	GENERIC_SUCCESS
    }   
}

/// Find the maximum length possible for a given algorithm's output.
#[no_mangle]
pub unsafe extern "C" fn khash_max_length(algo: u8, _input_sz: libc::size_t, max_len: *mut libc::size_t) -> i32
{
    no_unwind!{
	let hash_sz = match ctx::Algorithm::from(algo) {
	    ctx::Algorithm::Crc32 => std::mem::size_of::<hash::Crc32Checksum>(),
	    ctx::Algorithm::Crc64 => std::mem::size_of::<hash::Crc64Checksum>(),
	    ctx::Algorithm::Sha256 => std::mem::size_of::<hash::Sha256Hash>(),
	    ctx::Algorithm::Sha256Truncated => std::mem::size_of::<hash::Sha256Truncated>(),
	};
	*max_len =  std::mem::size_of::<char>() * hash_sz;
	GENERIC_SUCCESS
    }
}
