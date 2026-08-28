/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	fmt::{self, Display},
	str::FromStr,
};

use crate::error::PaperClientError;

/// A cache policy.
///
/// `Other` is the important one for this fork. The server it talks to is
/// paper-cache-cxl, whose hybrid designs -- `lru-hybrid`, `s3-fifo-hybrid-0.1`
/// and the rest -- are unknown here, and enumerating them would mean editing
/// this crate every time a design is added, with STATUS silently failing to
/// parse whenever it fell behind. Instead an unrecognized policy round-trips
/// verbatim: the client neither validates nor interprets it, which is the
/// server's job anyway.
///
/// This costs `Copy`, since the variant owns a `String`. `Clone` remains.
#[derive(Debug, Clone, PartialEq)]
pub enum PaperPolicy {
	Auto,
	Lfu,
	Fifo,
	Clock,
	Sieve,
	Lru,
	Mru,
	TwoQ(f64, f64),
	Arc,
	SThreeFifo(f64),

	/// A policy this client does not know; carried through unchanged.
	Other(String),
}

impl Display for PaperPolicy {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PaperPolicy::Auto => write!(f, "auto"),
			PaperPolicy::Lfu => write!(f, "lfu"),
			PaperPolicy::Fifo => write!(f, "fifo"),
			PaperPolicy::Clock => write!(f, "clock"),
			PaperPolicy::Sieve => write!(f, "sieve"),
			PaperPolicy::Lru => write!(f, "lru"),
			PaperPolicy::Mru => write!(f, "mru"),
			PaperPolicy::TwoQ(k_in, k_out) => write!(f, "2q-{k_in}-{k_out}"),
			PaperPolicy::Arc => write!(f, "arc"),
			PaperPolicy::SThreeFifo(ratio) => write!(f, "s3-fifo-{ratio}"),
			PaperPolicy::Other(name) => write!(f, "{name}"),
		}
	}
}

impl FromStr for PaperPolicy {
	type Err = PaperClientError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let policy = match value {
			"auto" => PaperPolicy::Auto,
			"lfu" => PaperPolicy::Lfu,
			"fifo" => PaperPolicy::Fifo,
			"clock" => PaperPolicy::Clock,
			"sieve" => PaperPolicy::Sieve,
			"lru" => PaperPolicy::Lru,
			"mru" => PaperPolicy::Mru,
			// A prefix match is not sufficient: "2q-hybrid-0.1" starts with
			// "2q-" but its tail is not a k_in/k_out pair. Fall through to
			// `Other` rather than rejecting -- the server understands it even
			// though this client does not.
			value if value.starts_with("2q-") => parse_two_q(value)
				.unwrap_or_else(|_| PaperPolicy::Other(value.to_owned())),
			"arc" => PaperPolicy::Arc,
			value if value.starts_with("s3-fifo-") => parse_s_three_fifo(value)
				.unwrap_or_else(|_| PaperPolicy::Other(value.to_owned())),

			value => PaperPolicy::Other(value.to_owned()),
		};

		Ok(policy)
	}
}

fn parse_two_q(value: &str) -> Result<PaperPolicy, PaperClientError> {
	// skip the "2q-"
	let tokens = value[3..].split('-').collect::<Vec<&str>>();

	if tokens.len() != 2 {
		return Err(PaperClientError::Internal);
	}

	let Ok(k_in) = tokens[0].parse::<f64>() else {
		return Err(PaperClientError::Internal);
	};

	let Ok(k_out) = tokens[1].parse::<f64>() else {
		return Err(PaperClientError::Internal);
	};

	Ok(PaperPolicy::TwoQ(k_in, k_out))
}

fn parse_s_three_fifo(value: &str) -> Result<PaperPolicy, PaperClientError> {
	// skip the "s3-fifo-"
	let tokens = value[8..].split('-').collect::<Vec<&str>>();

	if tokens.len() != 1 {
		return Err(PaperClientError::Internal);
	}

	let Ok(ratio) = tokens[0].parse::<f64>() else {
		return Err(PaperClientError::Internal);
	};

	Ok(PaperPolicy::SThreeFifo(ratio))
}
