use super::disposition::{Cible, Face};
use super::etat::EtatTable;

const BLANC: [f32; 3] = [1.0, 1.0, 1.0];
pub const MAIN: u32 = 0;
pub const PREMIER_PLAN: u32 = 1;
pub const DEFAUSSE: u32 = 2;
pub const SURVOL: u32 = 3;
pub const PIOCHE: u32 = 4;
const ADVERSAIRE: u32 = 10;

pub fn groupe(etat: &EtatTable, cible: &Cible, survol: i32) -> Option<u32> {
    match cible.face {
        Face::Main(i) if etat.courant == 0 && etat.est_au_premier_plan(i) => Some(PREMIER_PLAN),
        Face::Main(i) if etat.courant == 0 && survol == i as i32 => Some(SURVOL),
        Face::Main(_) if etat.courant == 0 => Some(MAIN),
        Face::Defausse(0) if etat.couleur.is_some() => Some(DEFAUSSE),
        Face::Bloc => (etat.courant == 0 && etat.premier_plan < 0).then_some(PIOCHE),
        Face::Dos if cible.id >= 4000 => {
            let adv = (cible.id - 4000) / 100;
            (etat.courant == adv as i32 + 1).then_some(ADVERSAIRE + adv)
        }
        _ => None,
    }
}

pub fn couleur(etat: &EtatTable, groupe: u32) -> [f32; 4] {
    let oblige = etat.jouables.is_empty();
    if groupe == PIOCHE {
        let force = if oblige { 1.5 } else { 0.6 };
        return [1.0, 0.96, 0.88, force];
    }
    if (groupe == MAIN || groupe == SURVOL) && oblige {
        return [BLANC[0], BLANC[1], BLANC[2], 0.2];
    }
    if groupe != DEFAUSSE {
        return [1.0, 0.98, 0.92, 0.9];
    }
    let rgb = teinte(etat.couleur.as_deref()).unwrap_or(BLANC);
    [rgb[0], rgb[1], rgb[2], 1.0]
}

pub fn teinte(nom: Option<&str>) -> Option<[f32; 3]> {
    match nom? {
        "rouge" => Some([0.85, 0.28, 0.18]),
        "jaune" => Some([0.91, 0.72, 0.19]),
        "vert" => Some([0.31, 0.6, 0.38]),
        "bleu" => Some([0.23, 0.39, 0.72]),
        _ => None,
    }
}

pub fn teinte_carte(cle: &str, demandee: Option<&str>) -> [f32; 3] {
    let couleur = cle.rsplit(':').next().and_then(|id| id.split('-').next());
    teinte(couleur).or_else(|| teinte(demandee)).unwrap_or(BLANC)
}
