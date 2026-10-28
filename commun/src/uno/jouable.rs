use super::cartes::{Carte, TypeCarte};
use super::etat::Partie;

impl Partie {
    pub fn a_la_couleur(&self, joueur: usize) -> bool {
        self.joueurs[joueur].main.iter().any(|c| c.couleur == Some(self.couleur))
    }

    pub fn identique(&self, carte: &Carte) -> bool {
        let haut = self.haut();
        carte.couleur.is_some() && carte.genre == haut.genre && carte.couleur == haut.couleur && carte.valeur == haut.valeur
    }

    pub fn peut_poser(&self, joueur: usize, carte: &Carte) -> bool {
        if self.penalite > 0 {
            return self.options.cumul_plus && carte.genre == self.haut().genre;
        }
        let haut = self.haut();
        let correspond = match carte.genre {
            TypeCarte::Joker | TypeCarte::JokerPlus4 => true,
            _ => carte.couleur == Some(self.couleur) || (carte.genre == haut.genre && carte.valeur == haut.valeur),
        };
        let plus4_permis = self.options.contester_plus4 || !self.a_la_couleur(joueur);
        correspond && (carte.genre != TypeCarte::JokerPlus4 || plus4_permis)
    }
}
