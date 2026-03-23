use crate::utils::res::{res_json_err, res_json_ok, Res, ResObj};
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use salvo::oapi::ToSchema;
use salvo::Writer;
use serde::{Deserialize, Serialize};
use zkp::{
    create_income_presentation, create_kyc_presentation, issue_income_credential, issue_kyc_credential,
    verify_income_presentation, verify_kyc_presentation, CredentialHeader, IncomePresentation, KycPresentation,
    PresentationVerification,
};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IssueIncomeCredentialRequest {
    pub subject_ref: String,
    pub actual_income: u64,
    pub issuer_id: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IssueKycCredentialRequest {
    pub subject_ref: String,
    pub kyc_level: u8,
    pub issuer_id: String,
    pub expires_at: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateIncomePresentationRequest {
    pub credential_id: String,
    pub required_tier: u8,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateKycPresentationRequest {
    pub credential_id: String,
    pub required_level: u8,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct VerifyIncomePresentationRequest {
    pub presentation: IncomePresentation,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct VerifyKycPresentationRequest {
    pub presentation: KycPresentation,
}

#[endpoint(
    tags("Selective Disclosure VC"),
    responses((status_code = 200, body = ResObj<CredentialHeader>, description = "Issue an income credential"))
)]
pub async fn issue_income(body: JsonBody<IssueIncomeCredentialRequest>) -> Res<CredentialHeader> {
    let req = body.into_inner();
    match issue_income_credential(req.subject_ref, req.actual_income, req.issuer_id, req.expires_at) {
        Ok(header) => Ok(res_json_ok(Some(header))),
        Err(err) => Ok(res_json_err(err.to_string())),
    }
}

#[endpoint(
    tags("Selective Disclosure VC"),
    responses((status_code = 200, body = ResObj<CredentialHeader>, description = "Issue a KYC credential"))
)]
pub async fn issue_kyc(body: JsonBody<IssueKycCredentialRequest>) -> Res<CredentialHeader> {
    let req = body.into_inner();
    match issue_kyc_credential(req.subject_ref, req.kyc_level, req.issuer_id, req.expires_at) {
        Ok(header) => Ok(res_json_ok(Some(header))),
        Err(err) => Ok(res_json_err(err.to_string())),
    }
}

#[endpoint(
    tags("Selective Disclosure VC"),
    responses((status_code = 200, body = ResObj<IncomePresentation>, description = "Create an income threshold presentation"))
)]
pub async fn present_income(body: JsonBody<CreateIncomePresentationRequest>) -> Res<IncomePresentation> {
    let req = body.into_inner();
    match create_income_presentation(&req.credential_id, req.required_tier) {
        Ok(presentation) => Ok(res_json_ok(Some(presentation))),
        Err(err) => Ok(res_json_err(err.to_string())),
    }
}

#[endpoint(
    tags("Selective Disclosure VC"),
    responses((status_code = 200, body = ResObj<KycPresentation>, description = "Create a KYC level presentation"))
)]
pub async fn present_kyc(body: JsonBody<CreateKycPresentationRequest>) -> Res<KycPresentation> {
    let req = body.into_inner();
    match create_kyc_presentation(&req.credential_id, req.required_level) {
        Ok(presentation) => Ok(res_json_ok(Some(presentation))),
        Err(err) => Ok(res_json_err(err.to_string())),
    }
}

#[endpoint(
    tags("Selective Disclosure VC"),
    responses((status_code = 200, body = ResObj<PresentationVerification>, description = "Verify an income threshold presentation"))
)]
pub async fn verify_income(body: JsonBody<VerifyIncomePresentationRequest>) -> Res<PresentationVerification> {
    let req = body.into_inner();
    match verify_income_presentation(&req.presentation) {
        Ok(result) => Ok(res_json_ok(Some(result))),
        Err(err) => Ok(res_json_err(err.to_string())),
    }
}

#[endpoint(
    tags("Selective Disclosure VC"),
    responses((status_code = 200, body = ResObj<PresentationVerification>, description = "Verify a KYC level presentation"))
)]
pub async fn verify_kyc(body: JsonBody<VerifyKycPresentationRequest>) -> Res<PresentationVerification> {
    let req = body.into_inner();
    match verify_kyc_presentation(&req.presentation) {
        Ok(result) => Ok(res_json_ok(Some(result))),
        Err(err) => Ok(res_json_err(err.to_string())),
    }
}
