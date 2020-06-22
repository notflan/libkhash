#![allow(unused_macros)]

pub const GENERIC_ERROR: i32 = -1;
pub const GENERIC_SUCCESS: i32 = 0;

macro_rules! c_try {
    ($e:expr) => {
	match $e {
	    Ok(v) => v,
	    Err(e) => return i32::from(e),
	}
    }
}

macro_rules! no_unwind {
    (try $t:expr; $($e:tt)*) => {
	{
	    match std::panic::catch_unwind(|| {
		$($e)*
	    }) {
		Ok(v) => i32::from(v),
		Err(_) => return i32::from($t),
	    }
	}
    };
    ($($e:tt)*) => {
	no_unwind! {try $crate::ffi::GENERIC_ERROR; $($e)*}
    }
    
}


macro_rules! string_from_ffi {
    ($file:expr) => {
	unsafe {
	    let file = $file;
	    if file.is_null() {
		return $crate::ffi::GENERIC_ERROR;
	    }
	    let file = CStr::from_ptr(file);
	    match file.to_str() {
		Ok(file) => file.to_owned(),
		Err(_) => return $crate::ffi::GENERIC_ERROR,
	    }
	}
    }
}
