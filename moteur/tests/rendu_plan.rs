use moteur::rendu::{Etape, SOMMETS_BLOC, SOMMETS_PAR_CARTE, planifier};
use moteur::scene::geometrie::{PLAT, Pose};
use moteur::scene::{MODE_CARTE, Selection, bloc::bloc, carte};

#[test]
fn le_plan_range_scene_puis_selections_avec_les_blocs_a_part() {
    let pose = Pose { pos: [0.0, 0.2, 0.0], lacet: 0.0, tangage: PLAT };
    let instances = [carte(&pose, 1.0, 3.0), bloc(&pose, [0.0, 0.0, 0.0, MODE_CARTE])];
    let mut selection = Selection::nouvelle([1.0; 4]);
    selection.ajouter_bloc(&pose);
    selection.ajouter_carte(&pose, 1.0);
    let (tout, traces) = planifier(&instances, &[selection]);
    assert_eq!(tout.len(), 2 + 3);
    let etapes: Vec<(bool, u32)> = traces.iter().map(|t| (t.etape == Etape::Selection, t.sommets)).collect();
    assert_eq!(etapes, vec![(false, SOMMETS_PAR_CARTE), (false, SOMMETS_BLOC), (true, SOMMETS_PAR_CARTE), (true, SOMMETS_BLOC)]);
}
