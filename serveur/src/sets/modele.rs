use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Carte {
    pub id: String,
    #[serde(rename = "type")]
    pub genre: String,
    pub couleur: Option<String>,
    pub valeur: Option<i32>,
    pub image: Option<String>,
}

#[derive(Serialize)]
pub struct Set {
    pub id: Uuid,
    pub nom: String,
    pub auteur: String,
    pub dos: Option<String>,
    pub cartes: Vec<Carte>,
}

#[derive(Serialize)]
pub struct Resume {
    pub id: Uuid,
    pub nom: String,
    pub auteur: String,
    pub nb_illustrees: i64,
}

#[derive(Deserialize)]
pub struct CarteEntrante {
    pub id: String,
    pub image: Option<String>,
}

#[derive(Deserialize)]
pub struct SetEntrant {
    pub nom: String,
    pub dos: Option<String>,
    pub cartes: Vec<CarteEntrante>,
}

#[derive(Deserialize)]
pub struct NouveauSet {
    pub nom: String,
}
