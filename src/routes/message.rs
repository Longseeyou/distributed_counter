use actix_web::web::{
    self, ServiceConfig,
    post, get, put, delete,
};

use crate::handlers::*;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/message")
            .route("", post().to(produce_message))
            .route("", get().to(get_viewers))
            
    );
}