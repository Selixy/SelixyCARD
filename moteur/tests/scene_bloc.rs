use moteur::scene::bloc::{SURELEVATION, bloc};
use moteur::scene::geometrie::{PLAT, Pose};
use moteur::scene::selection::FORCE_MAX;
use moteur::scene::{COUCHE_BLOC, HAUTEUR_CARTE, LARGEUR_CARTE, MODE_CARTE, MODE_SILHOUETTE, Selection};

#[test]
fn le_bloc_est_la_carte_extrudee_jusqu_au_sol_sous_la_carte_du_dessus() {
    let sommet = Pose { pos: [-1.4, 0.255, 0.0], lacet: 0.0, tangage: PLAT };
    let instance = bloc(&sommet, [0.0, 0.0, 0.0, MODE_CARTE]);
    let haut = 0.255 - SURELEVATION;
    assert_eq!(instance.forme, [LARGEUR_CARTE, HAUTEUR_CARTE, haut, COUCHE_BLOC]);
    assert_eq!(instance.position_lacet[1], haut);
}

#[test]
fn une_selection_marque_ses_formes_avec_sa_couleur_et_sa_force() {
    let sommet = Pose { pos: [-1.4, 0.255, 0.0], lacet: 0.0, tangage: PLAT };
    let mut selection = Selection::nouvelle([1.0, 0.5, 0.25, 0.9]);
    selection.ajouter_bloc(&sommet);
    selection.ajouter_carte(&sommet, 1.0);
    assert_eq!(selection.formes.len(), 3);
    assert!(selection.formes.iter().all(|f| f.couleur == [1.0, 0.5, 0.25, MODE_SILHOUETTE + 0.9]));
}

#[test]
fn la_force_d_une_selection_est_bornee() {
    let selection = Selection::nouvelle([1.0, 1.0, 1.0, 9.0]);
    assert_eq!(selection.marque()[3], MODE_SILHOUETTE + FORCE_MAX);
}
