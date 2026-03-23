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

pub mod middware;
pub mod router;

pub fn init_router() -> Router {
    let static_dir = Router::with_path("/static/<*path>").get(StaticDir::new(["static/"]).auto_list(true));

    let cors = Cors::new()
        .allow_origin("http://127.0.0.1:3000")
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::OPTIONS, Method::DELETE])
        .into_handler();

    let base_router = Router::new()
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .hoop(cors)
        .push(static_dir)
        .push(Router::with_path("/actuator/health").get(system_controller::health_check));

    let api_router = base_router.push(Router::with_path("api").push(router::init_sys_router()).push(router::init_vc_router()));

    let session_handler = SessionHandler::builder(
        CookieStore::new(),
        b"salvo-adminsalvo-adminalvo-adminsalvo-admin2023salvo-admin2023salvo-admin2023",
    )
    .build()
    .unwrap();

    let doc = OpenApi::new("Selective Disclosure VC Demo", "0.3.0")
        .tags(["Selective Disclosure VC"])
        .merge_router(&api_router);

    api_router.push(
        Router::new()
            .hoop(session_handler)
            .push(
                Router::new()
                    .hoop(swagger_controller::auth_token)
                    .push(doc.into_router("/api-doc/openapi.json"))
                    .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui")),
            )
            .push(Router::with_path("/swaggerLogin").post(swagger_controller::swagger_login)),
    )
}

pub fn init_service() -> Service {
    let router = init_router();
    Service::new(router)
        .catcher(Catcher::default().hoop(common_controller::catcher_err))
        .hoop(route_logger)
}
