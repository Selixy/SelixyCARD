use super::{cookie, jeton};
use crate::api::{erreur::ErreurApi, etat::Etat};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::cookie::CookieJar;
use uuid::Uuid;

pub struct CompteCourant {
    pub id: Uuid,
    pub pseudo: String,
}

impl FromRequestParts<Etat> for CompteCourant {
    type Rejection = ErreurApi;

    async fn from_request_parts(parts: &mut Parts, etat: &Etat) -> Result<Self, ErreurApi> {
        let jar = CookieJar::from_headers(&parts.headers);
        let present = jar.get(cookie::NOM).ok_or(ErreurApi::NonConnecte)?;
        let ligne = sqlx::query_as::<_, (Uuid, String)>(
            "select c.id, c.pseudo from sessions s join comptes c on c.id = s.compte_id \
             where s.empreinte = $1 and s.expire_le > now()",
        )
        .bind(jeton::empreinte(present.value()))
        .fetch_optional(&etat.bdd)
        .await?
        .ok_or(ErreurApi::NonConnecte)?;
        Ok(CompteCourant { id: ligne.0, pseudo: ligne.1 })
    }
}
