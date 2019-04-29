use nimiq_mnemonic::Entropy;
use num_bigint::BigUint;

pub(crate) fn add_to_entropy(entropy: &Entropy, value: u64) -> Entropy {
    let num: BigUint = BigUint::from_bytes_be(entropy.as_bytes()) + value;
    let mut new_entropy = [0u8; 32];
    new_entropy.copy_from_slice(&num.to_bytes_be()[..32]);
    Entropy::from(new_entropy)
}