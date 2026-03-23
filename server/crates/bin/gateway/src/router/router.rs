use crate::controller::{system_controller, vc_controller};
use crate::middleware::{AuthMiddleware, Role};
use salvo::Router;

pub fn init_sys_router() -> Router {
    Router::with_path("/sys").push(Router::with_path("/sys_account").get(system_controller::health_check))
}

pub fn init_vc_router() -> Router {
    Router::new()
        .path("/vc")
        .push(
            Router::new()
                .path("/income/issue")
                .hoop(AuthMiddleware::new(Role::FinancialInstitution))
                .post(vc_controller::issue_income),
        )
        .push(Router::new().path("/income/present").post(vc_controller::present_income))
        .push(Router::new().path("/income/verify").post(vc_controller::verify_income))
        .push(
            Router::new()
                .path("/kyc/issue")
                .hoop(AuthMiddleware::new(Role::KycProvider))
                .post(vc_controller::issue_kyc),
        )
        .push(Router::new().path("/kyc/present").post(vc_controller::present_kyc))
        .push(Router::new().path("/kyc/verify").post(vc_controller::verify_kyc))
}
