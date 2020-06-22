


pub fn copy_slice<T,D,S>(mut dst: D, src: S) -> usize
where T: Clone,
      D: AsMut<[T]>,
      S: AsRef<[T]>
{
    let mut i =0;
    for (d,s) in dst.as_mut().iter_mut().zip(src.as_ref().iter())
    {
	*d = s.clone();
	i+=1;
    }
    i
}
