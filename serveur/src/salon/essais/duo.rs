use super::aide::{connecter, envoyer, nouveau, recus};
use crate::salon::registre::Registre;
use commun::uno::{Couleur, carte_par_id};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Duo {
    pub reg: Registre,
    pub a: u64,
    pub ra: UnboundedReceiver<String>,
    pub b: u64,
    pub rb: UnboundedReceiver<String>,
    pub code: String,
}

pub fn salon_a_deux() -> Duo {
    let mut reg = nouveau();
    let (a, mut ra) = connecter(&mut reg);
    let (b, mut rb) = connecter(&mut reg);
    envoyer(&mut reg, a, r#"{"t":"creer","pseudo":"alice"}"#);
    let code = recus(&mut ra)[0]["code"].as_str().unwrap().to_string();
    envoyer(&mut reg, b, &format!(r#"{{"t":"rejoindre","code":"{code}","pseudo":"bob"}}"#));
    recus(&mut ra);
    recus(&mut rb);
    Duo { reg, a, ra, b, rb, code }
}

pub fn partie_lancee() -> Duo {
    let mut d = salon_a_deux();
    envoyer(&mut d.reg, d.a, r#"{"t":"lancer"}"#);
    recus(&mut d.ra);
    recus(&mut d.rb);
    let partie = d.reg.salons.get_mut(&d.code).unwrap().partie.as_mut().unwrap();
    partie.joueurs[0].main = ["rouge-5-a", "bleu-1-a"].map(|id| carte_par_id(id).unwrap()).to_vec();
    partie.defausse = vec![carte_par_id("rouge-9-a").unwrap()];
    partie.couleur = Couleur::Rouge;
    partie.courant = 0;
    d
}
