use crate::uno::erreur::Erreur;
use crate::uno::etat::Partie;

impl Partie {
    pub fn dire_uno(&mut self, pseudo: &str) -> Result<(), Erreur> {
        self.verifier_en_cours()?;
        let idx = self.index(pseudo)?;
        if !(1..=2).contains(&self.joueurs[idx].main.len()) {
            return Err(Erreur::TropDeCartes);
        }
        self.joueurs[idx].uno = true;
        if self.signalable == Some(idx) {
            self.signalable = None;
        }
        Ok(())
    }

    pub fn signaler(&mut self, signaleur: &str, cible: &str) -> Result<(), Erreur> {
        self.verifier_en_cours()?;
        if !self.options.penalite_uno {
            return Err(Erreur::OptionDesactivee);
        }
        let auteur = self.index(signaleur)?;
        let victime = self.index(cible)?;
        if self.signalable != Some(victime) || victime == auteur || self.joueurs[victime].uno {
            return Err(Erreur::SignalementInvalide);
        }
        self.tirer(victime, 2);
        self.signalable = None;
        Ok(())
    }

    pub fn marquer_signalable(&mut self, joueur: usize) {
        let expose = self.joueurs[joueur].main.len() == 1 && !self.joueurs[joueur].uno;
        if self.options.penalite_uno && expose {
            self.signalable = Some(joueur);
        }
    }
}
