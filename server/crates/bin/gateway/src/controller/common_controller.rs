use salvo::{handler, prelude::StatusCode, FlowCtrl, Request, Response};
use tracing::{error, warn};

use crate::utils::res::res_json_custom;


#[handler]
pub async fn catcher_err(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl) {
    // 记录请求基本信息
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let client_ip = req.remote_addr().to_string();

    // 仅处理错误状态码
    if let Some(status_code) = res.status_code {
        match status_code {
            StatusCode::NOT_FOUND => handle_not_found(req, res, ctrl).await,
            StatusCode::INTERNAL_SERVER_ERROR => handle_server_error(res, ctrl).await,
            _ => handle_other_errors(req, res, status_code, ctrl).await,
        }
    } else {
        // 记录未处理的成功请求（可选）
        warn!("请求成功但未记录 | 方法: {} | 路径: {} | 客户端IP: {}", method, path, client_ip);
    }
}

async fn handle_not_found(req: &Request, res: &mut Response, ctrl: &mut FlowCtrl) {
    ctrl.skip_rest();

    // 收集请求参数
    let params = req.params().iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(", ");

    // 记录详细错误信息
    error!(
        "未找到接口 | 路径: {} | 方法: {} | 参数: {} | 客户端IP: {}",
        req.uri().path(),
        req.method(),
        params,
        req.remote_addr()
    );

    res.render(res_json_custom::<()>(404, "请求的资源不存在".to_string()));
}

async fn handle_server_error(res: &mut Response, ctrl: &mut FlowCtrl) {
    ctrl.skip_rest();
    error!("服务器内部错误: {:?}", res.to_string());

    res.render(res_json_custom::<()>(500, "服务器内部错误，请稍后重试".to_string()));
}

async fn handle_other_errors(req: &Request, res: &mut Response, code: StatusCode, ctrl: &mut FlowCtrl) {
    ctrl.skip_rest();
    let status_code = code.as_u16();

    // 收集请求参数
    let params = req.params().iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(", ");

    // 记录详细错误信息
    error!(
        "其他错误, 路径: {} | 方法: {} | 参数: {} | 客户端IP: {},错误码: {}",
        req.uri().path(),
        req.method(),
        params,
        req.remote_addr(),
        code
    );
    res.render(res_json_custom::<()>(status_code as i32, format!("请求处理失败 (状态码: {})", status_code)));
}
