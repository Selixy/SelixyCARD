mod uno_aide;
use commun::uno::{Erreur, Options, Partie, TypeCarte};
use uno_aide::graines::{graine_avec, pseudos};

#[test]
fn distribution_de_sept_cartes() {
    let partie = Partie::nouvelle(&pseudos(4), &[], Options::default(), 1).unwrap();
    assert!(partie.joueurs.iter().all(|j| j.main.len() == 7 && !j.uno));
    assert_eq!(partie.defausse.len(), 1);
    assert_eq!(partie.pioche.len() + partie.defausse.len() + 28, 108);
    assert_eq!(partie.penalite, 0);
}

#[test]
fn joueurs_invalides_refuses() {
    let essai = |pseudos: &[String]| Partie::nouvelle(pseudos, &[], Options::default(), 1).err();
    assert_eq!(essai(&pseudos(1)), Some(Erreur::JoueursInvalides));
    assert_eq!(essai(&["a".to_string(), "a".to_string()]), Some(Erreur::JoueursInvalides));
    assert_eq!(essai(&["a".to_string(), String::new()]), Some(Erreur::JoueursInvalides));
    assert_eq!(essai(&(0..11).map(|i| format!("j{i}")).collect::<Vec<_>>()), Some(Erreur::JoueursInvalides));
    assert!(Partie::nouvelle(&(0..10).map(|i| format!("j{i}")).collect::<Vec<_>>(), &[], Options::default(), 1).is_ok());
}

#[test]
fn graine_reproductible_et_jamais_plus4_en_premiere() {
    let a = Partie::nouvelle(&pseudos(3), &[], Options::default(), 9).unwrap();
    let b = Partie::nouvelle(&pseudos(3), &[], Options::default(), 9).unwrap();
    assert_eq!(a.joueurs[0].main, b.joueurs[0].main);
    assert_eq!(a.haut(), b.haut());
    for graine in 0..300 {
        let p = Partie::nouvelle(&pseudos(2), &[], Options::default(), graine).unwrap();
        assert_ne!(p.haut().genre, TypeCarte::JokerPlus4);
        assert_eq!(p.pioche.len() + p.defausse.len() + p.joueurs.iter().map(|j| j.main.len()).sum::<usize>(), 108);
    }
}

#[test]
fn premiere_carte_passer_inverser_plus2() {
    let passer = graine_avec(TypeCarte::Passer, 3);
    assert_eq!((passer.courant, passer.sens), (1, 1));
    let inverser = graine_avec(TypeCarte::Inverser, 3);
    assert_eq!((inverser.courant, inverser.sens), (0, -1));
    let inverser_duel = graine_avec(TypeCarte::Inverser, 2);
    assert_eq!(inverser_duel.courant, 1);
    let plus2 = graine_avec(TypeCarte::Plus2, 3);
    assert_eq!((plus2.courant, plus2.joueurs[0].main.len()), (1, 9));
}

#[test]
fn premiere_carte_joker_choisit_une_couleur() {
    let p = graine_avec(TypeCarte::Joker, 3);
    assert_eq!(p.courant, 0);
    assert!(p.haut().couleur.is_none());
    let nombre = graine_avec(TypeCarte::Nombre, 3);
    assert_eq!(Some(nombre.couleur), nombre.haut().couleur);
}
