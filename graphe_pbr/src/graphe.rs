use serde::{Deserialize, Serialize};
use crate::noeud::{DescriptionNoeud, IdNoeud, Operation};
use crate::surface::SurfacePbr;
use crate::types::TypeDonnee;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphePbr {
    noeuds: Vec<DescriptionNoeud>,
    pub surface: SurfacePbr,
}

impl GraphePbr {
    pub fn nouveau(surface: SurfacePbr) -> Self {
        Self { noeuds: Vec::new(), surface }
    }

    pub fn ajouter(&mut self, operation: Operation, type_sortie: TypeDonnee) -> IdNoeud {
        let id = IdNoeud(self.noeuds.len());
        self.noeuds.push(DescriptionNoeud { id, operation, type_sortie });
        id
    }

    pub fn noeud(&self, id: IdNoeud) -> Option<&DescriptionNoeud> {
        self.noeuds.get(id.0)
    }

    pub fn tri_topologique(&self) -> Result<Vec<IdNoeud>, &'static str> {
        let mut resultat = Vec::new();
        let mut visites = vec![false; self.noeuds.len()];
        let mut en_cours = vec![false; self.noeuds.len()];
        for racine in self.surface.dependances() {
            self.parcourir(racine, &mut visites, &mut en_cours, &mut resultat)?;
        }
        Ok(resultat)
    }

    fn parcourir(&self, id: IdNoeud, visites: &mut [bool], en_cours: &mut [bool], out: &mut Vec<IdNoeud>) -> Result<(), &'static str> {
        if id.0 >= self.noeuds.len() { return Err("identifiant de noeud invalide"); }
        if en_cours[id.0] { return Err("cycle detecte dans le graphe"); }
        if visites[id.0] { return Ok(()); }
        en_cours[id.0] = true;
        for dep in self.noeuds[id.0].dependances() {
            self.parcourir(dep, visites, en_cours, out)?;
        }
        en_cours[id.0] = false;
        visites[id.0] = true;
        out.push(id);
        Ok(())
    }
}
