use crate::*;

/// A streaming kana hash digest
pub struct Digest<'a, T>
where T: Read
{
    input: &'a mut T,
}

impl<'a, T: Read> Digest<'a, T>
{
    /// Create a new stream digest from the input
    pub fn new(input: &'a mut T) -> Self
    {
	Self{input}
    }
}

impl<'a, T: Read> Iterator for Digest<'a, T>
{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item>
    {
	let mut buffer = [0u8; 2];
	let mut rd =0;
	while rd < 2 {
	    match self.input.read(&mut buffer[rd..]) {
		Ok(2) => break,
		Err(_) | Ok(0) => return None,
		Ok(v) => rd+=v,
	    }
	}
	Some(format!("{}",mnemonic::Digest::new(&buffer[..])))
    }
}
