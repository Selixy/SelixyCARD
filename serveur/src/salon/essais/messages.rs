use crate::salon::messages::Client;
use crate::salon::vues;
use commun::uno::{Couleur, Options};

#[test]
fn messages_du_contrat_lus() {
    let jouer: Client = serde_json::from_str(r#"{"t":"jouer","carte":"joker-1","couleur":"bleu","cible":"bob"}"#).unwrap();
    assert!(matches!(jouer, Client::Jouer { couleur: Some(Couleur::Bleu), cible: Some(_), .. }));
    for brut in [
        r#"{"t":"piocher"}"#,
        r#"{"t":"uno"}"#,
        r#"{"t":"contester"}"#,
        r#"{"t":"lancer"}"#,
        r#"{"t":"signaler","pseudo":"x"}"#,
        r#"{"t":"creer","pseudo":"Alice","set":null}"#,
    ] {
        assert!(serde_json::from_str::<Client>(brut).is_ok(), "{brut}");
    }
    assert!(serde_json::from_str::<Client>(r#"{"t":"inconnu"}"#).is_err());
}

#[test]
fn options_partielles_completees_par_defaut() {
    let Client::Options { options } = serde_json::from_str(r#"{"t":"options","options":{"jump_in":true}}"#).unwrap() else {
        panic!("options attendues")
    };
    assert_eq!(options, Options { jump_in: true, ..Options::default() });
}

#[test]
fn forme_de_l_erreur() {
    let texte = vues::erreur("Oups.");
    let valeur: serde_json::Value = serde_json::from_str(&texte).unwrap();
    assert_eq!((valeur["t"].as_str(), valeur["message"].as_str()), (Some("erreur"), Some("Oups.")));
}
