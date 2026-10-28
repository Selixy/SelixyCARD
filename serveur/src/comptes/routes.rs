use super::{depot, mot_de_passe, validation};
use crate::api::{erreur::ErreurApi, etat::Etat};
use crate::session::ouverture;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct Identifiants {
    pub pseudo: String,
    pub mot_de_passe: String,
}

pub fn routes() -> Router<Etat> {
    Router::new().route("/api/comptes", post(inscrire))
}

async fn inscrire(
    State(etat): State<Etat>,
    jar: CookieJar,
    Json(corps): Json<Identifiants>,
) -> Result<(CookieJar, Json<Value>), ErreurApi> {
    let pseudo = corps.pseudo.trim().to_string();
    validation::verifier_pseudo(&pseudo)?;
    validation::verifier_mot_de_passe(&corps.mot_de_passe)?;
    let empreinte = mot_de_passe::hacher(corps.mot_de_passe).await.map_err(|_| ErreurApi::Interne)?;
    let id = depot::creer(&etat.bdd, &pseudo, &empreinte).await?.ok_or(ErreurApi::PseudoPris)?;
    let jar = ouverture::ouvrir(&etat.bdd, jar, id).await?;
    Ok((jar, Json(json!({ "pseudo": pseudo }))))
}
