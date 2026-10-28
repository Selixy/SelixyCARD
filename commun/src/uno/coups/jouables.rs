use super::jouer::Pose;
use crate::uno::cartes::Couleur;
use crate::uno::etat::Partie;

impl Partie {
    pub fn jouables(&self, joueur: usize) -> Vec<String> {
        if self.gagnant.is_some() || joueur >= self.joueurs.len() {
            return Vec::new();
        }
        let cible = self.joueurs.iter().enumerate().find(|(i, _)| *i != joueur).map(|(_, j)| j.pseudo.clone());
        let essai = Pose { carte: String::new(), couleur: Some(Couleur::Rouge), cible };
        self.joueurs[joueur]
            .main
            .iter()
            .filter(|carte| self.verifier_pose(joueur, carte, &essai).is_ok())
            .map(|carte| carte.id.clone())
            .collect()
    }

    pub fn piochee_de(&self, joueur: usize) -> Option<&str> {
        if joueur == self.courant { self.piochee.as_deref() } else { None }
    }
}
