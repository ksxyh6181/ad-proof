use std::collections::HashMap;
use std::sync::Mutex;

use bellman::{
    groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Parameters, Proof},
    Circuit, ConstraintSystem, SynthesisError,
};
use blake3;
use bls12_381::{Bls12, Scalar};
use chrono::{NaiveDate, Utc};
use once_cell::sync::Lazy;
use rand::thread_rng;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

use crate::{hash_to_field, public_input_to_scalar, ZKPError, ZKProof};

const LEVEL_COUNT: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub enum CredentialKind {
    Income,
    Kyc,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub enum CredentialStatus {
    Active,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CredentialHeader {
    pub credential_id: String,
    pub kind: CredentialKind,
    pub issuer_id: String,
    pub subject_ref: String,
    pub issued_at: String,
    pub expires_at: String,
    pub status: CredentialStatus,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IncomePresentation {
    pub header: CredentialHeader,
    pub required_tier: u8,
    pub proof: ZKProof,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KycPresentation {
    pub header: CredentialHeader,
    pub required_level: u8,
    pub proof: ZKProof,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PresentationVerification {
    pub credential_id: String,
    pub issuer_id: String,
    pub kind: CredentialKind,
    pub valid: bool,
    pub message: String,
    pub expires_at: String,
}

#[derive(Debug, Clone)]
struct StoredIncomeCredential {
    header: CredentialHeader,
    income_tier: u8,
}

#[derive(Debug, Clone)]
struct StoredKycCredential {
    header: CredentialHeader,
    kyc_level: u8,
}

static INCOME_CREDENTIALS: Lazy<Mutex<HashMap<String, StoredIncomeCredential>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static KYC_CREDENTIALS: Lazy<Mutex<HashMap<String, StoredKycCredential>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone)]
struct TierPredicateCircuit {
    statement_hash: Option<Scalar>,
    required_flags: [Option<bool>; LEVEL_COUNT],
    claim_flags: [Option<bool>; LEVEL_COUNT],
}

impl Circuit<Scalar> for TierPredicateCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let statement_hash = self.statement_hash.ok_or(SynthesisError::AssignmentMissing)?;
        let public_statement = cs.alloc_input(|| "statement hash", || Ok(statement_hash))?;
        let private_statement = cs.alloc(|| "statement witness", || Ok(statement_hash))?;
        cs.enforce(
            || "bind statement hash",
            |lc| lc + private_statement,
            |lc| lc + CS::one(),
            |lc| lc + public_statement,
        );

        let mut required_vars = Vec::with_capacity(LEVEL_COUNT);
        for idx in 0..LEVEL_COUNT {
            let value = if self.required_flags[idx].unwrap_or(false) {
                Scalar::from(1u64)
            } else {
                Scalar::from(0u64)
            };
            let var = cs.alloc_input(|| format!("required_flag_{}", idx), || Ok(value))?;
            cs.enforce(
                || format!("required_flag_bool_{}", idx),
                |lc| lc + var,
                |lc| lc + CS::one() - var,
                |lc| lc,
            );
            required_vars.push(var);
        }

        cs.enforce(
            || "required_flags_sum_to_one",
            |lc| {
                let mut acc = lc;
                for var in &required_vars {
                    acc = acc + *var;
                }
                acc
            },
            |lc| lc + CS::one(),
            |lc| lc + CS::one(),
        );

        let mut claim_vars = Vec::with_capacity(LEVEL_COUNT);
        for idx in 0..LEVEL_COUNT {
            let value = if self.claim_flags[idx].unwrap_or(false) {
                Scalar::from(1u64)
            } else {
                Scalar::from(0u64)
            };
            let var = cs.alloc(|| format!("claim_flag_{}", idx), || Ok(value))?;
            cs.enforce(
                || format!("claim_flag_bool_{}", idx),
                |lc| lc + var,
                |lc| lc + CS::one() - var,
                |lc| lc,
            );
            claim_vars.push(var);
        }

        cs.enforce(
            || "claim_flags_sum_to_one",
            |lc| {
                let mut acc = lc;
                for var in &claim_vars {
                    acc = acc + *var;
                }
                acc
            },
            |lc| lc + CS::one(),
            |lc| lc + CS::one(),
        );

        for required_idx in 0..LEVEL_COUNT {
            cs.enforce(
                || format!("predicate_satisfied_{}", required_idx),
                |lc| lc + required_vars[required_idx],
                |lc| {
                    let mut allowed = lc + CS::one();
                    for claim_idx in required_idx..LEVEL_COUNT {
                        allowed = allowed - claim_vars[claim_idx];
                    }
                    allowed
                },
                |lc| lc,
            );
        }

        Ok(())
    }
}

static TIER_PARAMS: Lazy<Parameters<Bls12>> = Lazy::new(|| {
    let rng = &mut thread_rng();
    let circuit = TierPredicateCircuit {
        statement_hash: Some(Scalar::from(1u64)),
        required_flags: [Some(true), Some(false), Some(false), Some(false)],
        claim_flags: [Some(true), Some(false), Some(false), Some(false)],
    };
    generate_random_parameters::<Bls12, _, _>(circuit, rng).expect("failed to generate tier predicate parameters")
});

fn canonical_header(kind: &CredentialKind, credential_id: &str, issuer_id: &str, subject_ref: &str, issued_at: &str, expires_at: &str, status: &CredentialStatus) -> String {
    format!(
        "{:?}|{}|{}|{}|{}|{}|{:?}",
        kind, credential_id, issuer_id, subject_ref, issued_at, expires_at, status
    )
}

fn issuer_secret(kind: &CredentialKind, issuer_id: &str) -> Option<&'static str> {
    match (kind, issuer_id) {
        (CredentialKind::Income, "bank_demo_001") => Some("income-demo-secret"),
        (CredentialKind::Kyc, "kyc_demo_001") => Some("kyc-demo-secret"),
        _ => None,
    }
}

