use super::emplacements::emplacements;
use super::modele::SetEntrant;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn creer(bdd: &PgPool, auteur: Uuid, nom: &str) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    let cartes = emplacements();
    let rangs: Vec<i32> = (0..cartes.len() as i32).collect();
    let mut transaction = bdd.begin().await?;
    sqlx::query("insert into sets (id, auteur_id, nom) values ($1, $2, $3)")
        .bind(id)
        .bind(auteur)
        .bind(nom)
        .execute(&mut *transaction)
        .await?;
    sqlx::query(
        "insert into cartes_set (set_id, id, rang, genre, couleur, valeur) \
         select $1, * from unnest($2::text[], $3::int[], $4::text[], $5::text[], $6::int[])",
    )
    .bind(id)
    .bind(cartes.iter().map(|c| c.id.clone()).collect::<Vec<_>>())
    .bind(rangs)
    .bind(cartes.iter().map(|c| c.genre.clone()).collect::<Vec<_>>())
    .bind(cartes.iter().map(|c| c.couleur.clone()).collect::<Vec<_>>())
    .bind(cartes.iter().map(|c| c.valeur).collect::<Vec<_>>())
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(id)
}

pub async fn remplacer(bdd: &PgPool, id: Uuid, entrant: &SetEntrant) -> Result<(), sqlx::Error> {
    let mut transaction = bdd.begin().await?;
    sqlx::query("update sets set nom = $2, dos = $3, modifie_le = now() where id = $1")
        .bind(id)
        .bind(entrant.nom.trim())
        .bind(&entrant.dos)
        .execute(&mut *transaction)
        .await?;
    sqlx::query(
        "update cartes_set k set image = v.image \
         from unnest($2::text[], $3::text[]) as v(id, image) where k.set_id = $1 and k.id = v.id",
    )
    .bind(id)
    .bind(entrant.cartes.iter().map(|c| c.id.clone()).collect::<Vec<_>>())
    .bind(entrant.cartes.iter().map(|c| c.image.clone()).collect::<Vec<_>>())
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await
}

pub async fn supprimer(bdd: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sets where id = $1").bind(id).execute(bdd).await?;
    Ok(())
}
