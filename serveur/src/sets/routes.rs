use super::acces::{exiger_auteur, lire_set};
use super::modele::{NouveauSet, Resume, Set};
use super::{depot_ecriture, depot_lecture, routes_modification, validation};
use crate::api::{erreur::ErreurApi, etat::Etat};
use crate::session::courant::CompteCourant;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;

pub fn routes() -> Router<Etat> {
    Router::new()
        .route("/api/sets", get(lister).post(creer))
        .route("/api/sets/{id}", get(lire))
        .merge(routes_modification::routes())
}

async fn lister(
    State(etat): State<Etat>,
    compte: CompteCourant,
) -> Result<Json<Vec<Resume>>, ErreurApi> {
    Ok(Json(depot_lecture::lister(&etat.bdd, compte.id).await?))
}

async fn creer(
    State(etat): State<Etat>,
    compte: CompteCourant,
    Json(corps): Json<NouveauSet>,
) -> Result<(StatusCode, Json<Set>), ErreurApi> {
    validation::verifier_nom(&corps.nom)?;
    let id = depot_ecriture::creer(&etat.bdd, compte.id, corps.nom.trim()).await?;
    Ok((StatusCode::CREATED, Json(lire_set(&etat, id).await?)))
}

async fn lire(
    State(etat): State<Etat>,
    compte: CompteCourant,
    Path(id): Path<Uuid>,
) -> Result<Json<Set>, ErreurApi> {
    exiger_auteur(&etat, &compte, id).await?;
    Ok(Json(lire_set(&etat, id).await?))
}
