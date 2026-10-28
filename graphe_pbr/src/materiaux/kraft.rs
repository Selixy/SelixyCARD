use crate::entree::Entree;
use crate::graphe::GraphePbr;
use crate::noeud::Operation;
use crate::surface::SurfacePbr;
use crate::types::TypeDonnee;
use crate::valeur::Valeur;

pub fn fabriquer_graphe_kraft() -> GraphePbr {
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
    let f_fibre = g.ajouter(Operation::Source(Entree::Constante(Valeur::Vecteur2([9.0, 14.0]))), TypeDonnee::Vecteur2);
    let p_fibre = g.ajouter(Operation::Multiplication(p, f_fibre), TypeDonnee::Vecteur2);
    let fibre = g.ajouter(Operation::BruitPerlin(p_fibre), TypeDonnee::Flottant);
    let c_sombre = g.ajouter(Operation::Source(Entree::Constante(Valeur::Vecteur3([0.65, 0.50, 0.35]))), TypeDonnee::Vecteur3);
    let c_clair = g.ajouter(Operation::Source(Entree::Constante(Valeur::Vecteur3([0.76, 0.62, 0.46]))), TypeDonnee::Vecteur3);
    let albedo = g.ajouter(Operation::Melange(c_sombre, c_clair, fibre), TypeDonnee::Vecteur3);
    let rugosite = g.ajouter(Operation::Source(Entree::Constante(Valeur::Flottant(0.88))), TypeDonnee::Flottant);
    let metallique = g.ajouter(Operation::Source(Entree::Constante(Valeur::Flottant(0.01))), TypeDonnee::Flottant);
    let relief = g.ajouter(Operation::Multiplication(fibre, rugosite), TypeDonnee::Flottant);
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
