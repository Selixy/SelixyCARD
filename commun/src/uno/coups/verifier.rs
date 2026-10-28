use super::jouer::Pose;
use crate::uno::cartes::{Carte, TypeCarte};
use crate::uno::erreur::Erreur;
use crate::uno::etat::Partie;

impl Partie {
    pub fn verifier_pose(&self, joueur: usize, carte: &Carte, pose: &Pose) -> Result<Option<usize>, Erreur> {
        if joueur != self.courant {
            let saut = self.options.jump_in && self.penalite == 0 && self.identique(carte);
            if !saut {
                return Err(Erreur::PasTonTour);
            }
        } else {
            if self.piochee.as_ref().is_some_and(|id| *id != carte.id) {
                return Err(Erreur::SeulementPiochee);
            }
            if !self.peut_poser(joueur, carte) {
                return Err(if self.penalite > 0 { Erreur::PenaliteEnCours } else { Erreur::CarteInjouable });
            }
        }
        let joker = matches!(carte.genre, TypeCarte::Joker | TypeCarte::JokerPlus4);
        if joker && pose.couleur.is_none() {
            return Err(Erreur::CouleurRequise);
        }
        self.verifier_cible(joueur, carte, pose)
    }

    fn verifier_cible(&self, joueur: usize, carte: &Carte, pose: &Pose) -> Result<Option<usize>, Erreur> {
        let echange = self.options.sept_zero && carte.genre == TypeCarte::Nombre && carte.valeur == Some(7);
        if !echange {
            return Ok(None);
        }
        let pseudo = pose.cible.as_deref().ok_or(Erreur::CibleRequise)?;
        let cible = self.index(pseudo).map_err(|_| Erreur::CibleInvalide)?;
        if cible == joueur { Err(Erreur::CibleInvalide) } else { Ok(Some(cible)) }
    }
}
