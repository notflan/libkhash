use std::ops::RangeInclusive;

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Definition
{
    Single(RangeInclusive<usize>),
    Any,
    None,
}

impl Definition
{
    
    pub const fn single(idx: RangeInclusive<usize>) -> Self
    {
	Self::Single(idx)
    }
    pub const fn any() -> Self
    {
	Self::Any
    }
    pub const fn none() -> Self
    {
	Self::None
    }

    pub fn contains(&self, sz: usize) -> bool
    {
	use Definition::*;
	match self {
	    Single(range) => range.contains(&sz),
	    Any => true,
	    _ => false,
	}
    }
}
