use super::salle::Salon;
use commun::uno::{Attente, Partie};
use serde_json::{Value, json};

pub fn salon(salon: &Salon) -> Value {
    let joueurs: Vec<&str> = salon.membres.iter().map(|m| m.pseudo.as_str()).collect();
    let set = if salon.set.is_empty() { Value::Null } else { json!(salon.set) };
    json!({ "t": "salon", "code": salon.code, "hote": salon.hote, "joueurs": joueurs, "options": salon.options, "set": set })
}

pub fn etat(partie: &Partie) -> Value {
    let joueurs: Vec<Value> = partie
        .joueurs
        .iter()
        .map(|j| json!({ "pseudo": j.pseudo, "nb_cartes": j.main.len(), "uno": j.uno }))
        .collect();
    json!({
        "t": "etat",
        "defausse": partie.haut(),
        "couleur_courante": partie.couleur,
        "sens": partie.sens,
        "courant": partie.joueurs[partie.courant].pseudo,
        "joueurs": joueurs,
        "pioche": partie.pioche.len(),
        "penalite": partie.penalite,
        "contestable": matches!(partie.attente, Attente::Contestation { .. }),
        "signalable": partie.signalable.map(|i| partie.joueurs[i].pseudo.as_str()),
    })
}

pub fn main(partie: &Partie, joueur: usize) -> Value {
    json!({
        "t": "main",
        "cartes": partie.joueurs[joueur].main,
        "jouables": partie.jouables(joueur),
        "piochee": partie.piochee_de(joueur),
    })
}

pub fn fin(partie: &Partie) -> Option<Value> {
    partie.gagnant.map(|i| json!({ "t": "fin", "gagnant": partie.joueurs[i].pseudo }))
}

pub fn erreur(message: &str) -> String {
    json!({ "t": "erreur", "message": message }).to_string()
}
