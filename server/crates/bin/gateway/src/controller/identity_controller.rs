use crate::utils::res::{res_json_err, res_json_ok, Res, ResObj};
use salvo::prelude::*;
use salvo::oapi::extract::JsonBody;
use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use zkp::identity::{AgentCircuit, PersonhoodCircuit, verify_agent_proof};
use zkp::{ZKProof, ZKPError};
use uuid::Uuid;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// In-memory registry for demo purposes
static AGENT_REGISTRY: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateIdentityRequest {
    pub personal_id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateIdentityResponse {
    pub personhood_hash: String,
    pub proof: ZKProof,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterAgentRequest {
    pub owner_personhood_hash: String,
    pub agent_pubkey: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterAgentResponse {
    pub agent_binding_hash: String,
    pub proof: ZKProof,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct VerifyAgentRequest {
    pub proof: ZKProof,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct VerifyAgentResponse {
    pub message: String,
    pub valid: bool,
}

#[endpoint(
    tags("AI数字身份"),
    responses(
        (status_code = 200, body = ResObj<CreateIdentityResponse>, description = "创建个人数字身份(Personhood)")
    )
)]
pub async fn create_identity(body: JsonBody<CreateIdentityRequest>) -> Res<CreateIdentityResponse> {
    let req = body.into_inner();
    let salt = Uuid::new_v4().to_string(); // Generate a random salt
    
    let circuit = PersonhoodCircuit::new(&req.personal_id, &salt);
    
    let proof = match circuit.create_proof() {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to generate personhood proof: {}", e);
            return Ok(res_json_err("Failed to generate proof".to_string()));
        }
    };

    // The public input contains the hash representing the identity
    let personhood_hash = proof.public_inputs[0].clone();

    Ok(res_json_ok(Some(CreateIdentityResponse {
        personhood_hash,
        proof,
    })))
}

#[endpoint(
    tags("AI数字身份"),
    responses(
        (status_code = 200, body = ResObj<RegisterAgentResponse>, description = "注册AI Agent并绑定数字身份")
    )
)]
pub async fn register_agent(body: JsonBody<RegisterAgentRequest>) -> Res<RegisterAgentResponse> {
    let req = body.into_inner();
    
    let circuit = AgentCircuit::new(&req.owner_personhood_hash, &req.agent_pubkey);
    
    let proof = match circuit.create_proof() {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to generate agent binding proof: {}", e);
            return Ok(res_json_err("Failed to generate agent proof".to_string()));
        }
    };

    let agent_binding_hash = proof.public_inputs[0].clone();

    // Store in mock registry
    if let Ok(mut registry) = AGENT_REGISTRY.lock() {
        registry.insert(agent_binding_hash.clone(), req.agent_pubkey.clone());
    }

    Ok(res_json_ok(Some(RegisterAgentResponse {
        agent_binding_hash,
        proof,
    })))
}

#[endpoint(
    tags("AI数字身份"),
    responses(
        (status_code = 200, body = ResObj<VerifyAgentResponse>, description = "验证AI Agent身份")
    )
)]
pub async fn verify_agent(body: JsonBody<VerifyAgentRequest>) -> Res<VerifyAgentResponse> {
    let req = body.into_inner();
    
    match verify_agent_proof(&req.proof) {
        Ok(_) => {
            Ok(res_json_ok(Some(VerifyAgentResponse {
                message: "AI Agent 验证成功".to_string(),
                valid: true,
            })))
        },
        Err(e) => {
            log::warn!("Agent verification failed: {}", e);
            Ok(res_json_ok(Some(VerifyAgentResponse {
                message: format!("AI Agent 验证失败: {}", e),
                valid: false,
            })))
        }
    }
}
