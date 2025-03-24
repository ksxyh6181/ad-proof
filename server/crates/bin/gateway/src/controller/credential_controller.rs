use crate::utils::res::{res_custom, res_json_err, res_json_ok, Res, ResObj};
use salvo::prelude::*;
use salvo::{oapi::endpoint, Writer};
use zkp::{verify_zk_proof, Credential, CredentialCircuit, ZKPError, ZKProof, CREDENTIALS};
use std::path::PathBuf;
use std::string::ToString;
use salvo::oapi::extract::JsonBody;
use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use serde_json::json;
use blake3;
use hex;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IssueCredentialRequest {
    pub student_id: String,
    pub name: String,
    pub degree: String,
    pub graduation_date: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IssueCredentialResponse {
    pub credential: String,
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct VerifyCredentialRequest {
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct VerifyCredentialResponse {
    pub message: String,
    pub valid: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GetCredentialRequest {
    pub hash: String,
}

#[endpoint(
    tags("证书"),
    responses(
        (status_code = 200, body = ResObj<IssueCredentialResponse>, description = "颁发证书")
    ),
)]
pub async fn issue(body: JsonBody<IssueCredentialRequest>) -> Res<IssueCredentialResponse> {
    let req = body.into_inner();
    
    // 生成证明
    let circuit = CredentialCircuit::new(
        &req.student_id,
        &req.name,
        &req.degree,
        &req.graduation_date,
    );

    let proof = match circuit.create_proof() {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to generate proof: {}", e);
            return Ok(res_json_err("Failed to generate proof".to_string()));
        }
    };

    let mut credential = Credential::new(
        req.student_id,
        req.name,
        req.degree,
        req.graduation_date,
    );
    credential.proof = Some(proof);
    
    // 存储证书，使用store方法
    let hash = match credential.store() {
        Ok(h) => h,
        Err(e) => {
            log::error!("Failed to store credential: {}", e);
            return Ok(res_json_err(format!("Failed to store credential: {}", e)));
        }
    };

    Ok(res_json_ok(Some(IssueCredentialResponse {
        credential: "证书已颁发".to_string(),
        hash,
    })))
}

#[endpoint(
    tags("证书"),
    responses(
        (status_code = 200, body = ResObj<VerifyCredentialResponse>, description = "验证证书")
    ),
)]
pub async fn verify(body: JsonBody<VerifyCredentialRequest>) -> Res<VerifyCredentialResponse> {
    let req = body.into_inner();
    let hash = req.hash;
    
    // 根据hash获取证书
    let credential = match Credential::get(&hash) {
        Some(c) => c,
        None => {
            log::error!("Credential not found for hash: {}", hash);
            return Ok(res_json_err(format!("Credential not found for hash: {}", hash)));
        }
    };
    
    // 验证证书
    match credential.verify() {
        Ok(true) => {
            log::info!("Credential verified successfully");
            Ok(res_json_ok(Some(VerifyCredentialResponse {
                message: "证书验证成功".to_string(),
                valid: true,
            })))
        },
        Ok(false) => {
            log::warn!("Credential verification failed");
            Ok(res_json_ok(Some(VerifyCredentialResponse {
                message: "证书验证失败".to_string(),
                valid: false,
            })))
        },
        Err(e) => {
            log::error!("Credential verification error: {}", e);
            Ok(res_json_err(format!("Credential verification error: {}", e)))
        }
    }
}

#[endpoint(
    tags("证书"),
    responses(
        (status_code = 200, body = ResObj<Credential>, description = "获取证书")
    ),
)]
pub async fn get(req: &Request, body: JsonBody<GetCredentialRequest>) -> Res<Credential> {
    let req_body = body.into_inner();
    let res  = Credential::get(&req_body.hash).ok_or_else(|| {
        log::error!("Certificate not found: {}", &req_body.hash);
        StatusError::not_found()
    });
    match res  {
        Ok(credential) => {
            // 根据角色过滤字段
            let role: Option<String> = req.header("X-Role");
            let mut filtered_credential = credential.clone();

            // // 只有教育机构可以看到 proof
            // if role.as_deref() != Some("education_institution") {
            //     filtered_credential.proof = None;
            // }

            Ok(res_json_ok(Some(filtered_credential)))
        }
        Err(e) => {

            log::error!("Certificate not found: {}", e);
            Ok(res_json_err("Certificate not found".to_string()))
        }
    }
}