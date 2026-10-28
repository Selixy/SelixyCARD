use crate::uno::cartes::Couleur;
use crate::uno::erreur::Erreur;
use crate::uno::etat::{Attente, Partie};

#[derive(Clone, Debug, Default)]
pub struct Pose {
    pub carte: String,
    pub couleur: Option<Couleur>,
    pub cible: Option<String>,
}

impl Partie {
    pub fn jouer(&mut self, pseudo: &str, pose: Pose) -> Result<(), Erreur> {
        self.verifier_en_cours()?;
        let idx = self.index(pseudo)?;
        let position = self.joueurs[idx].main.iter().position(|c| c.id == pose.carte).ok_or(Erreur::CarteAbsente)?;
        let carte = self.joueurs[idx].main[position].clone();
        let cible = self.verifier_pose(idx, &carte, &pose)?;
        let avait_couleur = self.a_la_couleur(idx);
        self.joueurs[idx].main.remove(position);
        self.defausse.push(carte.clone());
        self.couleur = carte.couleur.or(pose.couleur).ok_or(Erreur::CouleurRequise)?;
        self.courant = idx;
        self.piochee = None;
        self.attente = Attente::Aucune;
        self.signalable = None;
        if self.joueurs[idx].main.is_empty() {
            self.gagnant = Some(idx);
            return Ok(());
        }
        self.appliquer_effet(idx, &carte, cible, avait_couleur);
        self.marquer_signalable(idx);
        Ok(())
    }
}
