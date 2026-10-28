#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Couleur {
    Rouge,
    Jaune,
    Vert,
    Bleu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TypeCarte {
    Nombre,
    Passer,
    Inverser,
    Plus2,
    Joker,
    JokerPlus4,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Carte {
    pub id: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub genre: TypeCarte,
    pub couleur: Option<Couleur>,
    pub valeur: Option<u8>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub image: Option<String>,
}

impl Couleur {
    pub const TOUTES: [Couleur; 4] = [Couleur::Rouge, Couleur::Jaune, Couleur::Vert, Couleur::Bleu];

    pub fn nom(self) -> &'static str {
        match self {
            Couleur::Rouge => "rouge",
            Couleur::Jaune => "jaune",
            Couleur::Vert => "vert",
            Couleur::Bleu => "bleu",
        }
    }

    pub fn depuis_nom(nom: &str) -> Option<Couleur> {
        Couleur::TOUTES.into_iter().find(|c| c.nom() == nom)
    }
}
