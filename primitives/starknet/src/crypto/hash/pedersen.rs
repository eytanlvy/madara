//! Pedersen hash module.
use starknet_crypto::{pedersen_hash, FieldElement};

use crate::traits::hash::Hasher;

/// The Pedersen hash function.
/// ### Arguments
/// * `x`: The x coordinate
/// * `y`: The y coordinate
pub fn hash(_data: &[u8]) -> [u8; 32] {
	let field_element = FieldElement::from_byte_slice_be(_data).unwrap();
	let result = FieldElement::to_bytes_be(&pedersen_hash(&FieldElement::ZERO, &field_element));
	result
}

/// The Pedersen hasher.
#[derive(Default)]
pub struct PedersenHasher;

/// The Pedersen hasher implementation.
impl Hasher for PedersenHasher {
	/// Hashes the given data.
	/// # Arguments
	/// * `data` - The data to hash.
	/// # Returns
	/// The hash of the data.
	fn hash(&self, data: &[u8]) -> [u8; 32] {
		hash(data)
	}
}
