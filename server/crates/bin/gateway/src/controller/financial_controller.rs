use crate::utils::res::{res_custom, res_json_err, res_json_ok, Res, ResObj};
use salvo::prelude::*;
use salvo::{oapi::endpoint, Writer};
use zkp::{FinancialCredential, ZKPError, ZKProof, FINANCIAL_CREDENTIALS};
use std::path::PathBuf;
use std::string::ToString;
use salvo::oapi::extract::JsonBody;
use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use serde_json::json;

/// 请求结构体定义
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IssueIncomeProofRequest {
    pub personal_id: String,
    pub actual_income: f64,
    pub issuer_id: String,
    pub expiry_date: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IssueCreditScoreRequest {
    pub personal_id: String,
    pub credit_score: u16,
    pub issuer_id: String,
    pub expiry_date: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IssueCrossBorderRequest {
    pub personal_id: String,
    pub income_level: String,
    pub credit_score_range: String,
    pub issuer_id: String,
    pub expiry_date: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IssueFinancialCredentialResponse {
    pub credential_type: String,
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct VerifyFinancialCredentialRequest {
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct VerifyFinancialCredentialResponse {
    pub message: String,
    pub valid: bool,
    pub credential_type: String,
    pub issuer_id: String,
    pub expiry_date: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GetFinancialCredentialRequest {
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ListFinancialCredentialsRequest {
    pub personal_id: Option<String>,
}

#[endpoint(
    tags("金融凭证"),
    responses(
        (status_code = 200, body = ResObj<IssueFinancialCredentialResponse>, description = "颁发收入凭证")
    ),
)]
pub async fn issue_income(body: JsonBody<IssueIncomeProofRequest>) -> Res<IssueFinancialCredentialResponse> {
    let req = body.into_inner();
    
    // 创建收入凭证
    let mut credential = FinancialCredential::for_income(
        req.personal_id,
        req.actual_income,
        req.issuer_id,
        req.expiry_date,
    );

    log::warn!("issue_income:{:?}", credential.personal_id.clone());
    // 添加证明
    if let Err(e) = credential.add_proof() {
        log::error!("无法生成证明: {:?}", e);
        return Ok(res_json_err(format!("无法生成证明: {:?}", e)));
    }

    // 存储凭证
    match credential.save() {
        Ok(hash) => {
            log::info!("成功存储income凭证，hash={}", hash);
            Ok(res_json_ok(Some(IssueFinancialCredentialResponse {
                credential_type: "income".to_string(),
                hash,
            })))
        },
        Err(e) => {
            log::error!("存储income凭证失败: {}", e);
            Ok(res_json_err(format!("存储income凭证失败: {}", e)))
        }
    }
}

#[endpoint(
    tags("金融凭证"),
    responses(
        (status_code = 200, body = ResObj<IssueFinancialCredentialResponse>, description = "颁发信用评分凭证")
    ),
)]
pub async fn issue_credit_score(body: JsonBody<IssueCreditScoreRequest>) -> Res<IssueFinancialCredentialResponse> {
    let req = body.into_inner();
    
    // 创建信用评分凭证
    let mut credential = FinancialCredential::for_credit_score(
        req.personal_id,
        req.credit_score,
        req.issuer_id,
        req.expiry_date,
    );
    
    // 添加证明
    if let Err(e) = credential.add_proof() {
        log::error!("无法生成证明: {:?}", e);
        return Ok(res_json_err(format!("无法生成证明: {:?}", e)));
    }

    // 存储凭证
    match credential.save() {
        Ok(hash) => {
            log::info!("成功存储credit凭证，hash={}", hash);
            Ok(res_json_ok(Some(IssueFinancialCredentialResponse {
                credential_type: "credit".to_string(),
                hash,
            })))
        },
        Err(e) => {
            log::error!("存储credit凭证失败: {}", e);
            Ok(res_json_err(format!("存储credit凭证失败: {}", e)))
        }
    }
}

#[endpoint(
    tags("金融凭证"),
    responses(
        (status_code = 200, body = ResObj<IssueFinancialCredentialResponse>, description = "颁发跨境信用凭证")
    ),
)]
pub async fn issue_cross_border(body: JsonBody<IssueCrossBorderRequest>) -> Res<IssueFinancialCredentialResponse> {
    let req = body.into_inner();
    
    // 创建跨境信用凭证
    let mut credential = FinancialCredential::for_cross_border(
        req.personal_id,
        req.income_level,
        req.credit_score_range,
        req.issuer_id,
        req.expiry_date,
    );
    
    // 添加证明
    if let Err(e) = credential.add_proof() {
        log::error!("无法生成证明: {:?}", e);
        return Ok(res_json_err(format!("无法生成证明: {:?}", e)));
    }

    // 存储凭证
    match credential.save() {
        Ok(hash) => {
            log::info!("成功存储cross_border凭证，hash={}", hash);
            Ok(res_json_ok(Some(IssueFinancialCredentialResponse {
                credential_type: "cross_border".to_string(),
                hash,
            })))
        },
        Err(e) => {
            log::error!("存储cross_border凭证失败: {}", e);
            Ok(res_json_err(format!("存储cross_border凭证失败: {}", e)))
        }
    }
}

#[endpoint(
    tags("金融凭证"),
    responses(
        (status_code = 200, body = ResObj<IssueFinancialCredentialResponse>, description = "验证金融凭证")
    ),
)]
pub async fn verify(body: JsonBody<VerifyFinancialCredentialRequest>) -> Res<VerifyFinancialCredentialResponse> {
    let req = body.into_inner();
    log::info!("【深度调试】开始验证金融凭证 Hash: {}", req.hash);
    
    // 获取凭证
    let credential = match FinancialCredential::get(&req.hash) {
        Some(c) => c,
        None => {
            log::error!("【深度调试】无法找到凭证 Hash: {}", req.hash);
            return Ok(res_json_err("未找到凭证".to_string()));
        }
    };
    
    // 记录找到的凭证信息
    log::info!("【深度调试】找到凭证: type={}, hash={}, personal_id={}", 
        credential.credential_type, 
        credential.hash(), 
        credential.personal_id
    );
    
    // 验证证明
    match credential.verify() {
        Ok(true) => {
            log::info!("【深度调试】凭证验证成功");
            Ok(res_json_ok(Some(VerifyFinancialCredentialResponse {
                message: "凭证验证成功".to_string(),
                valid: true,
                credential_type: credential.credential_type.clone(),
                issuer_id: credential.issuer_id.clone(),
                expiry_date: credential.expiry_date.clone(),
            })))
        },
        Ok(false) => {
            log::error!("【深度调试】凭证验证失败");
            Ok(res_json_err("凭证验证失败".to_string()))
        },
        Err(e) => {
            log::error!("【深度调试】凭证验证错误: {:?}", e);
            Ok(res_json_err(format!("凭证验证错误: {}", e)))
        }
    }
}

