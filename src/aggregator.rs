pub const HASH_LENGTH_U64: usize = 63;

pub fn aggregate_hashes<T>(hashes: T) -> [u64; HASH_LENGTH_U64]
where
    T: AsRef<[[u64; HASH_LENGTH_U64]]>,
{
    let hashes = hashes.as_ref();
    let mut res = [0u64; HASH_LENGTH_U64];
    for hash in hashes {
        append_hash(&mut res, hash);
    }
    res
}

fn append_hash(base_hash: &mut [u64; HASH_LENGTH_U64], hash: &[u64; HASH_LENGTH_U64]) {
    let mut carry = 0u64;

    for (base, &h) in base_hash.iter_mut().zip(hash.iter()) {
        let (sum, overflow) = base.overflowing_add(h);
        let (sum_with_carry, carry_overflow) = sum.overflowing_add(carry);
        *base = sum_with_carry;
        carry = (overflow | carry_overflow) as u64;
    }
}
