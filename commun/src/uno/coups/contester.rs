use crate::uno::erreur::Erreur;
use crate::uno::etat::{Attente, Partie};

impl Partie {
    pub fn contester(&mut self, pseudo: &str) -> Result<(), Erreur> {
        self.verifier_en_cours()?;
        let idx = self.index(pseudo)?;
        if idx != self.courant {
            return Err(Erreur::PasTonTour);
        }
        let Attente::Contestation { poseur, avait_couleur } = self.attente else {
            return Err(Erreur::RienAContester);
        };
        self.attente = Attente::Aucune;
        self.signalable = None;
        if avait_couleur {
            self.tirer(poseur, 4);
            self.penalite = self.penalite.saturating_sub(4);
        } else {
            self.tirer(idx, self.penalite + 2);
            self.penalite = 0;
            self.avancer(1);
        }
        Ok(())
    }
}
