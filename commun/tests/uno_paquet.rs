use commun::uno::{Alea, Carte, Couleur, TypeCarte, carte_par_id, emplacements, identifiants, paquet};
use std::collections::HashSet;

#[test]
fn cent_huit_emplacements_uniques() {
    let ids = identifiants();
    assert_eq!(ids.len(), 108);
    assert_eq!(ids.iter().collect::<HashSet<_>>().len(), 108);
    for attendu in ["rouge-0", "jaune-7-a", "vert-9-b", "bleu-passer-a", "rouge-inverser-b", "vert-plus2-a", "joker-1", "joker_plus4-4"] {
        assert!(ids.iter().any(|i| i == attendu), "{attendu} manque");
    }
}

#[test]
fn vingt_cinq_cartes_par_couleur() {
    let cartes = emplacements();
    for couleur in Couleur::TOUTES {
        assert_eq!(cartes.iter().filter(|c| c.couleur == Some(couleur)).count(), 25);
    }
    assert_eq!(cartes.iter().filter(|c| c.genre == TypeCarte::Joker).count(), 4);
    assert_eq!(cartes.iter().filter(|c| c.genre == TypeCarte::JokerPlus4).count(), 4);
    assert!(cartes.iter().filter(|c| c.couleur.is_none()).all(|c| c.valeur.is_none()));
}

#[test]
fn lecture_des_identifiants() {
    let sept = carte_par_id("bleu-7-b").unwrap();
    assert_eq!((sept.genre, sept.couleur, sept.valeur), (TypeCarte::Nombre, Some(Couleur::Bleu), Some(7)));
    assert_eq!(carte_par_id("rouge-0").unwrap().valeur, Some(0));
    assert_eq!(carte_par_id("vert-plus2-a").unwrap().genre, TypeCarte::Plus2);
    assert_eq!(carte_par_id("joker_plus4-2").unwrap().genre, TypeCarte::JokerPlus4);
    for faux in ["", "rouge-10-a", "rose-1-a", "rouge-x", "joker", "rouge-1-a-b"] {
        assert!(carte_par_id(faux).is_none(), "{faux} doit être refusé");
    }
}

#[test]
fn le_set_fournit_les_images() {
    let mut illustree: Carte = carte_par_id("rouge-5-a").unwrap();
    illustree.image = Some("abc".into());
    let cartes = paquet(&[illustree]);
    assert_eq!(cartes.len(), 108);
    assert_eq!(cartes.iter().find(|c| c.id == "rouge-5-a").unwrap().image.as_deref(), Some("abc"));
    assert_eq!(cartes.iter().filter(|c| c.image.is_some()).count(), 1);
}

#[test]
fn melange_deterministe_selon_la_graine() {
    let melanger = |graine| {
        let mut cartes = emplacements();
        Alea::nouvelle(graine).melanger(&mut cartes);
        cartes.into_iter().map(|c| c.id).collect::<Vec<_>>()
    };
    assert_eq!(melanger(42), melanger(42));
    assert_ne!(melanger(42), melanger(43));
    assert_ne!(melanger(42), identifiants());
    assert_eq!(melanger(7).iter().collect::<HashSet<_>>().len(), 108);
}
