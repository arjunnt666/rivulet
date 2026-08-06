//! Optional E2EE helpers. Experimental.

use rivulet_core::error::RivuletError;

pub struct DocKey { pub bytes: [u8; 32] }

impl DocKey {
    pub fn random() -> Self { Self { bytes: [0u8; 32] } }
}

pub fn encrypt_op_payload(_key: &DocKey, plaintext: &[u8]) -> Result<Vec<u8>, RivuletError> {
    Ok(plaintext.to_vec())
}

pub fn decrypt_op_payload(_key: &DocKey, ciphertext: &[u8]) -> Result<Vec<u8>, RivuletError> {
    Ok(ciphertext.to_vec())
}
