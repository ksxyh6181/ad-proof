use bellman::{
    groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, Proof},
    Circuit, ConstraintSystem, SynthesisError,
};
use blake3;
use bls12_381;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Once;

use bls12_381::Bls12;
use hex;
use log;
use once_cell::sync::Lazy;
use salvo::oapi::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub enum ZKPError {
    SerializationError,
    InvalidProof,
    InvalidPublicInputs,
    SynthesisError(String),
    HashError(String),
    NoProof,
    InvalidConstraints,
    OtherError(String),
}

impl fmt::Display for ZKPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZKPError::SerializationError => write!(f, "Serialization error"),
            ZKPError::InvalidProof => write!(f, "Invalid proof"),
            ZKPError::InvalidPublicInputs => write!(f, "Invalid public inputs"),
            ZKPError::SynthesisError(msg) => write!(f, "Synthesis error: {}", msg),
            ZKPError::HashError(msg) => write!(f, "Hash error: {}", msg),
            ZKPError::NoProof => write!(f, "NoProof inputs"),
            ZKPError::InvalidConstraints => write!(f, "电路约束未满足"),
            ZKPError::OtherError(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl From<SynthesisError> for ZKPError {
    fn from(e: SynthesisError) -> Self {
        ZKPError::SynthesisError(e.to_string())
    }
}

#[derive(Clone)]
pub struct CredentialCircuit {
    pub student_id: Option<String>,
    pub name: Option<String>,
    pub degree: Option<String>,
    pub graduation_date: Option<String>,
}

pub(crate) fn hash_to_field(input: &str) -> Result<bls12_381::Scalar, ZKPError> {
    // 使用blake3进行哈希
    let hash = blake3::hash(input.as_bytes());
    let hash_bytes = hash.as_bytes();
    
    // 构建64字节的输入数组，用于from_bytes_wide
    let mut repr = [0u8; 64];
    
    // 仅复制前32个字节，保持后32个字节为0
    // 这有助于确保生成的标量不会太接近曲线阶，提高数值稳定性
    repr[0..32].copy_from_slice(hash_bytes);
    
    // 记录哈希和生成的标量的十六进制表示以便调试
    let scalar = bls12_381::Scalar::from_bytes_wide(&repr);
    let scalar_bytes = scalar.to_bytes();
    
    log::debug!("哈希到字段: input='{}', hash=0x{}, scalar_bytes=0x{}", 
               input, 
               hex::encode(hash_bytes),
               hex::encode(&scalar_bytes));
    
    Ok(scalar)
}

impl Circuit<bls12_381::Scalar> for CredentialCircuit {
    fn synthesize<CS: ConstraintSystem<bls12_381::Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let combined = format!(
            "{}{}{}{}",
            self.student_id.as_deref().unwrap_or(""),
            self.name.as_deref().unwrap_or(""),
            self.degree.as_deref().unwrap_or(""),
            self.graduation_date.as_deref().unwrap_or("")
        );

        let combined_hash = hash_to_field(&combined).map_err(|_| SynthesisError::Unsatisfiable)?;

        let combined_var = cs.alloc(|| "combined hash", || Ok(combined_hash))?;

        let public_var = cs.alloc_input(|| "public input", || Ok(combined_hash))?;

        cs.enforce(|| "combined hash constraint", |lc| lc + combined_var, |lc| lc + CS::one(), |lc| lc + public_var);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ZKProof {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<String>,
}

pub(crate) static PARAMS: Lazy<bellman::groth16::Parameters<Bls12>> = Lazy::new(|| {
    static INIT: Once = Once::new();
    let mut params = None;
    INIT.call_once(|| {
        let rng = &mut thread_rng();
        let circuit = CredentialCircuit {
            student_id: Some("test_id".to_string()),
            name: Some("test_name".to_string()),
            degree: Some("test_degree".to_string()),
            graduation_date: Some("test_date".to_string()),
        };
        params = Some(generate_random_parameters::<Bls12, _, _>(circuit, rng).expect("Failed to generate parameters"));
    });
    params.unwrap()
});

impl CredentialCircuit {
    fn check_undergraduate_degree(degree: &str) -> bool {
        degree.to_lowercase().contains("bachelor") || degree.to_lowercase().contains("undergraduate")
    }

    pub fn new(student_id: &str, name: &str, degree: &str, graduation_date: &str) -> Self {
        Self {
            student_id: Some(student_id.to_string()),
            name: Some(name.to_string()),
            degree: Some(degree.to_string()),
            graduation_date: Some(graduation_date.to_string()),
        }
    }

    pub fn create_proof(&self) -> Result<ZKProof, ZKPError> {
        let rng = &mut thread_rng();

        let combined = format!(
            "{}{}{}{}",
            self.student_id.as_deref().unwrap_or(""),
            self.name.as_deref().unwrap_or(""),
            self.degree.as_deref().unwrap_or(""),
            self.graduation_date.as_deref().unwrap_or("")
        );

        let combined_hash = hash_to_field(&combined).map_err(|_| ZKPError::HashError("".into()))?;

        println!("生成证明时的公共输入: {:?}", combined_hash);

        let params: &bellman::groth16::Parameters<Bls12> = &PARAMS;
        let proof = create_random_proof(self.clone(), params, rng)?;

        let mut proof_bytes = Vec::new();
        proof.write(&mut proof_bytes).map_err(|_| ZKPError::SerializationError)?;

        Ok(ZKProof {
            proof: proof_bytes,
            public_inputs: vec![combined],
        })
    }


}

pub fn verify_zk_proof(zkproof: &ZKProof) -> Result<(), ZKPError> {
    println!("开始验证证明...");
    println!("公共输入: {:?}", zkproof.public_inputs);

    let pvk = prepare_verifying_key(&PARAMS.vk);

    let proof = match Proof::read(&zkproof.proof[..]) {
        Ok(p) => p,
        Err(e) => {
            println!("证明反序列化失败: {:?}", e);
            return Err(ZKPError::SerializationError);
        }
    };

    let inputs: Vec<bls12_381::Scalar> = match zkproof.public_inputs.iter().map(|s| hash_to_field(s)).collect::<Result<_, _>>() {
        Ok(i) => {
            println!("成功转换公共输入");
            i
        }
        Err(e) => {
            println!("公共输入转换失败: {:?}", e);
            return Err(e);
        }
    };

    println!("开始验证...");
    if bellman::groth16::verify_proof(&pvk, &proof, &inputs).is_ok() {
        // New degree verification
        println!("验证成功!");
        Ok(())
    } else {
        return Err(ZKPError::InvalidProof);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_full_zkp_flow() {
//         let _ = env_logger::try_init();
//
//         let circuit = CredentialCircuit::new("2024001", "张三", "计算机科学学士", "2024-06-30");
//
//         let proof = circuit.create_proof().expect("生成证明失败");
//
//         let verification_result = verify_zk_proof(&proof);
//         assert!(verification_result.is_ok(), "验证失败: {:?}", verification_result.err().unwrap());
//
//         let mut invalid_proof = proof.clone();
//         invalid_proof.public_inputs[0] = "invalid_hash".to_string();
//         let invalid_result = verify_zk_proof(&invalid_proof);
//         assert!(invalid_result.is_err(), "错误证明应该验证失败");
//     }
// }
