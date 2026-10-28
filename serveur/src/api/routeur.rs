use super::etat::Etat;
use crate::{comptes, images, salon, session, sets};
use axum::Router;
use axum::routing::get;
use sqlx::PgPool;

pub fn routeur(bdd: PgPool) -> Router {
    Router::new()
        .merge(comptes::routes::routes())
        .merge(session::routes::routes())
        .merge(sets::routes::routes())
        .merge(images::routes::routes())
        .route("/ws", get(salon::accepter_connecte))
        .route("/ws/partie", get(salon::accepter_connecte))
        .with_state(Etat { bdd })
}
