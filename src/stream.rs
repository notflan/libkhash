use crate::*;

/// A streaming kana hash digest.
///
/// This type can be used to generate kana mnemonics from any data.
/// It wraps a type implementing `std::io::Read` and produces a kana mnemonic reading from the stream until its end.
/// ```
/// # use khash::Digest;
/// let input = "Hello world!";
/// let mnemonic: String = Digest::new(&mut input.as_bytes()).collect(); // Read the bytes from the `input` string and collect the kana mnemonic into a `String` 
/// ```
pub struct Digest<'a, T>
where T: Read
{
    input: &'a mut T,
}

impl<'a, T: Read> Digest<'a, T>
{
    /// Create a new stream digest iterator from the input stream.
    pub fn new(input: &'a mut T) -> Self
    {
	Self{input}
    }
}

impl<'a, T: Read> Iterator for Digest<'a, T>
{
    type Item = String; //TODO: Change this to `char` and keep an internal buffer that we `fmt::write!` the mnemonic digest to instead of `format!`ing it.
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
