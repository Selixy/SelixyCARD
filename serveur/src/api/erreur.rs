use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub enum ErreurApi {
    Invalide(&'static str),
    PseudoPris,
    Identifiants,
    NonConnecte,
    Interdit,
    Introuvable(&'static str),
    TropGros,
    Interne,
}

impl ErreurApi {
    fn decrire(&self) -> (StatusCode, &'static str) {
        match self {
            Self::Invalide(message) => (StatusCode::BAD_REQUEST, message),
            Self::PseudoPris => (StatusCode::CONFLICT, "Ce pseudo est déjà pris."),
            Self::Identifiants => (StatusCode::UNAUTHORIZED, "Pseudo ou mot de passe incorrect."),
            Self::NonConnecte => (StatusCode::UNAUTHORIZED, "Connexion requise."),
            Self::Interdit => (StatusCode::FORBIDDEN, "Seul l'auteur peut modifier ce set."),
            Self::Introuvable(message) => (StatusCode::NOT_FOUND, message),
            Self::TropGros => (StatusCode::PAYLOAD_TOO_LARGE, "L'image dépasse 2 Mo."),
            Self::Interne => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne du serveur."),
        }
    }
}

impl From<sqlx::Error> for ErreurApi {
    fn from(erreur: sqlx::Error) -> Self {
        eprintln!("erreur base : {erreur}");
        Self::Interne
    }
}

impl IntoResponse for ErreurApi {
    fn into_response(self) -> Response {
        let (statut, message) = self.decrire();
        (statut, Json(json!({ "erreur": message }))).into_response()
    }
}
