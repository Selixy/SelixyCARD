use super::{cookie, jeton};
use crate::api::erreur::ErreurApi;
use axum_extra::extract::cookie::CookieJar;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn ouvrir(bdd: &PgPool, jar: CookieJar, compte_id: Uuid) -> Result<CookieJar, ErreurApi> {
    let valeur = jeton::nouveau().map_err(|_| ErreurApi::Interne)?;
    sqlx::query(
        "insert into sessions (empreinte, compte_id, expire_le) values ($1, $2, now() + make_interval(days => $3))",
    )
    .bind(jeton::empreinte(&valeur))
    .bind(compte_id)
    .bind(cookie::JOURS as i32)
    .execute(bdd)
    .await?;
    Ok(jar.add(cookie::creer(valeur)))
}

pub async fn fermer(bdd: &PgPool, jar: CookieJar) -> Result<CookieJar, ErreurApi> {
    if let Some(present) = jar.get(cookie::NOM) {
        sqlx::query("delete from sessions where empreinte = $1")
            .bind(jeton::empreinte(present.value()))
            .execute(bdd)
            .await?;
    }
    Ok(jar.remove(cookie::effacer()))
}
