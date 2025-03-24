use crate::controller::{common_controller, swagger_controller, system_controller};
use crate::router::middware::route_logger;
use salvo::catcher::Catcher;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::logging::Logger;
use salvo::oapi::swagger_ui::SwaggerUi;
use salvo::oapi::OpenApi;
use salvo::prelude::{CatchPanic, SessionHandler};
use salvo::serve_static::StaticDir;
use salvo::session::CookieStore;
use salvo::{Router, Service};

use crate::controller::credential_controller;
use crate::middleware::{AuthMiddleware, Role};
use salvo::prelude::*;

pub mod middware;
pub mod router;

pub fn init_router() -> Router {
    let static_dir = Router::with_path("/static/<*path>").get(StaticDir::new(["static/"]).auto_list(true));

    // 配置 CORS
    let cors = Cors::new()
        .allow_origin("http://127.0.0.1:3000") // 允许前端域名
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::OPTIONS, Method::DELETE])
        .into_handler();

    // 起始路由
    let router = Router::new()
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .hoop(cors) // 添加 CORS 中间件
        .push(static_dir)
        .push(Router::with_path("/actuator/health").get(system_controller::health_check));

    // 业务路由
    let router = router.push(
        Router::with_path("api")
            .push(router::init_sys_router())
            .push(router::init_credential_router())
            .push(router::init_financial_router()),
    );

    let session_handler = SessionHandler::builder(CookieStore::new(), b"salvo-adminsalvo-adminalvo-adminsalvo-admin2023salvo-admin2023salvo-admin2023")
        .build()
        .unwrap();
    let doc = OpenApi::new("教育信息存证信息接口文档", "0.2.1").tags(["教育信息存证系统", "证书"]).merge_router(&router);
    let router = router.push(
        Router::new()
            .hoop(session_handler)
            .push(
                Router::new()
                    .hoop(swagger_controller::auth_token)
                    .push(doc.into_router("/api-doc/openapi.json"))
                    .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui")),
            )
            .push(Router::with_path("/swaggerLogin").post(swagger_controller::swagger_login)),
    );

    router
}

pub fn init_service() -> Service {
    let router = init_router();
    // 捕获异常
    Service::new(router).catcher(Catcher::default().hoop(common_controller::catcher_err)).hoop(route_logger)
    //Service::new(router).catcher(Catcher::default().hoop(common_controller::catcher_err))
    //Service::new(router).hoop(route_logger)
}
