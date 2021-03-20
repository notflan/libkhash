use crate::*;

#[cfg(feature="ffi")] 
const _:() = { 
    pub trait HeapArrayExt: Sized
    {
	fn into_unsafe(self) -> Self;
	fn into_safe(self) -> Self;
	fn set_unsafe(self, un: bool) -> Self;
    }
    impl<T> HeapArrayExt for HeapArray<T>
    {
	fn into_unsafe(mut self) -> Self
	{
	    self.drop_check = false;
	    self
	}
	fn into_safe(mut self) -> Self
	{
	    self.drop_check = true;
	    self
	}
	fn set_unsafe(mut self, un: bool) -> Self
	{
	    self.drop_check = un;
	    self
	}
    }
};
