use crate::*;

pub const ALGO_CRC32: u8 = 0;
pub const ALGO_CRC64: u8 = 1;
pub const ALGO_SHA256: u8 = 2;

#[derive(Debug,PartialEq,Eq,Hash)]
pub enum Algorithm
{
    Crc32,
    Crc64,
    Sha256,
}

pub struct Context
{
    algo: Algorithm,
    salt: salt::Salt,
    
}

/// FFI context
#[derive(Debug)]
#[repr(C)]
pub struct CContext
{
    algo: u8,
    salt: *mut salt::FFI,
    
}
