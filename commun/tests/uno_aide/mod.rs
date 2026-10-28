#![allow(dead_code)]

pub mod graines;

use commun::uno::{Alea, Attente, Carte, Couleur, Joueur, Options, Partie, Pose, carte_par_id, emplacements};

pub fn carte(id: &str) -> Carte {
    carte_par_id(id).expect("identifiant valide")
}

pub fn partie(mains: &[&[&str]], haut: &str, couleur: Couleur, options: Options, pioche_haute: &[&str]) -> Partie {
    let pseudos = ["a", "b", "c", "d"];
    let joueurs: Vec<Joueur> = mains
        .iter()
        .enumerate()
        .map(|(i, m)| Joueur { pseudo: pseudos[i].into(), main: m.iter().map(|id| carte(id)).collect(), uno: false })
        .collect();
    let utilisees: Vec<&str> = mains.iter().flat_map(|m| m.iter().copied()).chain([haut]).chain(pioche_haute.iter().copied()).collect();
    let mut pioche: Vec<Carte> = emplacements().into_iter().filter(|c| !utilisees.contains(&c.id.as_str())).collect();
    pioche.extend(pioche_haute.iter().rev().map(|id| carte(id)));
    Partie {
        options,
        joueurs,
        pioche,
        defausse: vec![carte(haut)],
        couleur,
        sens: 1,
        courant: 0,
        penalite: 0,
        attente: Attente::Aucune,
        signalable: None,
        piochee: None,
        gagnant: None,
        alea: Alea::nouvelle(1),
    }
}

pub fn pose(id: &str) -> Pose {
    Pose { carte: id.into(), ..Pose::default() }
}

pub fn avec_couleur(id: &str, couleur: Couleur) -> Pose {
    Pose { carte: id.into(), couleur: Some(couleur), cible: None }
}

pub fn avec_cible(id: &str, cible: &str) -> Pose {
    Pose { carte: id.into(), couleur: None, cible: Some(cible.into()) }
}
