//! # khash - Kana mnemonic hashes
//!
//! This library pretty prints salted hashes of a veriaty of digests in kana.
//! Mnemonics can be generated from slices or from streams.
//!
//! It has a Rust API documented here, as well as C FFI bindings and a C header (see `include/`.)
//!
//! ## Digest
//! The digests available are:
//! * SHA256 truncated to the first 64 bits (8 bytes) (default digest)
//! * SHA256 full
//! * CRC64 (requires "crc" default feature enabled)
//! * CRC32 (requires "crc" default feature enabled)
//!
//! ### Salting
//! The salting options for the digests are:
//! * Hard-coded embedded 32 byte salt (default)
//! * Fixed compile time 32 byte salt
//! * Fixed runtime 32 byte salt
//! * Dynamically sized runtime salt
//! * No salt
//! The salt (if any) is fed into the digest directly after all the data.
//! (See `ctx` and `salt` modules).
//!
//! ## Generating kana mnemonics from arbitrary data
//! To use the mnemonic generation algorithm on any binary data instead of just hash outputs, the `Digest` iterator type is provided.
//! The `Digest` iterator can be created from any type implementing `std::io::Read` and produces a kana mnemonic reading from the stream until its end.
//! ```
//! # use khash::Digest;
//! let input = "Hello world!";
//! let mnemonic: String = Digest::new(&mut input.as_bytes()).collect(); // Read the bytes from the `input` string and collect the kana mnemonic into a `String` 
//! ```

#![cfg_attr(nightly, feature(test))] 
#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(nightly)] extern crate test;

use std::{
    io::{
	Read,
    },
    fmt::Write,
};

//type HASHER = hash::Crc64Checksum; //was unused?

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    #[cfg(nightly)] 
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
    #[cfg(feature="ffi")]
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

	let algos = [#[cfg(feature="crc")] ctx::Algorithm::Crc32,
		     #[cfg(feature="crc")] ctx::Algorithm::Crc64,
		     ctx::Algorithm::Sha256,
		     ctx::Algorithm::Sha256Truncated];
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

/// The size used for internal buffers
const BUFFER_SIZE: usize = 4096;

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
	.map(|bytes| mnemonic::Digest::new(&u16::to_le_bytes(bytes)[..]))//unsafe{reinterpret::bytes(&bytes)}))
    {
	write!(output, "{}", element)?;
    }

    Ok((read,output))
}

/// Generate kana hash from a slice of bytes with this digest.
///
/// # Example
/// To generate a hash with the default digest from a string
/// ```
/// # use khash::generate;
/// generate(&Default::default(), "Hello world!").expect("Failed to generate hash string");
/// ```
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

/// Generate kana hash from a stream of bytes with this digest.
/// # Example
/// To generate a hash from a file with the default digest
/// ```
/// # use khash::generate_stream;
/// # use std::{path::Path, fs::OpenOptions};
/// fn hash_file(file_name: impl AsRef<Path>) -> String
/// {
///     let mut file = OpenOptions::new()
/// 	.read(true)
/// 	.open(file_name).expect("Failed to open file");
/// 
///     let file_size = file.metadata().expect("Failed to stat file").len();
/// 
///     let (bytes_read, hash) = generate_stream(&Default::default(), &mut file).expect("Failed to generate hash from file");
///     assert_eq!(bytes_read as u64, file_size, "Failed to read whole file");
/// 
///     hash
/// }
/// ```
#[inline] pub fn generate_stream<T: Read+?Sized>(context: &ctx::Context, from: &mut T) -> Result<(usize, String), error::Error>
{
    compute(context, from)
}

use std::ffi::c_void;
#[cfg(feature="ffi")] 
use libc::{
    size_t,
    c_char,
};

#[cfg(feature="ffi")] 
use malloc_array::{
    HeapArray,
};



// FFI section
#[cfg(feature="ffi")] 
mod c;
#[cfg(feature="ffi")] 
pub use c::*;
