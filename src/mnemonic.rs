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

	let master = usize::from(from[0]) % map::KANA.len();
	d.0 = Some(map::KANA[master]);
	if from[1] > 0 {
	    if let Some(slaves) = map::sub(master) {
		if slaves.len() > 0 {
		    d.1 = Some(slaves[usize::from(from[1]) % slaves.len()]);
		    return d;
		}
	    }
	    let from = [from[1]];
	    d.1 = Self::new(&from[..]).0;
	}
	d
	/*let master = usize::from(from) % map::KANA.len();
	d.0 = Some(map::KANA[master]);
	if let Some(slaves) = map::sub(master) {
	    if slaves.len() > 0 {
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
	if let Some(master) = self.0 {
	    write!(f, "{}", master)?;
	}
	if let Some(slave) = self.1 {
	    write!(f, "{}", slave)?;
	}
	Ok(())
    }
}
