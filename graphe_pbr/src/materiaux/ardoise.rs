use crate::entree::Entree;
use crate::graphe::GraphePbr;
use crate::noeud::Operation;
use crate::surface::SurfacePbr;
use crate::types::TypeDonnee;
use crate::valeur::Valeur;

pub fn fabriquer_graphe_ardoise() -> GraphePbr {
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
    let ech = g.ajouter(Operation::Source(Entree::Constante(Valeur::Vecteur2([130.0, 86.0]))), TypeDonnee::Vecteur2);
    let p = g.ajouter(Operation::Multiplication(uv, ech), TypeDonnee::Vecteur2);
    let f_pierre = g.ajouter(Operation::Source(Entree::Constante(Valeur::Vecteur2([0.35, 0.35]))), TypeDonnee::Vecteur2);
    let p_pierre = g.ajouter(Operation::Multiplication(p, f_pierre), TypeDonnee::Vecteur2);
    let bruit_pierre = g.ajouter(Operation::BruitPerlin(p_pierre), TypeDonnee::Flottant);
    let grain = g.ajouter(Operation::BruitGrain(p), TypeDonnee::Flottant);
    let mod_grain = g.ajouter(Operation::Multiplication(grain, bruit_pierre), TypeDonnee::Flottant);
    let c_base = g.ajouter(Operation::Source(Entree::Constante(Valeur::Vecteur3([0.17, 0.19, 0.22]))), TypeDonnee::Vecteur3);
    let c_clair = g.ajouter(Operation::Source(Entree::Constante(Valeur::Vecteur3([0.22, 0.24, 0.27]))), TypeDonnee::Vecteur3);
    let albedo = g.ajouter(Operation::Melange(c_base, c_clair, mod_grain), TypeDonnee::Vecteur3);
    let rug_min = g.ajouter(Operation::Source(Entree::Constante(Valeur::Flottant(0.80))), TypeDonnee::Flottant);
    let rug_max = g.ajouter(Operation::Source(Entree::Constante(Valeur::Flottant(0.96))), TypeDonnee::Flottant);
    let rugosite = g.ajouter(Operation::Melange(rug_min, rug_max, bruit_pierre), TypeDonnee::Flottant);
    let metallique = g.ajouter(Operation::Source(Entree::Constante(Valeur::Flottant(0.02))), TypeDonnee::Flottant);
    let relief = g.ajouter(Operation::Multiplication(grain, rugosite), TypeDonnee::Flottant);
    g.surface = SurfacePbr {
        albedo,
        rugosite,
        metallique,
        normale: None,
        relief: Some(relief),
        emission: None,
        alpha: None,
    };
    g
}
