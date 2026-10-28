use crate::entree::Entree;
use crate::graphe::GraphePbr;
use crate::noeud::Operation;
use crate::surface::SurfacePbr;
use crate::types::TypeDonnee;
use crate::valeur::Valeur;

pub fn fabriquer_graphe_carte() -> GraphePbr {
    let mut g = GraphePbr::nouveau(SurfacePbr {
        albedo: crate::noeud::IdNoeud(0),
        rugosite: crate::noeud::IdNoeud(0),
        metallique: crate::noeud::IdNoeud(0),
        normale: None,
        relief: None,
        emission: None,
        alpha: None,
    });
    let uv = g.ajouter(Operation::Source(Entree::Uv), TypeDonnee::Vecteur2);
    let forme = g.ajouter(Operation::Source(Entree::FormeInstance), TypeDonnee::Vecteur4);
    let couche = g.ajouter(Operation::Composante(forme, "w".into()), TypeDonnee::Flottant);
    let tex = g.ajouter(Operation::TextureTableau(uv, couche), TypeDonnee::Vecteur4);
    let albedo = g.ajouter(Operation::Composante(tex, "rgb".into()), TypeDonnee::Vecteur3);
    let alpha = g.ajouter(Operation::Composante(tex, "a".into()), TypeDonnee::Flottant);
    let rugosite = g.ajouter(Operation::Source(Entree::Constante(Valeur::Flottant(0.52))), TypeDonnee::Flottant);
    let metallique = g.ajouter(Operation::Source(Entree::Constante(Valeur::Flottant(0.04))), TypeDonnee::Flottant);
    g.surface = SurfacePbr {
        albedo,
        rugosite,
        metallique,
        normale: None,
        relief: None,
        emission: None,
        alpha: Some(alpha),
    };
    g
}
