use crate::uno::etat::Partie;

impl Partie {
    pub fn echanger(&mut self, a: usize, b: usize) {
        let main_a = std::mem::take(&mut self.joueurs[a].main);
        let main_b = std::mem::replace(&mut self.joueurs[b].main, main_a);
        self.joueurs[a].main = main_b;
        self.joueurs[a].uno = false;
        self.joueurs[b].uno = false;
    }

    pub fn tourner_mains(&mut self) {
        let n = self.joueurs.len();
        let mut anciennes: Vec<_> = self.joueurs.iter_mut().map(|j| std::mem::take(&mut j.main)).collect();
        for i in 0..n {
            let destinataire = self.voisin(i, 1);
            self.joueurs[destinataire].main = std::mem::take(&mut anciennes[i]);
        }
        self.joueurs.iter_mut().for_each(|j| j.uno = false);
    }
}
