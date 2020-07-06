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
    use std::collections::HashMap;
    use test::{Bencher, black_box,};
    #[test]
    fn distrubution()
    {
	const THREADS: usize = 10;
	const ITERATIONS: usize = 1000;
	
	use std::{
	    sync::{
		Arc,
		Mutex,
	    },
	    thread,
	};
	let global = Arc::new(Mutex::new(HashMap::with_capacity(map::KANA.len()+map::KANA_SUB.len())));

	let _ = {
	    let mut global = global.lock().unwrap();
	    for init_c in map::KANA.iter().chain(map::KANA_SUB.iter())
	    {
		global.insert(*init_c, 0);
	    }
	    for init_c in map::KANA_SWAP.iter().chain(map::KANA_SWAP2.iter())
	    {
		if let &Some(init_c) = init_c {
		    global.insert(init_c, 0);
		}
	    }
	};

	fn do_work(num: usize, global: Arc<Mutex<HashMap<char, usize>>>, mut local: HashMap<char, usize>)
	{
	    let mut random_buffer = [0u8; 4096];
	    let context = ctx::Context::new(ctx::Algorithm::Sha256, salt::Salt::none());
	    for _ in 0..num
	    {
		getrandom::getrandom(&mut random_buffer[..]).unwrap();
		let kana = generate(&context, &random_buffer[..]).unwrap();
		for c in kana.chars()
		{
		    *local.get_mut(&c).unwrap() += 1;
		}
	    }

	    let mut global = global.lock().unwrap();
	    for (k,v) in local.into_iter()
	    {
		*global.get_mut(&k).unwrap() += v;
	    }
	}

	let joiners: Vec<thread::JoinHandle<()>> = {
	    let lock = global.lock().unwrap();

	    (0..THREADS).map(|_| {
		let global  = Arc::clone(&global);
		let local = lock.clone();
		thread::spawn(move || {
		    do_work(ITERATIONS, global, local);
		})
	    }).collect()
	};

	for x in joiners.into_iter()
	{
	    x.join().unwrap();
	}

	println!("Running {} x {} ({}) hashes (sha256)", ITERATIONS, THREADS, (ITERATIONS*THREADS));
	let global = global.lock().unwrap();
	let mut lowest = usize::MAX;
	let mut highest = 0;

	let mut lowest_char = '.';
	let mut highest_char = '.';
	const FMAX: f64 = (ITERATIONS*THREADS) as f64;

	let global = {
	    let mut out = Vec::with_capacity(global.len());
	    for (&k, &v) in global.iter()
	    {
		out.push((k, v));
	    }
	    out.sort_by(|b, a| a.1.partial_cmp(&b.1).unwrap());
	    out.into_iter()
	};
	
	for (k, v) in global
	{
	    println!("{} -> {} ({}%)", k, v, ((v as f64)/FMAX)*100.00);
	    if v < lowest {
		lowest = v;
		lowest_char = k;
	    }
	    if v > highest {
		highest = v;
		highest_char = k;
	    }
	}
	println!("Lowest was '{}' {} ({}%)", lowest_char, lowest, ((lowest as f64)/FMAX)*100.00);
	println!("Highest was '{}' {} ({}%)", highest_char, highest, ((highest as f64)/FMAX)*100.00);
	println!("Range was {}", highest-lowest);
	assert!(lowest > 0);
    }
    
    #[test]
    fn it_works() -> Result<(), error::Error>
    {
	let input = b"lolis are super ultra mega cute!";
	let context = ctx::Context::default();
	let kana = generate(&context, input)?;
	println!("kana: {}", kana);
	
	assert_eq!(kana, "もシちゅゆをヌョ");
	Ok(())
    }
    #[test]
    fn rng()
    {
	let input = b"loli";
	for _ in 0..100
	{
	    let context = ctx::Context::new(ctx::Algorithm::Sha256, salt::Salt::random().unwrap());
	    let kana = generate(&context, input).unwrap();
	    println!("kana: {}", kana);
	}
    }

    #[test]
    fn max_len()
    {
	fn max_length(algo: ctx::Algorithm, data_len: usize) -> usize
	{
	    let mut output: libc::size_t = 0;
	    unsafe {
		assert_eq!(khash_max_length(algo.into(), data_len.into(), &mut output as *mut libc::size_t), GENERIC_SUCCESS);
	    }
	    output
	}

	let input = "owowowoakpwodkapowkdapowkdpaokwpdoakwd";

	let algos = [ctx::Algorithm::Crc32, ctx::Algorithm::Crc64, ctx::Algorithm::Sha256, ctx::Algorithm::Sha256Truncated];
	for i in  0..1000
	{
	    let max_len = max_length(algos[i%algos.len()].clone(), 0);
	    print!("{} - len of {:?}: {}... ", i, algos[i%algos.len()], max_len);
	    let len = {
		let con = ctx::Context::new(algos[i%algos.len()].clone(), salt::Salt::random().unwrap());
		generate(&con, input).unwrap().len()
	    };
	    assert!(len < max_len);
	    println!("\t\tOK {}", len);
	}
    }
}

pub const BUFFER_SIZE: usize = 4096;

mod array;
mod reinterpret;
mod ext;
//use ext::*;
mod group; //unused
mod sixteen;
use sixteen::Bit16IterExt;
mod def;
mod map;
pub mod salt;
mod hash;
mod provider;
mod mnemonic;
pub mod error;
pub mod ctx;
mod stream;
pub use stream::Digest;

#[macro_use]
mod ffi;
use ffi::*;

fn compute<T: Read>(context: &ctx::Context, mut from: T) -> Result<(usize, String), error::Error>
{
    //let (read, hash) = provider::compute::<_, Digest>(&mut from, salt)?;
    let (read, hash) = context.compute(&mut from)?;

    let mut output = String::with_capacity(128);
    for element in hash.into_iter()
	.into_16()
	.map(|bytes| mnemonic::Digest::new(unsafe{reinterpret::bytes(&bytes)}))
    {
	write!(output, "{}", element)?;
    }

    Ok((read,output))
}

/// Generate kana hash from a slice.
pub fn generate<T: AsRef<[u8]>>(context: &ctx::Context, bytes: T) -> Result<String, error::Error>
{
    let bytes = bytes.as_ref();
    let mut nbytes = bytes;
    let (ok, string) = compute(context, &mut nbytes)?;
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
