use std::{
    fmt,
    io,
    error,
};

#[derive(Debug)]
pub enum Error
{
    IO(io::Error),
    Format(fmt::Error),
    Length{expected: usize, got:usize,},
    RNG(getrandom::Error),
    Unknown,
}

impl error::Error for Error
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)>
    {
	match &self {
	    Error::IO(e_io) => Some(e_io),
	    Error::Format(e_fmt) => Some(e_fmt),
	    Error::RNG(e_rng) => Some(e_rng),
	    _ => None,
	}
    }
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "kana-hash error: ")?;
	match self {
	    Error::IO(io) => write!(f, "io: {}", io),
	    Error::Format(fmt) => write!(f, "fmt: {}", fmt),
	    Error::Length{expected, got} => write!(f, "invalid length: expected {}, got {}", expected, got),
	    Error::RNG(rng) => write!(f, "rng error: {}", rng),
	    _ => write!(f, "unknown failure"),
	}
    }
}

impl From<Error> for i32
{
    fn from(er: Error) -> Self
    {
	match er {
	    Error::IO(_) => 1,
	    Error::Format(_) => 2,
	    Error::Length{..} => 3,
	    Error::RNG(_) => 4,
	    _ => -1,
	}
    }
}

impl From<io::Error> for Error
{
    fn from(i: io::Error) -> Self
    {
	Self::IO(i)
    }
}
impl From<fmt::Error> for Error
{
    fn from(i: fmt::Error) -> Self
    {
	Self::Format(i)
    }
}

impl From<getrandom::Error> for Error
{
    fn from(rng: getrandom::Error) -> Self
    {
	Self::RNG(rng)
    }
}
