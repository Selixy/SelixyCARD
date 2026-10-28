use super::sets::SourceSets;
use crate::sets::depot_lecture;
use commun::uno::{Carte, emplacements};
use sqlx::PgPool;
use tokio::runtime::Handle;
use uuid::Uuid;

pub struct SetsPostgres {
    pub bdd: PgPool,
}

impl SourceSets for SetsPostgres {
    fn charger(&self, id: &str) -> Option<Vec<Carte>> {
        if id.is_empty() || id == "defaut" || id == "default" {
            return Some(emplacements());
        }
        let id = Uuid::parse_str(id).ok()?;
        let set = tokio::task::block_in_place(|| Handle::current().block_on(depot_lecture::lire(&self.bdd, id)))
            .ok()??;
        serde_json::from_value(serde_json::to_value(set.cartes).ok()?).ok()
    }
}
