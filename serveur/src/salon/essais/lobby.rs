use super::aide::{SansSet, connecter, envoyer, nouveau, recus};
use super::duo::salon_a_deux;
use serde_json::json;
use std::sync::Arc;

#[test]
fn creer_puis_rejoindre() {
    let mut reg = nouveau();
    let (a, mut ra) = connecter(&mut reg);
    let (b, mut rb) = connecter(&mut reg);
    envoyer(&mut reg, a, r#"{"t":"creer","pseudo":"alice","set":null}"#);
    let salon = recus(&mut ra).remove(0);
    assert_eq!((salon["t"].as_str(), salon["hote"].as_str(), salon["code"].as_str().map(str::len)), (Some("salon"), Some("alice"), Some(5)));
    assert_eq!(salon["options"]["penalite_uno"], json!(true));
    let rejoindre = format!(r#"{{"t":"rejoindre","code":"{}","pseudo":"bob"}}"#, salon["code"].as_str().unwrap().to_lowercase());
    envoyer(&mut reg, b, &rejoindre);
    for rx in [&mut ra, &mut rb] {
        assert_eq!(recus(rx)[0]["joueurs"], json!(["alice", "bob"]));
    }
}

#[test]
fn refus_de_pseudo_de_salon_et_de_set() {
    let mut d = salon_a_deux();
    let (c, mut rc) = connecter(&mut d.reg);
    envoyer(&mut d.reg, c, &format!(r#"{{"t":"rejoindre","code":"{}","pseudo":"bob"}}"#, d.code));
    envoyer(&mut d.reg, c, r#"{"t":"rejoindre","code":"ZZZZZ","pseudo":"carl"}"#);
    envoyer(&mut d.reg, c, r#"{"t":"creer","pseudo":"   "}"#);
    envoyer(&mut d.reg, d.b, r#"{"t":"creer","pseudo":"bob"}"#);
    let refus = recus(&mut rc);
    assert_eq!(refus.len(), 3);
    assert!(refus.iter().all(|m| m["t"] == "erreur"));
    assert_eq!(recus(&mut d.rb)[0]["message"], "Tu es déjà dans un salon.");
    assert!(recus(&mut d.ra).is_empty());
    d.reg.source = Arc::new(SansSet);
    envoyer(&mut d.reg, c, r#"{"t":"creer","pseudo":"carl","set":"inconnu"}"#);
    assert_eq!(recus(&mut rc)[0]["message"], "Set introuvable.");
}

#[test]
fn options_et_set_reserves_a_l_hote() {
    let mut d = salon_a_deux();
    envoyer(&mut d.reg, d.b, r#"{"t":"options","options":{"cumul_plus":true}}"#);
    envoyer(&mut d.reg, d.b, r#"{"t":"choisir_set","set":"x"}"#);
    assert!(recus(&mut d.rb).iter().all(|m| m["t"] == "erreur"));
    envoyer(&mut d.reg, d.a, r#"{"t":"options","options":{"cumul_plus":true,"sept_zero":true}}"#);
    envoyer(&mut d.reg, d.a, r#"{"t":"choisir_set","set":"mon-set"}"#);
    let dernier = recus(&mut d.rb).pop().unwrap();
    assert_eq!(dernier["set"], "mon-set");
    assert_eq!((dernier["options"]["cumul_plus"].as_bool(), dernier["options"]["jump_in"].as_bool()), (Some(true), Some(false)));
    assert!(!recus(&mut d.ra).is_empty());
}

#[test]
fn lancement_distribue_etat_et_mains() {
    let mut d = salon_a_deux();
    envoyer(&mut d.reg, d.b, r#"{"t":"lancer"}"#);
    assert_eq!(recus(&mut d.rb)[0]["t"], "erreur");
    envoyer(&mut d.reg, d.a, r#"{"t":"lancer"}"#);
    let recus_a = recus(&mut d.ra);
    let etat = recus_a.iter().find(|m| m["t"] == "etat").unwrap();
    assert_eq!(etat["joueurs"].as_array().unwrap().len(), 2);
    assert_eq!(etat["joueurs"][0]["nb_cartes"], 7);
    let main = recus_a.iter().find(|m| m["t"] == "main").unwrap();
    assert_eq!(main["cartes"].as_array().unwrap().len(), 7);
    assert!(main["cartes"][0]["type"].is_string());
    assert!(recus(&mut d.rb).iter().any(|m| m["t"] == "main"));
    envoyer(&mut d.reg, d.a, r#"{"t":"lancer"}"#);
    assert_eq!(recus(&mut d.ra)[0]["message"], "La partie a déjà commencé.");
}

#[test]
fn deconnexions_et_reconnexion() {
    let mut d = salon_a_deux();
    d.reg.deconnecter(d.a);
    assert_eq!(recus(&mut d.rb)[0]["hote"], "bob");
    d.reg.deconnecter(d.b);
    assert!(d.reg.salons.is_empty());
    let mut e = salon_a_deux();
    envoyer(&mut e.reg, e.a, r#"{"t":"lancer"}"#);
    e.reg.deconnecter(e.b);
    let (b2, mut rb2) = connecter(&mut e.reg);
    envoyer(&mut e.reg, b2, &format!(r#"{{"t":"rejoindre","code":"{}","pseudo":"bob"}}"#, e.code));
    let recus_b = recus(&mut rb2);
    assert!(recus_b.iter().any(|m| m["t"] == "etat") && recus_b.iter().any(|m| m["t"] == "main"));
    assert_eq!(e.reg.salons.len(), 1);
}
