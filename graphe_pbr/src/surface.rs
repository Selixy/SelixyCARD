use serde::{Deserialize, Serialize};
use crate::noeud::IdNoeud;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SurfacePbr {
    pub albedo: IdNoeud,
    pub rugosite: IdNoeud,
    pub metallique: IdNoeud,
    pub normale: Option<IdNoeud>,
    pub relief: Option<IdNoeud>,
    pub emission: Option<IdNoeud>,
    pub alpha: Option<IdNoeud>,
}

impl SurfacePbr {
    pub fn dependances(&self) -> Vec<IdNoeud> {
        let mut deps = vec![self.albedo, self.rugosite, self.metallique];
        if let Some(n) = self.normale { deps.push(n); }
        if let Some(r) = self.relief { deps.push(r); }
        if let Some(e) = self.emission { deps.push(e); }
        if let Some(a) = self.alpha { deps.push(a); }
        deps
    }
}
