use std::borrow::Borrow;

/// Iter that converts 2 `u8`s into 1 `u16`
pub struct Bit16Iter<I>
where I: Iterator,
<I as Iterator>::Item: Borrow<u8>
{
    iter: I,
}

impl<I> Iterator for Bit16Iter<I>
where I: Iterator,
<I as Iterator>::Item: Borrow<u8>
{
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item>
    {
	let mut c = 0u16;
	unsafe {
	    if let Some(a) = self.iter.next() {
		crate::reinterpret::bytes_mut(&mut c)[0] = *a.borrow();
	    } else {
		return None;
	    }
	    if let Some(b) = self.iter.next() {
		crate::reinterpret::bytes_mut(&mut c)[1] = *b.borrow();
	    }
	}
	Some(c)
    }
}

pub trait Bit16IterExt: Iterator + Sized
where <Self as Iterator>::Item: Borrow<u8>
{
    fn into_16(self) -> Bit16Iter<Self>;
}

impl<I> Bit16IterExt for I
where I: Iterator,
<I as Iterator>::Item: Borrow<u8>
{
    fn into_16(self) -> Bit16Iter<Self>
    {
	Bit16Iter{
	    iter: self
	}
    }
}
