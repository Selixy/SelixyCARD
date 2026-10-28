use super::{courant::CompteCourant, ouverture};
use crate::api::{erreur::ErreurApi, etat::Etat};
use crate::comptes::{depot, mot_de_passe, routes::Identifiants};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::cookie::CookieJar;
use serde_json::{Value, json};

pub fn routes() -> Router<Etat> {
    Router::new()
        .route("/api/session", post(connecter).delete(deconnecter))
        .route("/api/moi", get(moi))
}

async fn connecter(
    State(etat): State<Etat>,
    jar: CookieJar,
    Json(corps): Json<Identifiants>,
) -> Result<(CookieJar, Json<Value>), ErreurApi> {
    let compte = depot::par_pseudo(&etat.bdd, corps.pseudo.trim()).await?;
    let empreinte = compte.as_ref().map(|c| c.mot_de_passe.clone());
    if !mot_de_passe::verifier(corps.mot_de_passe, empreinte).await {
        return Err(ErreurApi::Identifiants);
    }
    let compte = compte.ok_or(ErreurApi::Identifiants)?;
    let jar = ouverture::ouvrir(&etat.bdd, jar, compte.id).await?;
    Ok((jar, Json(json!({ "pseudo": compte.pseudo }))))
}

async fn deconnecter(State(etat): State<Etat>, jar: CookieJar) -> Result<(CookieJar, StatusCode), ErreurApi> {
    let jar = ouverture::fermer(&etat.bdd, jar).await?;
    Ok((jar, StatusCode::NO_CONTENT))
}

async fn moi(compte: CompteCourant) -> Json<Value> {
    Json(json!({ "pseudo": compte.pseudo }))
}
