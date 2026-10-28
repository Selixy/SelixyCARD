use crate::uno::etat::{Attente, Partie};

impl Partie {
    pub fn infliger(&mut self, n: u32, attente: Attente) {
        let contestable = attente != Attente::Aucune && self.options.contester_plus4;
        if self.options.cumul_plus || contestable {
            self.penalite += n;
            if contestable {
                self.attente = attente;
            }
            self.avancer(1);
        } else {
            let victime = self.voisin(self.courant, 1);
            self.tirer(victime, n);
            self.avancer(2);
        }
    }

    pub fn subir_penalite(&mut self, joueur: usize) {
        self.tirer(joueur, self.penalite);
        self.penalite = 0;
        self.attente = Attente::Aucune;
        self.avancer(1);
    }
}