fn sign_header(header: &CredentialHeader) -> Result<String, ZKPError> {
    let secret = issuer_secret(&header.kind, &header.issuer_id)
        .ok_or_else(|| ZKPError::InvalidIssuer(header.issuer_id.clone()))?;
    let payload = canonical_header(
        &header.kind,
        &header.credential_id,
        &header.issuer_id,
        &header.subject_ref,
        &header.issued_at,
        &header.expires_at,
        &header.status,
    );
    Ok(blake3::hash(format!("{}|{}", secret, payload).as_bytes()).to_hex().to_string())
}

fn verify_header_signature(header: &CredentialHeader) -> Result<(), ZKPError> {
    let expected = sign_header(header)?;
    if expected == header.signature {
        Ok(())
    } else {
        Err(ZKPError::InvalidSignature)
    }
}

fn ensure_not_expired(expires_at: &str) -> Result<(), ZKPError> {
    let expiry = NaiveDate::parse_from_str(expires_at, "%Y-%m-%d")
        .map_err(|e| ZKPError::OtherError(format!("invalid expires_at format: {}", e)))?;
    if expiry < Utc::now().date_naive() {
        Err(ZKPError::ExpiredCredential)
    } else {
        Ok(())
    }
}

fn tier_to_flags(level: u8) -> Result<[bool; LEVEL_COUNT], ZKPError> {
    if level as usize >= LEVEL_COUNT {
        return Err(ZKPError::UnsupportedPredicate);
    }

    let mut flags = [false; LEVEL_COUNT];
    flags[level as usize] = true;
    Ok(flags)
}

fn tier_public_inputs(statement_id: &str, required_level: u8) -> Result<Vec<String>, ZKPError> {
    let flags = tier_to_flags(required_level)?;
    let mut public_inputs = vec![statement_id.to_string()];
    for flag in flags {
        public_inputs.push(format!("num:{}", if flag { 1 } else { 0 }));
    }
    Ok(public_inputs)
}

