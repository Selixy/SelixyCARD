use super::emplacements::emplacements;
use super::modele::SetEntrant;
use crate::api::erreur::ErreurApi;
use crate::images::{depot, verification};
use sqlx::PgPool;
use std::collections::HashSet;

pub fn verifier_nom(nom: &str) -> Result<(), ErreurApi> {
    let longueur = nom.trim().chars().count();
    if (1..=60).contains(&longueur) {
        Ok(())
    } else {
        Err(ErreurApi::Invalide("Le nom du set doit compter de 1 à 60 caractères."))
    }
}

fn verifier_cartes(entrant: &SetEntrant) -> Result<(), ErreurApi> {
    let attendus: HashSet<String> = emplacements().into_iter().map(|c| c.id).collect();
    let recus: HashSet<&String> = entrant.cartes.iter().map(|c| &c.id).collect();
    let complet = entrant.cartes.len() == attendus.len() && recus.len() == attendus.len();
    if complet && recus.iter().all(|id| attendus.contains(*id)) {
        Ok(())
    } else {
        Err(ErreurApi::Invalide("Le set doit contenir exactement les 108 cartes."))
    }
}

pub async fn verifier_set(bdd: &PgPool, entrant: &SetEntrant) -> Result<(), ErreurApi> {
    verifier_nom(&entrant.nom)?;
    verifier_cartes(entrant)?;
    let images: HashSet<String> = entrant
        .cartes
        .iter()
        .filter_map(|c| c.image.clone())
        .chain(entrant.dos.clone())
        .collect();
    if images.iter().any(|sha| !verification::est_sha256(sha)) {
        return Err(ErreurApi::Invalide("Identifiant d'image invalide."));
    }
    let liste: Vec<String> = images.into_iter().collect();
    if depot::compter_existantes(bdd, &liste).await? != liste.len() as i64 {
        return Err(ErreurApi::Invalide("Une image du set n'existe pas sur le serveur."));
    }
    Ok(())
}
