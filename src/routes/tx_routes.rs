use actix_web::web;
use actix_web_lab::middleware::from_fn;

use crate::routes::middlewares;

use super::handlers::tx_handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("secure/tx")
            .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
            .service(tx_handlers::create_tx),
    )
    .service(
        web::scope("/tx")
            .service(tx_handlers::all_txs)
            .service(tx_handlers::one_tx)
            .service(tx_handlers::tx_by_block_id)
            .service(tx_handlers::tx_by_user_id)
    );
}
