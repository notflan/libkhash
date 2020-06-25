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

	let sign0 = from[0] & 0x80 != 0;//unsafe { *reinterpret::value::<i8>(from) < 0 };
	let range = &map::KANA_SIGN[sign0 as usize];
	let kana = &map::KANA[range.clone()];
	let oneesan = usize::from(from[0]) % kana.len();
	let xor = if from.len() > 1 {
	    from[0] ^ from[1]
	} else {
	    from[0]
	} as u32;
	d.0 = Some(match map::KANA_SWAP[range.start()+oneesan] {
	    Some(swap) if xor & 2 == 0 => swap,
	    Some(_) if xor & 8 == 0 &&  map::KANA_SWAP2[range.start() + oneesan].is_some() => map::KANA_SWAP2[range.start()+oneesan].unwrap(),
	    _ => kana[oneesan],
	});
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
