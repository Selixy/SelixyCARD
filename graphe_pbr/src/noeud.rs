use serde::{Deserialize, Serialize};
use crate::entree::Entree;
use crate::types::TypeDonnee;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdNoeud(pub usize);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    Source(Entree),
    Addition(IdNoeud, IdNoeud),
    Soustraction(IdNoeud, IdNoeud),
    Multiplication(IdNoeud, IdNoeud),
    Division(IdNoeud, IdNoeud),
    Puissance(IdNoeud, IdNoeud),
    Melange(IdNoeud, IdNoeud, IdNoeud),
    Borne(IdNoeud, IdNoeud, IdNoeud),
    Lissage(IdNoeud, IdNoeud, IdNoeud),
    ProduitScalaire(IdNoeud, IdNoeud),
    Longueur(IdNoeud),
    Normaliser(IdNoeud),
    Absolu(IdNoeud),
    Sinus(IdNoeud),
    Cosinus(IdNoeud),
    Exponentielle(IdNoeud),
    Minimum(IdNoeud, IdNoeud),
    Maximum(IdNoeud, IdNoeud),
    Seuil(IdNoeud, IdNoeud),
    TextureTableau(IdNoeud, IdNoeud),
    BruitGrain(IdNoeud),
    BruitPerlin(IdNoeud),
    Composante(IdNoeud, String),
    AssemblerVecteur3(IdNoeud, IdNoeud, IdNoeud),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DescriptionNoeud {
    pub id: IdNoeud,
    pub operation: Operation,
    pub type_sortie: TypeDonnee,
}

impl DescriptionNoeud {
    pub fn dependances(&self) -> Vec<IdNoeud> {
        match &self.operation {
            Operation::Source(_) => vec![],
            Operation::Addition(a, b)
            | Operation::Soustraction(a, b)
            | Operation::Multiplication(a, b)
            | Operation::Division(a, b)
            | Operation::Puissance(a, b)
            | Operation::ProduitScalaire(a, b)
            | Operation::Minimum(a, b)
            | Operation::Maximum(a, b)
            | Operation::Seuil(a, b)
            | Operation::TextureTableau(a, b) => vec![*a, *b],
            Operation::Melange(a, b, c)
            | Operation::Borne(a, b, c)
            | Operation::Lissage(a, b, c)
            | Operation::AssemblerVecteur3(a, b, c) => vec![*a, *b, *c],
            Operation::Longueur(a)
            | Operation::Normaliser(a)
            | Operation::Absolu(a)
            | Operation::Sinus(a)
            | Operation::Cosinus(a)
            | Operation::Exponentielle(a)
            | Operation::BruitGrain(a)
            | Operation::BruitPerlin(a)
            | Operation::Composante(a, _) => vec![*a],
        }
    }
}
