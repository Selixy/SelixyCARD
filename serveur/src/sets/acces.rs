use super::depot_lecture;
use super::modele::Set;
use crate::api::{erreur::ErreurApi, etat::Etat};
use crate::session::courant::CompteCourant;
use uuid::Uuid;

pub async fn lire_set(etat: &Etat, id: Uuid) -> Result<Set, ErreurApi> {
    depot_lecture::lire(&etat.bdd, id).await?.ok_or(ErreurApi::Introuvable("Set introuvable."))
}

pub async fn exiger_auteur(etat: &Etat, compte: &CompteCourant, id: Uuid) -> Result<(), ErreurApi> {
    match depot_lecture::auteur_de(&etat.bdd, id).await? {
        None => Err(ErreurApi::Introuvable("Set introuvable.")),
        Some(auteur) if auteur == compte.id => Ok(()),
        Some(_) => Err(ErreurApi::Interdit),
    }
}
