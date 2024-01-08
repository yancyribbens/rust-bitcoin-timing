use bitcoin::Amount;
use core::str::FromStr;

#[inline]
pub fn bitcoin_sum() -> Amount {
	let mut start = Amount::ZERO;
	let one = Amount::from_str("1 sat").unwrap();

	for _x in 1..100_000 {
		start += one; 
	}

	start
}

#[inline]
pub fn primitive_sum() -> u64 {
	let mut start:u64 = 0;

	for _x in 1..100_000 {
		start += 1; 
	}

	start
}

#[cfg(test)]
mod tests {
}
