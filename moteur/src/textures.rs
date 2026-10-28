use std::collections::HashMap;

pub const COUCHES: u32 = 128;

#[derive(Default)]
pub struct Textures {
    couches: HashMap<String, u32>,
}

impl Textures {
    pub fn attribuer(&mut self, cle: &str) -> Option<u32> {
        if let Some(couche) = self.couches.get(cle) {
            return Some(*couche);
        }
        let suivante = self.couches.len() as u32;
        (suivante < COUCHES).then(|| {
            self.couches.insert(cle.to_owned(), suivante);
            suivante
        })
    }

    pub fn couche(&self, cle: &str) -> Option<u32> {
        self.couches.get(cle).copied()
    }
}
