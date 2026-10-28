use super::aide::{envoyer, recus};
use super::duo::{partie_lancee, salon_a_deux};

#[test]
fn coup_hors_tour_refuse_a_lui_seul() {
    let mut d = partie_lancee();
    envoyer(&mut d.reg, d.b, r#"{"t":"piocher"}"#);
    let refus = recus(&mut d.rb);
    assert_eq!((refus[0]["t"].as_str(), refus[0]["message"].as_str()), (Some("erreur"), Some("Ce n'est pas ton tour.")));
    assert!(recus(&mut d.ra).is_empty());
}

#[test]
fn jouer_diffuse_etat_et_main_privee() {
    let mut d = partie_lancee();
    envoyer(&mut d.reg, d.a, r#"{"t":"jouer","carte":"rouge-5-a"}"#);
    let recus_b = recus(&mut d.rb);
    let etat = recus_b.iter().find(|m| m["t"] == "etat").unwrap();
    assert_eq!((etat["courant"].as_str(), etat["defausse"]["id"].as_str(), etat["couleur_courante"].as_str()), (Some("bob"), Some("rouge-5-a"), Some("rouge")));
    assert_eq!((etat["joueurs"][0]["nb_cartes"].as_u64(), etat["sens"].as_i64(), etat["penalite"].as_u64()), (Some(1), Some(1), Some(0)));
    assert_eq!(etat["signalable"], "alice");
    let main_b = recus_b.iter().find(|m| m["t"] == "main").unwrap();
    assert_eq!(main_b["cartes"].as_array().unwrap().len(), 7);
    let main_a = recus(&mut d.ra).into_iter().find(|m| m["t"] == "main").unwrap();
    assert_eq!(main_a["cartes"][0]["id"], "bleu-1-a");
}

#[test]
fn coup_injouable_puis_signalement_penalise() {
    let mut d = partie_lancee();
    envoyer(&mut d.reg, d.a, r#"{"t":"jouer","carte":"bleu-1-a"}"#);
    assert_eq!(recus(&mut d.ra).iter().find(|m| m["t"] == "erreur").unwrap()["t"], "erreur");
    envoyer(&mut d.reg, d.a, r#"{"t":"jouer","carte":"rouge-5-a"}"#);
    envoyer(&mut d.reg, d.b, r#"{"t":"signaler","pseudo":"alice"}"#);
    let etat = recus(&mut d.rb).into_iter().filter(|m| m["t"] == "etat").last().unwrap();
    assert_eq!(etat["joueurs"][0]["nb_cartes"], 3);
}

#[test]
fn la_fin_est_annoncee_a_tous() {
    let mut d = partie_lancee();
    d.reg.salons.get_mut(&d.code).unwrap().partie.as_mut().unwrap().joueurs[0].main.truncate(1);
    envoyer(&mut d.reg, d.a, r#"{"t":"jouer","carte":"rouge-5-a"}"#);
    for rx in [&mut d.ra, &mut d.rb] {
        let fin = recus(rx).into_iter().find(|m| m["t"] == "fin").unwrap();
        assert_eq!(fin["gagnant"], "alice");
    }
}

#[test]
fn actions_de_partie_sans_partie_refusees() {
    let mut d = salon_a_deux();
    envoyer(&mut d.reg, d.a, r#"{"t":"piocher"}"#);
    envoyer(&mut d.reg, d.a, r#"{"t":"contester"}"#);
    let messages = recus(&mut d.ra);
    assert_eq!(messages.len(), 2);
    assert!(messages.iter().all(|m| m["message"] == "La partie n'a pas commencé."));
}
