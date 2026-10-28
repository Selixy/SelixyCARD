use super::etat::Partie;

impl Partie {
    pub fn tirer(&mut self, joueur: usize, n: u32) -> u32 {
        let mut tirees = 0;
        for _ in 0..n {
            if self.pioche.is_empty() {
                self.recycler();
            }
            match self.pioche.pop() {
                Some(carte) => self.joueurs[joueur].main.push(carte),
                None => break,
            }
            tirees += 1;
        }
        if tirees > 0 {
            self.joueurs[joueur].uno = false;
        }
        tirees
    }

    fn recycler(&mut self) {
        if self.defausse.len() <= 1 {
            return;
        }
        let haut = self.defausse.pop().expect("défausse non vide");
        let mut reste = std::mem::take(&mut self.defausse);
        self.alea.melanger(&mut reste);
        self.pioche = reste;
        self.defausse.push(haut);
    }
}