fn create_tier_predicate_proof(statement_id: &str, claim_level: u8, required_level: u8) -> Result<ZKProof, ZKPError> {
    let claim_flags = tier_to_flags(claim_level)?;
    let required_flags = tier_to_flags(required_level)?;
    let statement_hash = hash_to_field(statement_id)?;

    let circuit = TierPredicateCircuit {
        statement_hash: Some(statement_hash),
        required_flags: required_flags.map(Some),
        claim_flags: claim_flags.map(Some),
    };

    let rng = &mut thread_rng();
    let proof = create_random_proof(circuit, &*TIER_PARAMS, rng)
        .map_err(|e| ZKPError::SynthesisError(e.to_string()))?;

    let mut proof_bytes = Vec::new();
    proof
        .write(&mut proof_bytes)
        .map_err(|_| ZKPError::SerializationError)?;

    Ok(ZKProof {
        proof: proof_bytes,
        public_inputs: tier_public_inputs(statement_id, required_level)?,
    })
}

fn verify_tier_predicate_proof(zkproof: &ZKProof) -> Result<(), ZKPError> {
    if zkproof.public_inputs.len() != LEVEL_COUNT + 1 {
        return Err(ZKPError::InvalidPublicInputs);
    }

    let pvk = prepare_verifying_key(&TIER_PARAMS.vk);
    let proof = Proof::read(&zkproof.proof[..]).map_err(|_| ZKPError::SerializationError)?;
    let inputs: Vec<Scalar> = zkproof
        .public_inputs
        .iter()
        .map(|value| public_input_to_scalar(value))
        .collect::<Result<_, _>>()?;

    verify_proof(&pvk, &proof, &inputs).map_err(|_| ZKPError::InvalidProof)
}

fn build_header(kind: CredentialKind, issuer_id: String, subject_ref: String, expires_at: String) -> Result<CredentialHeader, ZKPError> {
    if issuer_secret(&kind, &issuer_id).is_none() {
        return Err(ZKPError::InvalidIssuer(issuer_id));
    }

    let issued_at = Utc::now().date_naive().to_string();
    let credential_id = blake3::hash(
        format!("{}|{}|{}|{}", issuer_id, subject_ref, expires_at, issued_at).as_bytes(),
    )
    .to_hex()
    .to_string();

    let mut header = CredentialHeader {
        credential_id,
        kind,
        issuer_id,
        subject_ref,
        issued_at,
        expires_at,
        status: CredentialStatus::Active,
        signature: String::new(),
    };
    header.signature = sign_header(&header)?;
    Ok(header)
}

fn income_tier_from_amount(amount: u64) -> u8 {
    if amount >= 10_000 {
        3
    } else if amount >= 5_000 {
        2
    } else if amount >= 3_000 {
        1
    } else {
        0
    }
}

fn ensure_active_status(status: &CredentialStatus) -> Result<(), ZKPError> {
    match status {
        CredentialStatus::Active => Ok(()),
        CredentialStatus::Revoked => Err(ZKPError::RevokedCredential),
    }
}

pub fn issue_income_credential(subject_ref: String, actual_income: u64, issuer_id: String, expires_at: String) -> Result<CredentialHeader, ZKPError> {
    let header = build_header(CredentialKind::Income, issuer_id, subject_ref, expires_at)?;
    let credential = StoredIncomeCredential {
        income_tier: income_tier_from_amount(actual_income),
        header: header.clone(),
    };

    let mut store = INCOME_CREDENTIALS
        .lock()
        .map_err(|_| ZKPError::OtherError("failed to lock income credential store".to_string()))?;
    store.insert(header.credential_id.clone(), credential);

    Ok(header)
}

pub fn create_income_presentation(credential_id: &str, required_tier: u8) -> Result<IncomePresentation, ZKPError> {
    let store = INCOME_CREDENTIALS
        .lock()
        .map_err(|_| ZKPError::OtherError("failed to lock income credential store".to_string()))?;
    let credential = store
        .get(credential_id)
        .cloned()
        .ok_or_else(|| ZKPError::NotFound(credential_id.to_string()))?;

    let proof = create_tier_predicate_proof(credential_id, credential.income_tier, required_tier)?;

    Ok(IncomePresentation {
        header: credential.header,
        required_tier,
        proof,
    })
}

