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
	if from.len() == 0 {
	    return d;
	}

	let sign0 = unsafe { *reinterpret::value::<i8>(from) < 0 };
	let range = &map::KANA_SIGN[sign0 as usize];
	let kana = &map::KANA[range.clone()];
	let oneesan = usize::from(from[0]) % kana.len();
	d.0 = Some(kana[oneesan]);
	if from.len() > 1 {
	    if let Some(imoutos) = map::sub(range.start()+oneesan) {
		if let Some(imouto) = imoutos[usize::from(from[1]) % map::KANA_SUB.len()]
		{
		    d.1 = Some(imouto);
		    return d;
		}
	    }
	    let from = [from[1]];
	    d.1 = Self::new(&from[..]).0;
	}
	d
	// Old
	/*let mut d = Self::default();

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
	    d*/
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
