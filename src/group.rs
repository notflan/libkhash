/// Group iterator output every n values into `Vec<T>`.
pub struct GroupIter<T,I>
where I: Iterator<Item=T>
{
    buffer: Vec<T>,
    iter: I,
    group_at: usize,
}

impl<T,I> Iterator for GroupIter<T,I>
where I: Iterator<Item=T>
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item>
    {
	while self.buffer.len() < self.group_at
	{
	    if let Some(value) = self.iter.next() {
		self.buffer.push(value)
	    } else {
		return self.swap();
	    }
	}
	self.swap()
    }
}

impl<T,I> GroupIter<T,I>
where I: Iterator<Item=T>
{
    fn swap(&mut self) -> Option<Vec<T>>
    {
	let buff = {
	    let next = Vec::with_capacity(self.group_at);
	    std::mem::replace(&mut self.buffer, next)
	};
	if buff.len() > 0 {
	    Some(buff)
	} else {
	    None
	}
    }
}

pub trait GroupExt: Iterator + Sized {
    fn group_at(self, at: usize) -> GroupIter<<Self as Iterator>::Item, Self>
    {
	GroupIter{
	    group_at: at,
	    iter: self,
	    buffer: Vec::with_capacity(at),
	}
    }
}
impl<T> GroupExt for T where T: Iterator{}

