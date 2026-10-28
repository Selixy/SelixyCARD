use crate::uno::erreur::Erreur;
use crate::uno::etat::Partie;

impl Partie {
    pub fn piocher(&mut self, pseudo: &str) -> Result<(), Erreur> {
        self.verifier_en_cours()?;
        let idx = self.index(pseudo)?;
        if idx != self.courant {
            return Err(Erreur::PasTonTour);
        }
        self.signalable = None;
        if self.penalite > 0 {
            self.subir_penalite(idx);
        } else if self.piochee.is_some() {
            self.avancer(1);
        } else {
            self.piocher_normalement(idx);
        }
        Ok(())
    }

    fn piocher_normalement(&mut self, idx: usize) {
        let mut tirees = 0;
        let mut jouable = false;
        while !jouable && (self.options.piocher_jusqua_jouable || tirees < 1) {
            if self.tirer(idx, 1) == 0 {
                break;
            }
            tirees += 1;
            let derniere = self.joueurs[idx].main.last().expect("carte piochée").clone();
            jouable = self.peut_poser(idx, &derniere);
        }
        if jouable {
            self.piochee = self.joueurs[idx].main.last().map(|c| c.id.clone());
        } else {
            self.avancer(1);
        }
    }
}
