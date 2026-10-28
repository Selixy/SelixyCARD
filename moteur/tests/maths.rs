use moteur::maths::{multiplier, perspective, regarder};

#[test]
fn identite_neutre() {
    let i = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    let p = perspective(1.0, 1.5, 0.1, 100.0, true);
    assert_eq!(multiplier(&i, &p), p);
}

#[test]
fn regarder_place_la_cible_devant() {
    let v = regarder([0.0, 0.0, 5.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    assert!((v[14] + 5.0).abs() < 1e-6);
}
