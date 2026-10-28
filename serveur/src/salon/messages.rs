use commun::uno::{Couleur, Options};
use serde::Deserialize;

pub type Resultat = Result<(), &'static str>;

#[derive(Debug, Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum Client {
    Creer {
        set: Option<String>,
        pseudo: Option<String>,
    },
    Rejoindre {
        code: String,
        pseudo: Option<String>,
    },
    Options {
        options: Options,
    },
    ChoisirSet {
        set: String,
    },
    Lancer,
    Jouer {
        carte: String,
        couleur: Option<Couleur>,
        cible: Option<String>,
    },
    Piocher,
    Uno,
    Signaler {
        pseudo: String,
    },
    Contester,
}
