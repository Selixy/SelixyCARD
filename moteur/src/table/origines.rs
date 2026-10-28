use super::compose::identifiant;
use super::disposition::{CARTES_PAR_PILE, pose_pioche};
use super::etat::EtatTable;
use super::eventail::CARTES_VISIBLES_ADVERSAIRE;
use super::mouvement::Mouvements;
use super::souffle::Souffles;

pub fn preparer(ancien: &EtatTable, nouveau: &EtatTable, id_defausse: u32, mouvements: &mut Mouvements, souffles: &mut Souffles) {
    if nouveau.defausse != ancien.defausse
        && let Some(cle) = nouveau.defausse.as_deref()
        && let Some(origine) = origine_jouee(ancien, cle, mouvements)
    {
        mouvements.apparaitre(id_defausse, origine);
    }
    let sommet = pose_pioche(ancien.pioche.min(CARTES_PAR_PILE).saturating_sub(1));
    let piochees: Vec<&String> = nouveau.main.iter().filter(|c| !ancien.main.contains(c)).collect();
    for cle in &piochees {
        mouvements.apparaitre(identifiant(cle), sommet);
    }
    if !piochees.is_empty() && piochees.len() <= 4 {
        souffles.impact(sommet.pos, 0.45, super::souffle::CRAIE);
    }
}

fn origine_jouee(ancien: &EtatTable, cle: &str, mouvements: &Mouvements) -> Option<crate::scene::geometrie::Pose> {
    if ancien.main.iter().any(|c| c == cle) {
        return mouvements.pose(identifiant(cle));
    }
    let adversaire = usize::try_from(ancien.courant - 1).ok()?;
    let visibles = ancien.adversaires.get(adversaire)?.min(&CARTES_VISIBLES_ADVERSAIRE).saturating_sub(1);
    mouvements.pose(4000 + adversaire as u32 * 100 + visibles)
}
