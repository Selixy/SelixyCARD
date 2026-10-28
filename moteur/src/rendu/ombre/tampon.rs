#[derive(Clone, Debug, PartialEq)]
pub struct TamponProfondeur {
    pub resolution: u32,
    pub donnees: Vec<f32>,
}

impl TamponProfondeur {
    pub fn nouveau(resolution: u32, valeur_initiale: f32) -> Self {
        Self {
            resolution,
            donnees: vec![valeur_initiale; (resolution * resolution) as usize],
        }
    }

    pub fn ecrire(&mut self, x: u32, y: u32, profondeur: f32) {
        if x < self.resolution && y < self.resolution {
            let idx = (y * self.resolution + x) as usize;
            self.donnees[idx] = profondeur;
        }
    }

    pub fn lire(&self, u: f32, v: f32) -> f32 {
        let cu = u.clamp(0.0, 1.0);
        let cv = v.clamp(0.0, 1.0);
        let x = (cu * (self.resolution - 1) as f32).round() as usize;
        let y = (cv * (self.resolution - 1) as f32).round() as usize;
        self.donnees[y * (self.resolution as usize) + x]
    }
}
