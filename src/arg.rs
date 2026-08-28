/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

/// A cache key.
///
/// Bytes rather than `&str`, which is what the wire has always carried: the
/// key is written with a `u32` length prefix and the server stores it as a
/// `Buffer`, never interpreting it as UTF-8. So this is not a protocol change
/// -- `write_str` was only ever `write_buf(s.as_bytes())`.
///
/// The reason it matters is allocation. The benchmark's traces are `u64`
/// keyed, and with a `&str`-only trait every request had to format its key
/// into a `String` first -- an allocation per operation, inside the region
/// being timed, and exactly what paper-benchmark commit 0df0ba9 removed from
/// the in-process path. With bytes, `key.to_le_bytes()` is an eight-byte array
/// on the stack and there is nothing to allocate.
pub trait AsPaperKey {
	fn as_paper_key(&self) -> &[u8];
}

pub trait AsPaperAuthToken {
	fn as_paper_auth_token(&self) -> &str;
}

impl AsPaperKey for &str {
	fn as_paper_key(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsPaperKey for String {
	fn as_paper_key(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsPaperKey for &String {
	fn as_paper_key(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsPaperKey for &[u8] {
	fn as_paper_key(&self) -> &[u8] {
		self
	}
}

/// The zero-allocation path for numeric keys: `client.get(key.to_le_bytes())`
/// borrows an array that lives in the caller's frame.
impl<const N: usize> AsPaperKey for [u8; N] {
	fn as_paper_key(&self) -> &[u8] {
		self
	}
}

impl<const N: usize> AsPaperKey for &[u8; N] {
	fn as_paper_key(&self) -> &[u8] {
		*self
	}
}

impl AsPaperAuthToken for &str {
	fn as_paper_auth_token(&self) -> &str {
		self
	}
}

impl AsPaperAuthToken for String {
	fn as_paper_auth_token(&self) -> &str {
		self
	}
}

impl AsPaperAuthToken for &String {
	fn as_paper_auth_token(&self) -> &str {
		self
	}
}
