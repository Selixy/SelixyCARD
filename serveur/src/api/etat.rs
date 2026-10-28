use sqlx::PgPool;

#[derive(Clone)]
pub struct Etat {
    pub bdd: PgPool,
}
