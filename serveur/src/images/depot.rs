use sqlx::PgPool;
use uuid::Uuid;

pub async fn enregistrer(bdd: &PgPool, sha256: &str, octets: &[u8], auteur: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("insert into images (sha256, octets, auteur_id) values ($1, $2, $3) on conflict do nothing")
        .bind(sha256)
        .bind(octets)
        .bind(auteur)
        .execute(bdd)
        .await?;
    Ok(())
}

pub async fn lire(bdd: &PgPool, sha256: &str) -> Result<Option<Vec<u8>>, sqlx::Error> {
    sqlx::query_scalar("select octets from images where sha256 = $1")
        .bind(sha256)
        .fetch_optional(bdd)
        .await
}

pub async fn compter_existantes(bdd: &PgPool, sha256: &[String]) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("select count(*) from images where sha256 = any($1)")
        .bind(sha256)
        .fetch_one(bdd)
        .await
}
