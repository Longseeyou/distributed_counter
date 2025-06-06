use actix_web::web::{
    self, ServiceConfig,
    post, get, put, delete,
};

use crate::handlers::*;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/message")
            .route("/join", post().to(join_channel))
            .route("/leave", post().to(leave_channel))
            .route("/viewers", get().to(get_viewers))
            
    );
}