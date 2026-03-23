use std::fmt;

use blake3;
use bls12_381::Scalar;
use hex;
use log;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ZKPError {
    SerializationError,
    InvalidProof,
    InvalidPublicInputs,
    SynthesisError(String),
    HashError(String),
    UnsupportedPredicate,
    NotFound(String),
    InvalidIssuer(String),
    InvalidSignature,
    ExpiredCredential,
    RevokedCredential,
    OtherError(String),
}

impl fmt::Display for ZKPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZKPError::SerializationError => write!(f, "serialization error"),
            ZKPError::InvalidProof => write!(f, "invalid proof"),
            ZKPError::InvalidPublicInputs => write!(f, "invalid public inputs"),
            ZKPError::SynthesisError(msg) => write!(f, "synthesis error: {}", msg),
            ZKPError::HashError(msg) => write!(f, "hash error: {}", msg),
            ZKPError::UnsupportedPredicate => write!(f, "unsupported predicate"),
            ZKPError::NotFound(msg) => write!(f, "not found: {}", msg),
            ZKPError::InvalidIssuer(msg) => write!(f, "invalid issuer: {}", msg),
            ZKPError::InvalidSignature => write!(f, "invalid signature"),
            ZKPError::ExpiredCredential => write!(f, "credential expired"),
            ZKPError::RevokedCredential => write!(f, "credential revoked"),
            ZKPError::OtherError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ZKPError {}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ZKProof {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<String>,
}

pub fn hash_to_field(input: &str) -> Result<Scalar, ZKPError> {
    let hash = blake3::hash(input.as_bytes());
    let hash_bytes = hash.as_bytes();

    let mut wide = [0u8; 64];
    wide[..32].copy_from_slice(hash_bytes);

    let scalar = Scalar::from_bytes_wide(&wide);
    log::debug!(
        "hash_to_field input='{}' hash=0x{} scalar=0x{}",
        input,
        hex::encode(hash_bytes),
        hex::encode(scalar.to_bytes())
    );
    Ok(scalar)
}

pub fn public_input_to_scalar(input: &str) -> Result<Scalar, ZKPError> {
    if let Some(raw) = input.strip_prefix("num:") {
        let value = raw
            .parse::<u64>()
            .map_err(|e| ZKPError::HashError(format!("failed to parse numeric public input: {}", e)))?;
        Ok(Scalar::from(value))
    } else {
        hash_to_field(input)
    }
}
