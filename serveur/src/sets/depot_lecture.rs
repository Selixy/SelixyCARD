use super::modele::{Carte, Resume, Set};
use sqlx::PgPool;
use uuid::Uuid;

type LigneCarte = (String, String, Option<String>, Option<i32>, Option<String>);

pub async fn lister(bdd: &PgPool, auteur_id: Uuid) -> Result<Vec<Resume>, sqlx::Error> {
    let lignes = sqlx::query_as::<_, (Uuid, String, String, i64)>(
        "select s.id, s.nom, c.pseudo, count(k.image) from sets s \
         join comptes c on c.id = s.auteur_id left join cartes_set k on k.set_id = s.id \
         where s.auteur_id = $1 \
         group by s.id, s.nom, c.pseudo, s.cree_le order by s.cree_le desc",
    )
    .bind(auteur_id)
    .fetch_all(bdd)
    .await?;
    Ok(lignes
        .into_iter()
        .map(|(id, nom, auteur, nb_illustrees)| Resume { id, nom, auteur, nb_illustrees })
        .collect())
}

pub async fn auteur_de(bdd: &PgPool, id: Uuid) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar("select auteur_id from sets where id = $1")
        .bind(id)
        .fetch_optional(bdd)
        .await
}

pub async fn lire(bdd: &PgPool, id: Uuid) -> Result<Option<Set>, sqlx::Error> {
    let entete = sqlx::query_as::<_, (String, Option<String>, String)>(
        "select s.nom, s.dos, c.pseudo from sets s join comptes c on c.id = s.auteur_id where s.id = $1",
    )
    .bind(id)
    .fetch_optional(bdd)
    .await?;
    let Some((nom, dos, auteur)) = entete else { return Ok(None) };
    let lignes = sqlx::query_as::<_, LigneCarte>(
        "select id, genre, couleur, valeur, image from cartes_set where set_id = $1 order by rang",
    )
    .bind(id)
    .fetch_all(bdd)
    .await?;
    let cartes = lignes
        .into_iter()
        .map(|(id, genre, couleur, valeur, image)| Carte { id, genre, couleur, valeur, image })
        .collect();
    Ok(Some(Set { id, nom, auteur, dos, cartes }))
}
