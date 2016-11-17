//! *merkle_sigs* implements Merkle signatures in Rust.

#[doc(no_inline)]
extern crate lamport_sigs;
extern crate merkle;
extern crate crypto;

mod signatures;
pub use signatures::{MerkleSignature, MerkleSignedData, verify_data_vec_signature, sign_data_vec};

#[cfg(test)]
mod tests;
