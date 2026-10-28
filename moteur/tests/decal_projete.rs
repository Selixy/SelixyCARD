use moteur::maths::{identite, inverser, mise_a_echelle, multiplier, rotation_euler, translation};
use moteur::rendu::passe_decal::{PasseDecal, SOMMETS_BOITE, SOMMETS_PAR_BOITE};
use moteur::scene::VolumeDecal;

#[test]
fn test_inversion_matrice() {
    let id = identite();
    let inv_id = inverser(&id).expect("identite inversible");
    assert!((inv_id[0] - 1.0).abs() < 1e-6);

    let t = translation([12.0, -4.5, 8.0]);
    let r = rotation_euler([0.2, -0.4, 0.1]);
    let s = mise_a_echelle([2.0, 3.0, 4.0]);
    let m = multiplier(&multiplier(&t, &r), &s);
    let inv_m = inverser(&m).expect("matrice inversible");
    let produit = multiplier(&m, &inv_m);
    for col in 0..4 {
        for lig in 0..4 {
            let attendu = if col == lig { 1.0 } else { 0.0 };
            let diff = (produit[col * 4 + lig] - attendu).abs();
            assert!(diff < 1e-4, "col {col} lig {lig} diff {diff}");
        }
    }

    let singuliere = [0.0; 16];
    assert!(inverser(&singuliere).is_none());
}

#[test]
fn test_detection_points_volume() {
    let pos = [10.0, 5.0, -2.0];
    let dim = [4.0, 2.0, 6.0];
    let decal = VolumeDecal::composer(pos, [0.0, 0.0, 0.0], dim, 1, 1.0, 0.1);

    assert!(decal.contient(pos));
    assert!(decal.contient([11.8, 5.9, -1.0]));
    assert!(decal.contient([8.2, 4.1, -4.8]));
    assert!(!decal.contient([12.5, 5.0, -2.0]));
    assert!(!decal.contient([10.0, 6.5, -2.0]));
    assert!(!decal.contient([10.0, 5.0, 1.5]));
    assert!(!decal.contient([100.0, 50.0, 0.0]));
}

#[test]
fn test_generation_coordonnees_uv() {
    let decal = VolumeDecal::composer([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [10.0, 2.0, 10.0], 0, 1.0, 0.0);

    let uv_centre = decal.projeter_uv([0.0, 0.0, 0.0]).expect("centre dans le volume");
    assert!((uv_centre[0] - 0.5).abs() < 1e-5);
    assert!((uv_centre[1] - 0.5).abs() < 1e-5);

    let uv_coin_min = decal.projeter_uv([-5.0, 0.0, -5.0]).expect("coin min");
    assert!((uv_coin_min[0] - 0.0).abs() < 1e-5);
    assert!((uv_coin_min[1] - 0.0).abs() < 1e-5);

    let uv_coin_max = decal.projeter_uv([5.0, 0.0, 5.0]).expect("coin max");
    assert!((uv_coin_max[0] - 1.0).abs() < 1e-5);
    assert!((uv_coin_max[1] - 1.0).abs() < 1e-5);

    let uv_quart = decal.projeter_uv([2.5, 0.0, -2.5]).expect("quart");
    assert!((uv_quart[0] - 0.75).abs() < 1e-5);
    assert!((uv_quart[1] - 0.25).abs() < 1e-5);

    assert!(decal.projeter_uv([6.0, 0.0, 0.0]).is_none());
}

#[test]
fn test_attenuation_fondu() {
    let decal = VolumeDecal::composer([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [10.0, 10.0, 10.0], 0, 0.8, 0.2);

    let att_centre = decal.attenuation([0.0, 0.0, 0.0]);
    assert!((att_centre - 0.8).abs() < 1e-5);

    let att_bord = decal.attenuation([4.5, 0.0, 0.0]);
    assert!(att_bord < 0.8);
    assert!(att_bord > 0.0);

    let att_dehors = decal.attenuation([5.5, 0.0, 0.0]);
    assert_eq!(att_dehors, 0.0);
}

#[test]
fn test_passe_decal() {
    let config = PasseDecal::configuration();
    assert!(config.test_profondeur);
    assert!(!config.ecriture_profondeur);
    assert!(config.lecture_profondeur);
    assert_eq!(SOMMETS_PAR_BOITE, 36);
    assert_eq!(SOMMETS_BOITE.len(), 36);

    let mut passe = PasseDecal::nouveau();
    let decal = VolumeDecal::composer([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], 2, 1.0, 0.0);
    passe.ajouter(decal);
    assert_eq!(passe.decals_visibles().len(), 1);

    let params = PasseDecal::preparer_parametres(&identite(), [800.0, 600.0]).expect("parametres");
    assert_eq!(params.resolution, [800.0, 600.0]);
}
