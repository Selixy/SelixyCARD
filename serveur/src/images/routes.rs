use super::{depot, verification};
use crate::api::{erreur::ErreurApi, etat::Etat};
use crate::session::courant::CompteCourant;
use axum::body::Bytes;
use axum::extract::rejection::BytesRejection;
use axum::extract::{DefaultBodyLimit, Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::{Value, json};

pub fn routes() -> Router<Etat> {
    Router::new()
        .route(
            "/api/images",
            post(deposer).layer(DefaultBodyLimit::max(verification::TAILLE_MAX + 1)),
        )
        .route("/api/images/{fichier}", get(servir))
}

async fn deposer(
    State(etat): State<Etat>,
    compte: CompteCourant,
    corps: Result<Bytes, BytesRejection>,
) -> Result<(StatusCode, Json<Value>), ErreurApi> {
    let octets = corps.map_err(|_| ErreurApi::TropGros)?;
    verification::verifier(&octets)?;
    let sha256 = verification::sha256(&octets);
    depot::enregistrer(&etat.bdd, &sha256, &octets, compte.id).await?;
    Ok((StatusCode::CREATED, Json(json!({ "image": sha256 }))))
}

async fn servir(State(etat): State<Etat>, Path(fichier): Path<String>) -> Result<Response, ErreurApi> {
    let absente = ErreurApi::Introuvable("Image introuvable.");
    let Some(sha256) = fichier.strip_suffix(".webp").filter(|s| verification::est_sha256(s)) else {
        return Err(absente);
    };
    let octets = depot::lire(&etat.bdd, sha256).await?.ok_or(absente)?;
    let entetes = [
        (header::CONTENT_TYPE, "image/webp"),
        (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
    ];
    Ok((entetes, octets).into_response())
}
