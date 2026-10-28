use moteur::scene::geometrie::{PLAT, Pose};
use moteur::table::trajet::Trajet;

fn pose(x: f32, y: f32) -> Pose {
    Pose { pos: [x, y, 0.0], lacet: 0.0, tangage: PLAT }
}

#[test]
fn un_trajet_monte_en_arc_puis_atterrit_sur_la_cible() {
    let mut t = Trajet::nouveau(pose(0.0, 1.0), pose(6.0, 0.0), false);
    let (milieu, _) = (0..10).map(|_| t.avancer(0.03)).last().unwrap();
    assert!(milieu.pos[1] > 1.0);
    let (fin, fini) = (0..100).map(|_| t.avancer(0.03)).last().unwrap();
    assert!(fini);
    assert_eq!(fin.pos, [6.0, 0.0, 0.0]);
}

#[test]
fn la_carte_jouee_accelere_jusqu_a_l_impact() {
    let mut t = Trajet::nouveau(pose(0.0, 1.0), pose(6.0, 0.0), false);
    let positions: Vec<f32> = (0..40).map(|_| t.avancer(0.01).0.pos[0]).collect();
    let pas: Vec<f32> = positions.windows(2).map(|w| w[1] - w[0]).filter(|d| *d > 0.0).collect();
    let (milieu, fin) = (pas[pas.len() / 2], pas[pas.len() - 1]);
    assert!(fin > milieu * 3.0, "pas au milieu {milieu}, à la fin {fin}");
}
