use super::cartes::{Carte, Couleur, TypeCarte};

pub fn identifiants() -> Vec<String> {
    let mut ids = Vec::with_capacity(108);
    for couleur in Couleur::TOUTES {
        let n = couleur.nom();
        ids.push(format!("{n}-0"));
        for v in 1..=9 {
            ids.extend(["a", "b"].map(|l| format!("{n}-{v}-{l}")));
        }
        for t in ["passer", "inverser", "plus2"] {
            ids.extend(["a", "b"].map(|l| format!("{n}-{t}-{l}")));
        }
    }
    for i in 1..=4 {
        ids.push(format!("joker-{i}"));
        ids.push(format!("joker_plus4-{i}"));
    }
    ids
}

fn type_et_valeur(mot: &str) -> Option<(TypeCarte, Option<u8>)> {
    match mot {
        "passer" => Some((TypeCarte::Passer, None)),
        "inverser" => Some((TypeCarte::Inverser, None)),
        "plus2" => Some((TypeCarte::Plus2, None)),
        n => n.parse::<u8>().ok().filter(|v| (1..=9).contains(v)).map(|v| (TypeCarte::Nombre, Some(v))),
    }
}

pub fn carte_par_id(id: &str) -> Option<Carte> {
    let parties: Vec<&str> = id.split('-').collect();
    let (genre, couleur, valeur) = match parties.as_slice() {
        ["joker", _] => (TypeCarte::Joker, None, None),
        ["joker_plus4", _] => (TypeCarte::JokerPlus4, None, None),
        [c, "0"] => (TypeCarte::Nombre, Some(Couleur::depuis_nom(c)?), Some(0)),
        [c, mot, _] => {
            let (genre, valeur) = type_et_valeur(mot)?;
            (genre, Some(Couleur::depuis_nom(c)?), valeur)
        }
        _ => return None,
    };
    Some(Carte { id: id.to_string(), genre, couleur, valeur, image: None })
}

pub fn emplacements() -> Vec<Carte> {
    identifiants().iter().filter_map(|id| carte_par_id(id)).collect()
}

pub fn paquet(set: &[Carte]) -> Vec<Carte> {
    emplacements()
        .into_iter()
        .map(|vide| set.iter().find(|c| c.id == vide.id).cloned().unwrap_or(vide))
        .collect()
}
