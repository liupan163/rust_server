use actix_web::web;
use actix_web_lab::middleware::from_fn;

use crate::routes::middlewares;

use super::handlers::block_handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("secure/block")
            .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
            .service(block_handlers::create_block),
    )
    .service(
        web::scope("/block")
            .service(block_handlers::all_blocks)
            .service(block_handlers::one_block),
    );
}
