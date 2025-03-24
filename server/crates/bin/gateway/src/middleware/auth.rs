use salvo::prelude::*;
use salvo::http::header::HeaderValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    EducationInstitution,
    Student,
    Employer,
    FinancialInstitution,
}

impl Role {
    pub fn from_str(role: &str) -> Option<Self> {
        match role.to_lowercase().as_str() {
            "education_institution" => Some(Role::EducationInstitution),
            "student" => Some(Role::Student),
            "employer" => Some(Role::Employer),
            "financial_institution" => Some(Role::FinancialInstitution),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct AuthError {
    pub code: u16,
    pub message: String,
}

impl AuthError {
    pub fn unauthorized(msg: &str) -> Self {
        Self {
            code: 401,
            message: msg.to_string(),
        }
    }

    pub fn forbidden(msg: &str) -> Self {
        Self {
            code: 403,
            message: msg.to_string(),
        }
    }
}

#[async_trait]
pub trait RoleChecker: Send + Sync {
    async fn check_role(&self, req: &mut Request, expected_role: Role) -> Result<bool, AuthError>;
}

pub struct HeaderRoleChecker;

#[async_trait]
impl RoleChecker for HeaderRoleChecker {
    async fn check_role(&self, req: &mut Request, expected_role: Role) -> Result<bool, AuthError> {
        let role = req
            .header::<String>("X-Role")
            .ok_or_else(|| AuthError::unauthorized("Missing role header"))
            .and_then(|header| {
                Role::from_str(&header)
                    .ok_or_else(|| AuthError::unauthorized("Invalid role"))
            })?;

        Ok(role == expected_role)
    }
}

pub struct AuthMiddleware {
    required_role: Role,
    role_checker: Box<dyn RoleChecker>,
}

impl AuthMiddleware {
    pub fn new(required_role: Role) -> Self {
        Self {
            required_role,
            role_checker: Box::new(HeaderRoleChecker),
        }
    }
}

#[async_trait]
impl Handler for AuthMiddleware {
    async fn handle(&self, req: &mut Request, _depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        match self.role_checker.check_role(req, self.required_role.clone()).await {
            Ok(true) => {
                // 角色验证通过，继续处理请求
                return;
            }
            Ok(false) => {
                // 角色不匹配
                res.status_code(StatusCode::FORBIDDEN);
                ctrl.skip_rest();
            }
            Err(e) => {
                // 验证过程出错
                res.status_code(StatusCode::from_u16(e.code).unwrap());
                ctrl.skip_rest();
            }
        }
    }
}
