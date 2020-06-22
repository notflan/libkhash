

pub unsafe fn bytes<'a, T>(src: &'a T) -> &'a [u8]
    where T: ?Sized
{
    std::slice::from_raw_parts(src as *const T as *const u8, std::mem::size_of_val(src))
}

pub unsafe fn bytes_mut<'a, T>(src: &'a mut T) -> &'a mut [u8]
    where T: ?Sized
{
    std::slice::from_raw_parts_mut(src as *mut T as *mut u8, std::mem::size_of_val(src))
}
