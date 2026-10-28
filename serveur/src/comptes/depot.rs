use sqlx::PgPool;
use uuid::Uuid;

pub struct Compte {
    pub id: Uuid,
    pub pseudo: String,
    pub mot_de_passe: String,
}

pub async fn creer(bdd: &PgPool, pseudo: &str, empreinte: &str) -> Result<Option<Uuid>, sqlx::Error> {
    let id = Uuid::new_v4();
    let resultat = sqlx::query(
        "insert into comptes (id, pseudo, mot_de_passe) values ($1, $2, $3) on conflict do nothing",
    )
    .bind(id)
    .bind(pseudo)
    .bind(empreinte)
    .execute(bdd)
    .await?;
    Ok((resultat.rows_affected() == 1).then_some(id))
}

pub async fn par_pseudo(bdd: &PgPool, pseudo: &str) -> Result<Option<Compte>, sqlx::Error> {
    let ligne = sqlx::query_as::<_, (Uuid, String, String)>(
        "select id, pseudo, mot_de_passe from comptes where lower(pseudo) = lower($1)",
    )
    .bind(pseudo)
    .fetch_optional(bdd)
    .await?;
    Ok(ligne.map(|(id, pseudo, mot_de_passe)| Compte { id, pseudo, mot_de_passe }))
}
