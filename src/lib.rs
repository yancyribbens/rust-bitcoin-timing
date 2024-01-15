use bitcoin::Amount;

#[inline]
pub fn bitcoin_sum(mut accum: Amount, inc: Amount) -> Amount {
	for _x in 1..100_000 {
		accum += inc; 
	}

    accum
}

#[inline]
pub fn primitive_sum(mut accum: u64, inc: u64) -> u64 {
	for _x in 1..200_000 {
		accum += inc; 
	}

    accum}

#[cfg(test)]
mod tests {
}
