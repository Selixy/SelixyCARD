mod uno_aide;
use commun::uno::{Couleur, Erreur, Options};
use uno_aide::*;

fn cumul() -> Options {
    Options { cumul_plus: true, ..Options::default() }
}

#[test]
fn cumul_actif_les_plus_deux_s_additionnent() {
    let mains: &[&[&str]] = &[&["rouge-plus2-a", "rouge-1-a"], &["jaune-plus2-a", "bleu-5-a"], &["vert-1-a", "joker_plus4-1"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, cumul(), &[]);
    p.jouer("a", pose("rouge-plus2-a")).unwrap();
    assert_eq!((p.penalite, p.courant), (2, 1));
    assert_eq!(p.jouer("b", pose("bleu-5-a")), Err(Erreur::PenaliteEnCours));
    p.jouer("b", pose("jaune-plus2-a")).unwrap();
    assert_eq!((p.penalite, p.courant), (4, 2));
    assert_eq!(p.jouer("c", avec_couleur("joker_plus4-1", Couleur::Bleu)), Err(Erreur::PenaliteEnCours));
    p.piocher("c").unwrap();
    assert_eq!((p.joueurs[2].main.len(), p.penalite, p.courant), (6, 0, 0));
}

#[test]
fn cumul_inactif_le_plus_deux_est_immediat() {
    let mains: &[&[&str]] = &[&["rouge-plus2-a", "rouge-1-a"], &["jaune-plus2-a", "bleu-5-a"], &["vert-1-a", "vert-2-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, Options::default(), &[]);
    p.jouer("a", pose("rouge-plus2-a")).unwrap();
    assert_eq!((p.penalite, p.courant, p.joueurs[1].main.len()), (0, 2, 4));
}

#[test]
fn cumul_actif_les_plus_quatre_se_repondent() {
    let options = Options { contester_plus4: false, ..cumul() };
    let mains: &[&[&str]] = &[&["joker_plus4-1", "bleu-1-a"], &["joker_plus4-2", "rouge-2-a"], &["vert-1-a", "vert-2-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Rouge, options, &[]);
    p.jouer("a", avec_couleur("joker_plus4-1", Couleur::Jaune)).unwrap();
    p.jouer("b", avec_couleur("joker_plus4-2", Couleur::Bleu)).unwrap();
    assert_eq!((p.penalite, p.courant, p.couleur), (8, 2, Couleur::Bleu));
    p.piocher("c").unwrap();
    assert_eq!((p.joueurs[2].main.len(), p.penalite), (10, 0));
}

#[test]
fn cumul_inactif_le_plus_quatre_sans_contestation_est_immediat() {
    let options = Options { contester_plus4: false, ..Options::default() };
    let mains: &[&[&str]] = &[&["joker_plus4-1", "bleu-1-a"], &["joker_plus4-2", "rouge-2-a"], &["vert-1-a", "vert-2-a"]];
    let mut p = partie(mains, "rouge-9-a", Couleur::Vert, options, &[]);
    p.jouer("a", avec_couleur("joker_plus4-1", Couleur::Bleu)).unwrap();
    assert_eq!((p.joueurs[1].main.len(), p.courant, p.penalite, p.couleur), (6, 2, 0, Couleur::Bleu));
}
