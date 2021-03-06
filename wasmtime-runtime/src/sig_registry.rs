//! Implement a registry of function signatures, for fast indirect call
//! signature checking.

use crate::vmcontext::VMSharedSignatureIndex;
use cast;
use cranelift_codegen::ir;
use std::collections::{hash_map, HashMap};

/// WebAssembly requires that the caller and callee signatures in an indirect
/// call must match. To implement this efficiently, keep a registry of all
/// signatures, shared by all instances, so that call sites can just do an
/// index comparison.
#[derive(Debug)]
pub struct SignatureRegistry {
    signature_hash: HashMap<ir::Signature, VMSharedSignatureIndex>,
}

impl SignatureRegistry {
    /// Create a new `SignatureRegistry`.
    pub fn new() -> Self {
        Self {
            signature_hash: HashMap::new(),
        }
    }

    /// Register a signature and return its unique index.
    pub fn register(&mut self, sig: &ir::Signature) -> VMSharedSignatureIndex {
        let len = self.signature_hash.len();
        match self.signature_hash.entry(sig.clone()) {
            hash_map::Entry::Occupied(entry) => *entry.get(),
            hash_map::Entry::Vacant(entry) => {
                #[cfg(target_pointer_width = "32")]
                let sig_id = VMSharedSignatureIndex::new(cast::u32(len));
                #[cfg(target_pointer_width = "64")]
                let sig_id = VMSharedSignatureIndex::new(cast::u32(len).unwrap());
                entry.insert(sig_id);
                sig_id
            }
        }
    }
}
