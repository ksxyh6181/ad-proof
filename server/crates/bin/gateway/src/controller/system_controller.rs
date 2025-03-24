use crate::utils::res::{res_json_ok, Res};

use salvo::prelude::*;
use salvo::{oapi::endpoint, Writer};

/// 系统状态测试
#[endpoint(
    tags("系统"),
    responses(
    (status_code = 200,description ="系统状态测试")
    ),
)]
pub async fn health_check() -> Res<String> {
    Ok(res_json_ok(Some("Hello World".to_string())))
}
