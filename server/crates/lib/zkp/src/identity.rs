use bellman::{
    groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, Proof},
    Circuit, ConstraintSystem, SynthesisError,
};
use bls12_381::Bls12;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::sync::Once;
use once_cell::sync::Lazy;
use salvo::oapi::ToSchema;

use crate::{hash_to_field, ZKPError, ZKProof};

// --- Personhood Circuit ---

#[derive(Clone)]
pub struct PersonhoodCircuit {
    pub personal_id: Option<String>,
    pub salt: Option<String>,
}

impl Circuit<bls12_381::Scalar> for PersonhoodCircuit {
    fn synthesize<CS: ConstraintSystem<bls12_381::Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let combined = format!(
            "{}{}",
            self.personal_id.as_deref().unwrap_or(""),
            self.salt.as_deref().unwrap_or("")
        );

        let combined_hash = hash_to_field(&combined).map_err(|_| SynthesisError::Unsatisfiable)?;

        // private variable representing the hash of ID + salt
        let combined_var = cs.alloc(|| "personhood secret hash", || Ok(combined_hash))?;

        // public variable representing the expected output hash (the Personhood DID/Hash)
        let public_var = cs.alloc_input(|| "public personhood hash", || Ok(combined_hash))?;

        // Constraint: private hash must equal public hash
        cs.enforce(|| "personhood hash constraint", |lc| lc + combined_var, |lc| lc + CS::one(), |lc| lc + public_var);

        Ok(())
    }
}

pub(crate) static PERSONHOOD_PARAMS: Lazy<bellman::groth16::Parameters<Bls12>> = Lazy::new(|| {
    static INIT: Once = Once::new();
    let mut params = None;
    INIT.call_once(|| {
        let rng = &mut thread_rng();
        let circuit = PersonhoodCircuit {
            personal_id: Some("test_id".to_string()),
            salt: Some("test_salt".to_string()),
        };
        params = Some(generate_random_parameters::<Bls12, _, _>(circuit, rng).expect("Failed to generate parameters"));
    });
    params.unwrap()
});

impl PersonhoodCircuit {
    pub fn new(personal_id: &str, salt: &str) -> Self {
        Self {
            personal_id: Some(personal_id.to_string()),
            salt: Some(salt.to_string()),
        }
    }

    pub fn create_proof(&self) -> Result<ZKProof, ZKPError> {
        let rng = &mut thread_rng();
        let combined = format!(
            "{}{}",
            self.personal_id.as_deref().unwrap_or(""),
            self.salt.as_deref().unwrap_or("")
        );

        let _ = hash_to_field(&combined).map_err(|_| ZKPError::HashError("".into()))?;
        
        // Use the string as public input representation for now
        let public_input_str = combined.clone();

        let params: &bellman::groth16::Parameters<Bls12> = &PERSONHOOD_PARAMS;
        let proof = create_random_proof(self.clone(), params, rng)?;

        let mut proof_bytes = Vec::new();
        proof.write(&mut proof_bytes).map_err(|_| ZKPError::SerializationError)?;

        Ok(ZKProof {
            proof: proof_bytes,
            public_inputs: vec![public_input_str],
        })
    }
}

pub fn verify_personhood_proof(zkproof: &ZKProof) -> Result<(), ZKPError> {
    let pvk = prepare_verifying_key(&PERSONHOOD_PARAMS.vk);

    let proof = match Proof::read(&zkproof.proof[..]) {
        Ok(p) => p,
        Err(_) => return Err(ZKPError::SerializationError),
    };

    let inputs: Vec<bls12_381::Scalar> = match zkproof.public_inputs.iter().map(|s| hash_to_field(s)).collect::<Result<_, _>>() {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    if bellman::groth16::verify_proof(&pvk, &proof, &inputs).is_ok() {
        Ok(())
    } else {
        Err(ZKPError::InvalidProof)
    }
}


// --- Agent Binding Circuit ---

#[derive(Clone)]
pub struct AgentCircuit {
    pub owner_personhood_hash: Option<String>,
    pub agent_pubkey: Option<String>,
}

impl Circuit<bls12_381::Scalar> for AgentCircuit {
    fn synthesize<CS: ConstraintSystem<bls12_381::Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let combined = format!(
            "{}{}",
            self.owner_personhood_hash.as_deref().unwrap_or(""),
            self.agent_pubkey.as_deref().unwrap_or("")
        );

        let combined_hash = hash_to_field(&combined).map_err(|_| SynthesisError::Unsatisfiable)?;

        let combined_var = cs.alloc(|| "agent binding hash", || Ok(combined_hash))?;
        let public_var = cs.alloc_input(|| "public agent binding hash", || Ok(combined_hash))?;

        cs.enforce(|| "agent binding constraint", |lc| lc + combined_var, |lc| lc + CS::one(), |lc| lc + public_var);

        Ok(())
    }
}

pub(crate) static AGENT_PARAMS: Lazy<bellman::groth16::Parameters<Bls12>> = Lazy::new(|| {
    static INIT: Once = Once::new();
    let mut params = None;
    INIT.call_once(|| {
        let rng = &mut thread_rng();
        let circuit = AgentCircuit {
            owner_personhood_hash: Some("test_owner_hash".to_string()),
            agent_pubkey: Some("test_agent_pk".to_string()),
        };
        params = Some(generate_random_parameters::<Bls12, _, _>(circuit, rng).expect("Failed to generate parameters"));
    });
    params.unwrap()
});

impl AgentCircuit {
    pub fn new(owner_personhood_hash: &str, agent_pubkey: &str) -> Self {
        Self {
            owner_personhood_hash: Some(owner_personhood_hash.to_string()),
            agent_pubkey: Some(agent_pubkey.to_string()),
        }
    }

    pub fn create_proof(&self) -> Result<ZKProof, ZKPError> {
        let rng = &mut thread_rng();
        let combined = format!(
            "{}{}",
            self.owner_personhood_hash.as_deref().unwrap_or(""),
            self.agent_pubkey.as_deref().unwrap_or("")
        );

        let _ = hash_to_field(&combined).map_err(|_| ZKPError::HashError("".into()))?;
        let public_input_str = combined.clone();

        let params: &bellman::groth16::Parameters<Bls12> = &AGENT_PARAMS;
        let proof = create_random_proof(self.clone(), params, rng)?;

        let mut proof_bytes = Vec::new();
        proof.write(&mut proof_bytes).map_err(|_| ZKPError::SerializationError)?;

        Ok(ZKProof {
            proof: proof_bytes,
            public_inputs: vec![public_input_str],
        })
    }
}

pub fn verify_agent_proof(zkproof: &ZKProof) -> Result<(), ZKPError> {
    let pvk = prepare_verifying_key(&AGENT_PARAMS.vk);

    let proof = match Proof::read(&zkproof.proof[..]) {
        Ok(p) => p,
        Err(_) => return Err(ZKPError::SerializationError),
    };

    let inputs: Vec<bls12_381::Scalar> = match zkproof.public_inputs.iter().map(|s| hash_to_field(s)).collect::<Result<_, _>>() {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    if bellman::groth16::verify_proof(&pvk, &proof, &inputs).is_ok() {
        Ok(())
    } else {
        Err(ZKPError::InvalidProof)
    }
}