pub fn verify_income_presentation(presentation: &IncomePresentation) -> Result<PresentationVerification, ZKPError> {
    let store = INCOME_CREDENTIALS
        .lock()
        .map_err(|_| ZKPError::OtherError("failed to lock income credential store".to_string()))?;
    let stored = store
        .get(&presentation.header.credential_id)
        .cloned()
        .ok_or_else(|| ZKPError::NotFound(presentation.header.credential_id.clone()))?;

    if stored.header.signature != presentation.header.signature {
        return Err(ZKPError::InvalidSignature);
    }

    verify_header_signature(&presentation.header)?;
    ensure_not_expired(&presentation.header.expires_at)?;
    ensure_active_status(&stored.header.status)?;

    let expected_inputs = tier_public_inputs(&presentation.header.credential_id, presentation.required_tier)?;
    if presentation.proof.public_inputs != expected_inputs {
        return Err(ZKPError::InvalidPublicInputs);
    }

    verify_tier_predicate_proof(&presentation.proof)?;

    Ok(PresentationVerification {
        credential_id: presentation.header.credential_id.clone(),
        issuer_id: presentation.header.issuer_id.clone(),
        kind: CredentialKind::Income,
        valid: true,
        message: "income threshold proof verified".to_string(),
        expires_at: presentation.header.expires_at.clone(),
    })
}

pub fn issue_kyc_credential(subject_ref: String, kyc_level: u8, issuer_id: String, expires_at: String) -> Result<CredentialHeader, ZKPError> {
    if kyc_level as usize >= LEVEL_COUNT {
        return Err(ZKPError::UnsupportedPredicate);
    }

    let header = build_header(CredentialKind::Kyc, issuer_id, subject_ref, expires_at)?;
    let credential = StoredKycCredential {
        kyc_level,
        header: header.clone(),
    };

    let mut store = KYC_CREDENTIALS
        .lock()
        .map_err(|_| ZKPError::OtherError("failed to lock kyc credential store".to_string()))?;
    store.insert(header.credential_id.clone(), credential);

    Ok(header)
}

pub fn create_kyc_presentation(credential_id: &str, required_level: u8) -> Result<KycPresentation, ZKPError> {
    let store = KYC_CREDENTIALS
        .lock()
        .map_err(|_| ZKPError::OtherError("failed to lock kyc credential store".to_string()))?;
    let credential = store
        .get(credential_id)
        .cloned()
        .ok_or_else(|| ZKPError::NotFound(credential_id.to_string()))?;

    let proof = create_tier_predicate_proof(credential_id, credential.kyc_level, required_level)?;

    Ok(KycPresentation {
        header: credential.header,
        required_level,
        proof,
    })
}

pub fn verify_kyc_presentation(presentation: &KycPresentation) -> Result<PresentationVerification, ZKPError> {
    let store = KYC_CREDENTIALS
        .lock()
        .map_err(|_| ZKPError::OtherError("failed to lock kyc credential store".to_string()))?;
    let stored = store
        .get(&presentation.header.credential_id)
        .cloned()
        .ok_or_else(|| ZKPError::NotFound(presentation.header.credential_id.clone()))?;

    if stored.header.signature != presentation.header.signature {
        return Err(ZKPError::InvalidSignature);
    }

    verify_header_signature(&presentation.header)?;
    ensure_not_expired(&presentation.header.expires_at)?;
    ensure_active_status(&stored.header.status)?;

    let expected_inputs = tier_public_inputs(&presentation.header.credential_id, presentation.required_level)?;
    if presentation.proof.public_inputs != expected_inputs {
        return Err(ZKPError::InvalidPublicInputs);
    }

    verify_tier_predicate_proof(&presentation.proof)?;

    Ok(PresentationVerification {
        credential_id: presentation.header.credential_id.clone(),
        issuer_id: presentation.header.issuer_id.clone(),
        kind: CredentialKind::Kyc,
        valid: true,
        message: "kyc level proof verified".to_string(),
        expires_at: presentation.header.expires_at.clone(),
    })
}
