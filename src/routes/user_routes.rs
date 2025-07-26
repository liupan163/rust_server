use actix_web::web;
use actix_web_lab::middleware::from_fn;

use super::handlers::user_handlers;
use super::middlewares::auth_middleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(user_handlers::register))
        .service(
            web::scope("secure/user")
                .wrap(from_fn(auth_middleware::check_auth_middleware))
                .service(user_handlers::my_info)
                .service(user_handlers::update_user_info),
        );
}
