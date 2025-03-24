use crate::controller::{system_controller, credential_controller, financial_controller};
use salvo::Router;
use crate::controller::credential_controller::VerifyCredentialRequest;
use crate::middleware::{AuthMiddleware, Role};

pub fn init_sys_router() -> Router {
    let router = Router::with_path("/sys");
    router.push(Router::with_path("/sys_account").get(system_controller::health_check))
}

pub fn init_credential_router() -> Router {
    Router::new()
        .path("/credential")
        .push(
            Router::new()
                .path("/issue")
                .hoop(AuthMiddleware::new(Role::EducationInstitution))
                .post(credential_controller::issue)
        )
        .push(
            Router::new()
                .path("/verify")
                .post(credential_controller::verify)
        )
        .push(
            Router::new()
                .path("/get")
                .post(credential_controller::get)
        )
}

pub fn init_financial_router() -> Router {
    Router::new()
        .path("/financial")
        .push(
            Router::new()
                .path("/income")
                .hoop(AuthMiddleware::new(Role::FinancialInstitution))
                .post(financial_controller::issue_income)
        )
        .push(
            Router::new()
                .path("/credit")
                .hoop(AuthMiddleware::new(Role::FinancialInstitution))
                .post(financial_controller::issue_credit_score)
        )
        .push(
            Router::new()
                .path("/cross_border")
                .hoop(AuthMiddleware::new(Role::FinancialInstitution))
                .post(financial_controller::issue_cross_border)
        )
        .push(
            Router::new()
                .path("/verify")
                .post(financial_controller::verify)
        )
        .push(
            Router::new()
                .path("/get")
                .post(financial_controller::get_credential)
        )
        .push(
            Router::new()
                .path("/list")
                .post(financial_controller::list_credentials)
        )
}