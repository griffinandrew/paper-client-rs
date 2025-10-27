/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	string::FromUtf8Error,
	str::{self, Utf8Error},
	fmt::{self, Formatter},
};


#[cfg(not(feature = "allocator_api"))]
pub struct PaperValue(Box<[u8]>);

#[cfg(not(feature = "allocator_api"))]
impl From<Box<[u8]>> for PaperValue {
	fn from(value: Box<[u8]>) -> Self {
		PaperValue(value)
	}
}

#[cfg(not(feature = "allocator_api"))]
impl From<&[u8]> for PaperValue {
	fn from(value: &[u8]) -> Self {
		let buf = value
			.to_vec()
			.into_boxed_slice();

		PaperValue(buf)
	}
}

#[cfg(not(feature = "allocator_api"))]
impl From<Vec<u8>> for PaperValue {
	fn from(value: Vec<u8>) -> Self {
		PaperValue(value.into_boxed_slice())
	}
}

#[cfg(not(feature = "allocator_api"))]
impl From<&str> for PaperValue {
	fn from(value: &str) -> Self {
		let buf = value
			.as_bytes()
			.to_vec()
			.into_boxed_slice();

		PaperValue(buf)
	}
}

#[cfg(not(feature = "allocator_api"))]
impl From<String> for PaperValue {
	fn from(value: String) -> Self {
		value.as_str().into()
	}
}

#[cfg(not(feature = "allocator_api"))]
impl From<&String> for PaperValue {
	fn from(value: &String) -> Self {
		value.as_str().into()
	}
}

#[cfg(not(feature = "allocator_api"))]
impl From<PaperValue> for Box<[u8]> {
	fn from(value: PaperValue) -> Self {
		value.0
	}
}

#[cfg(not(feature = "allocator_api"))]
impl<'a> From<&'a PaperValue> for &'a [u8] {
	fn from(value: &'a PaperValue) -> Self {
		&value.0
	}
}

#[cfg(not(feature = "allocator_api"))]
impl From<PaperValue> for Vec<u8> {
	fn from(value: PaperValue) -> Self {
		value.0.to_vec()
	}
}

#[cfg(not(feature = "allocator_api"))]
impl<'a> TryFrom<&'a PaperValue> for &'a str {
	type Error = Utf8Error;

	fn try_from(value: &'a PaperValue) -> Result<Self, Self::Error> {
		str::from_utf8(&value.0)
	}
}

#[cfg(not(feature = "allocator_api"))]
impl TryFrom<PaperValue> for String {
	type Error = FromUtf8Error;

	fn try_from(value: PaperValue) -> Result<Self, Self::Error> {
		String::from_utf8(value.0.to_vec())
	}
}

#[cfg(not(feature = "allocator_api"))]
impl fmt::Debug for PaperValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		if self.0.len() > 16 {
			return write!(f, "PaperValue(...)");
		}

		let value: Result<&str, Utf8Error> = self.try_into();

		match value {
			Ok(value) => write!(f, "PaperValue(\"{value}\")"),
			Err(_) => write!(f, "PaperValue(...)"),
		}
	}
}


#[cfg(feature = "allocator_api")]
use crate::allocator::HybridPaperValue as Hybrid;



#[cfg(feature = "allocator_api")]
use std::alloc::Allocator;
pub struct PaperValue<A: Allocator = Hybrid>(Box<[u8], A>);
use std::mem::MaybeUninit;
use std::ptr;


#[cfg(feature = "allocator_api")]
impl From<Box<[u8]>> for PaperValue {
	fn from(value: Box<[u8]>) -> Self {

		let src = value.as_ref();
        let len = src.len();

		let mut buf: Box<[MaybeUninit<u8>], Hybrid> = unsafe {
            Box::new_uninit_slice_in(len, Hybrid)
        };

		unsafe {
            // copy_nonoverlapping is more efficient when we know the memory regions don't overlap.
            ptr::copy(src.as_ptr(), buf.as_mut_ptr().cast(), len);
            // ptr::copy can also be used, which is safe for overlapping regions.
        }

        // SAFETY: The memory has been fully initialized by the copy.
        let buf = unsafe { buf.assume_init() };

        PaperValue(buf)
	
	}
}



#[cfg(feature = "allocator_api")]
impl From<&[u8]> for PaperValue {
	fn from(value: &[u8]) -> Self {
		let vec = value.to_vec_in(Hybrid);
        PaperValue(vec.into_boxed_slice())
	}
}



#[cfg(feature = "allocator_api")]
impl From<Vec<u8>> for PaperValue {
	fn from(value: Vec<u8>) -> Self {
		let vec = value.to_vec_in(Hybrid);
		let boxed = vec.into_boxed_slice();
		PaperValue(boxed)
	}
}

#[cfg(feature = "allocator_api")]
impl From<&str> for PaperValue {
	fn from(value: &str) -> Self {
		value.as_bytes().into()
	}
}

#[cfg(feature = "allocator_api")]
impl From<String> for PaperValue {
	fn from(value: String) -> Self {
		value.as_str().into()
	}
}

#[cfg(feature = "allocator_api")]
impl From<&String> for PaperValue {
	fn from(value: &String) -> Self {
		value.as_str().into()
	}
}

#[cfg(feature = "allocator_api")]
impl From<PaperValue> for Box<[u8], Hybrid> {
	fn from(value: PaperValue) -> Self {
		value.0
	}
}

#[cfg(feature = "allocator_api")]
impl<'a> From<&'a PaperValue> for &'a [u8] {
	fn from(value: &'a PaperValue) -> Self {
		&value.0
	}
}

#[cfg(feature = "allocator_api")]
impl From<PaperValue> for Vec<u8> {
	fn from(value: PaperValue) -> Self {
		value.0.to_vec()
	}
}

#[cfg(feature = "allocator_api")]
impl<'a> TryFrom<&'a PaperValue> for &'a str {
	type Error = Utf8Error;

	fn try_from(value: &'a PaperValue) -> Result<Self, Self::Error> {
		str::from_utf8(&value.0)
	}
}

#[cfg(feature = "allocator_api")]
impl TryFrom<PaperValue> for String {
	type Error = FromUtf8Error;

	fn try_from(value: PaperValue) -> Result<Self, Self::Error> {
		String::from_utf8(value.0.to_vec())
	}
}


#[cfg(feature = "allocator_api")]
impl fmt::Debug for PaperValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		if self.0.len() > 16 {
			return write!(f, "PaperValue(...)");
		}

		let value: Result<&str, Utf8Error> = self.try_into();

		match value {
			Ok(value) => write!(f, "PaperValue(\"{value}\")"),
			Err(_) => write!(f, "PaperValue(...)"),
		}
	}
}

