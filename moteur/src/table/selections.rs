use super::disposition::{Cible, Face};
use super::etat::EtatTable;
use super::lueurs;
use crate::scene::Selection;
use crate::scene::geometrie::Pose;
use std::collections::BTreeMap;

pub fn marquer(selections: &mut BTreeMap<u32, Selection>, etat: &EtatTable, cible: &Cible, pose: &Pose, survol: i32) {
    let Some(groupe) = lueurs::groupe(etat, cible, survol) else { return };
    let selection = selections.entry(groupe).or_insert_with(|| Selection::nouvelle(lueurs::couleur(etat, groupe)));
    if cible.face == Face::Bloc {
        selection.ajouter_bloc(pose);
    }
    selection.ajouter_carte(pose, cible.echelle);
}
