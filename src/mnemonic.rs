use crate::*;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct Digest(Option<char>, Option<char>);

impl Default for Digest
{
    fn default() -> Self
    {
	Self(None,None)
    }
}

impl Digest {
    /// Create new single 2-byte digest.
    pub fn new(from: &[u8]) -> Self
    {
	let mut d = Self::default();

	let oneesan = usize::from(from[0]) % map::KANA.len();
	d.0 = Some(map::KANA[oneesan]);
	if from[1] > 0 {
	    if let Some(imoutos) = map::sub(oneesan) {
		let one = (usize::from(from[1]) / map::KANA.len()) % 2;
		if imoutos.len() > 0 && one > 0{
		    d.1 = Some(imoutos[usize::from(from[1]) % imoutos.len()]);
		    return d;
		}
	    }
	    let from = [from[1], 0];
	    d.1 = Self::new(&from[..]).0;
	}
	d
	/*let oneesan = usize::from(from) % map::KANA.len();
	d.0 = Some(map::KANA[oneesan]);
	if let Some(imoutos) = map::sub(oneesan) {
	    if imoutos.len() > 0 {
	    }
	} else {
	    
	}

	return d;*/
    }
}

use std::fmt;
impl fmt::Display for Digest
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	if let Some(oneesan) = self.0 {
	    write!(f, "{}", oneesan)?;
	}
	if let Some(imouto) = self.1 {
	    write!(f, "{}", imouto)?;
	}
	Ok(())
    }
}
