use super::acces::{exiger_auteur, lire_set};
use super::modele::{Set, SetEntrant};
use super::{depot_ecriture, validation};
use crate::api::{erreur::ErreurApi, etat::Etat};
use crate::session::courant::CompteCourant;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::put;
use axum::{Json, Router};
use uuid::Uuid;

pub fn routes() -> Router<Etat> {
    Router::new().route("/api/sets/{id}", put(remplacer).delete(supprimer))
}

async fn remplacer(
    State(etat): State<Etat>,
    compte: CompteCourant,
    Path(id): Path<Uuid>,
    Json(entrant): Json<SetEntrant>,
) -> Result<Json<Set>, ErreurApi> {
    exiger_auteur(&etat, &compte, id).await?;
    validation::verifier_set(&etat.bdd, &entrant).await?;
    depot_ecriture::remplacer(&etat.bdd, id, &entrant).await?;
    Ok(Json(lire_set(&etat, id).await?))
}

async fn supprimer(
    State(etat): State<Etat>,
    compte: CompteCourant,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ErreurApi> {
    exiger_auteur(&etat, &compte, id).await?;
    depot_ecriture::supprimer(&etat.bdd, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
