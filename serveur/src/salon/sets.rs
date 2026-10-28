use commun::uno::{Carte, emplacements};

pub trait SourceSets: Send + Sync {
    fn charger(&self, id: &str) -> Option<Vec<Carte>>;
}

pub struct SetParDefaut;

impl SourceSets for SetParDefaut {
    fn charger(&self, _id: &str) -> Option<Vec<Carte>> {
        Some(emplacements())
    }
}