#[endpoint(
    tags("金融凭证"),
    responses(
        (status_code = 200, body = ResObj<FinancialCredential>, description = "获取金融凭证")
    ),
)]
pub async fn get_credential(body: JsonBody<GetFinancialCredentialRequest>) -> Res<FinancialCredential> {
    let req = body.into_inner();
    let hash = req.hash;

    // 获取凭证
    match FinancialCredential::get(&hash) {
        Some(credential) => Ok(res_json_ok(Some(credential))),
        None => {
            log::error!("Credential not found");
            Ok(res_json_err("Credential not found".to_string()))
        }
    }
}

#[endpoint(
    tags("金融凭证"),
    responses(
        (status_code = 200, body = ResObj<Vec<FinancialCredential>>, description = "获取所有金融凭证")
    ),
)]
pub async fn list_credentials(body: JsonBody<ListFinancialCredentialsRequest>) -> Res<Vec<FinancialCredential>> {
    let req = body.into_inner();
    let personal_id = req.personal_id;
    
    // 获取所有凭证
    let credentials = match FinancialCredential::list(personal_id) {
        Some(credentials) => credentials,
        None => {
            log::error!("No credentials found");
            return Ok(res_json_err("No credentials found".to_string()));
        }
    };
    
    Ok(res_json_ok(Some(credentials)))
}
