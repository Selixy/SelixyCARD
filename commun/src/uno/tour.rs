use super::etat::Partie;

impl Partie {
    pub fn voisin(&self, depuis: usize, pas: usize) -> usize {
        let n = self.joueurs.len() as i32;
        (depuis as i32 + self.sens * pas as i32).rem_euclid(n) as usize
    }

    pub fn avancer(&mut self, pas: usize) {
        self.courant = self.voisin(self.courant, pas);
        self.piochee = None;
    }
}
