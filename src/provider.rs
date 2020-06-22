use crate::*;

pub trait ByteProvider: Sized + std::fmt::Display
{
    fn compute<T: Read + ?Sized>(input: &mut T, salt: &salt::Salt, provided: &mut usize) -> Result<Self, error::Error>;
    fn bytes(&self) -> &[u8];
}


pub fn compute<T: Read + ?Sized, P: ByteProvider>(input: &mut T, salt: salt::Salt) -> Result<(usize, P), error::Error>
{
    let mut output = 0usize;
    let this = P::compute(input, &salt, &mut output)?;
    Ok((output, this))
}
