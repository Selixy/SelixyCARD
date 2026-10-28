use crate::uno::cartes::{Carte, TypeCarte};
use crate::uno::etat::{Attente, Partie};

impl Partie {
    pub fn appliquer_effet(&mut self, joueur: usize, carte: &Carte, cible: Option<usize>, avait_couleur: bool) {
        match carte.genre {
            TypeCarte::Nombre => self.effet_nombre(carte.valeur, joueur, cible),
            TypeCarte::Passer => self.avancer(2),
            TypeCarte::Inverser => {
                self.sens = -self.sens;
                self.avancer(if self.joueurs.len() == 2 { 2 } else { 1 });
            }
            TypeCarte::Plus2 => self.infliger(2, Attente::Aucune),
            TypeCarte::JokerPlus4 => self.infliger(4, Attente::Contestation { poseur: joueur, avait_couleur }),
            TypeCarte::Joker => self.avancer(1),
        }
    }

    fn effet_nombre(&mut self, valeur: Option<u8>, joueur: usize, cible: Option<usize>) {
        if self.options.sept_zero {
            match (valeur, cible) {
                (Some(7), Some(cible)) => self.echanger(joueur, cible),
                (Some(0), _) => self.tourner_mains(),
                _ => {}
            }
        }
        self.avancer(1);
    }
}
