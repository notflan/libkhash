use crate::*;

pub trait ByteProvider: Sized + std::fmt::Display
{
    fn compute<T: Read + ?Sized>(input: &mut T, provided: &mut usize) -> Result<Self, error::Error>;
    fn bytes(&self) -> &[u8];
}


pub fn compute<T: Read + ?Sized, P: ByteProvider>(input: &mut T) -> Result<(usize, P), error::Error>
{
    let mut output = 0usize;
    let this = P::compute(input, &mut output)?;
    Ok((output, this))
}
