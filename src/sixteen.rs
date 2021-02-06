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
	let mut ar = [0u8; std::mem::size_of::<u16>()];
	if let Some(a) = self.iter.next()
	{
	    ar[0] = *a.borrow();
	} else {
	    return None;
	}
	if let Some(b) = self.iter.next()
	{
	    ar[1] = *b.borrow();
	}
	Some(u16::from_le_bytes(ar))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
	let (l, h) = self.iter.size_hint();
	(l/2, h.map(|x| x/2))
    }
}

impl<I> std::iter::FusedIterator for Bit16Iter<I>
where I: Iterator + std::iter::FusedIterator,
<I as Iterator>::Item: Borrow<u8>{}

impl<I> ExactSizeIterator for Bit16Iter<I>
where I: Iterator + ExactSizeIterator,
<I as Iterator>::Item: Borrow<u8>{}

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
