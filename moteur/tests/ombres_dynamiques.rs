use moteur::maths::transformer;
use moteur::rendu::ombre::{
    CameraLumiere, ConfigurationOmbres, TamponProfondeur, biais_dynamique,
    coordonnees_projection, filtrage_pcf, matrice_biais, occlusion,
    tester_occlusion,
};
use moteur::rendu::passe_ombre::{
    FormatTextureProfondeur, PasseOmbre,
};
use moteur::scene::Instance;

#[test]
fn test_matrice_vue_projection_et_biais() {
    let cam_gl = CameraLumiere::principale(false);
    let vp_gl = cam_gl.matrice_vue_projection_lumiere();
    let cible_gl = transformer(&vp_gl, cam_gl.cible);
    assert!((cible_gl[0] / cible_gl[3]).abs() < 1e-4);
    assert!((cible_gl[1] / cible_gl[3]).abs() < 1e-4);

    let biais_gl = matrice_biais(false);
    let p_centre = transformer(&biais_gl, [0.0, 0.0, 0.0]);
    assert!((p_centre[0] - 0.5).abs() < 1e-5);
    assert!((p_centre[1] - 0.5).abs() < 1e-5);
    assert!((p_centre[2] - 0.5).abs() < 1e-5);

    let p_min = transformer(&biais_gl, [-1.0, -1.0, -1.0]);
    assert!(p_min[0].abs() < 1e-5);
    assert!(p_min[1].abs() < 1e-5);
    assert!(p_min[2].abs() < 1e-5);

    let p_max = transformer(&biais_gl, [1.0, 1.0, 1.0]);
    assert!((p_max[0] - 1.0).abs() < 1e-5);
    assert!((p_max[1] - 1.0).abs() < 1e-5);
    assert!((p_max[2] - 1.0).abs() < 1e-5);

    let biais_gpu = matrice_biais(true);
    let p_gpu = transformer(&biais_gpu, [-1.0, 1.0, 0.5]);
    assert!(p_gpu[0].abs() < 1e-5);
    assert!((p_gpu[1] - 1.0).abs() < 1e-5);
    assert!((p_gpu[2] - 0.5).abs() < 1e-5);
}

#[test]
fn test_detection_occlusion_directe() {
    let eclaire = occlusion([0.0, 0.0, 0.2], 1.0, 0.001);
    assert_eq!(eclaire, 1.0);

    let hors_champ = occlusion([1000.0, 0.0, 0.0], 0.2, 0.001);
    assert_eq!(hors_champ, 1.0);

    let normale = [0.0, 1.0, 0.0];
    let dir_face = [0.0, 1.0, 0.0];
    let biais_face = biais_dynamique(normale, dir_face, 0.005, 0.001);
    assert!((biais_face - 0.001).abs() < 1e-5);

    let dir_rasante = [1.0, 0.0, 0.0];
    let biais_rasant = biais_dynamique(normale, dir_rasante, 0.005, 0.001);
    assert!((biais_rasant - 0.005).abs() < 1e-5);
}

#[test]
fn test_detection_occlusion_masquee() {
    let cam = CameraLumiere::nouvelle([0.0, 10.0, 0.0], [0.0, 0.0, 0.0], 10.0, 1.0, 20.0, true);
    let coords_obstacle = coordonnees_projection(&cam, [0.0, 5.0, 0.0]).expect("projection obstacle");
    let prof_obstacle = coords_obstacle[2];

    let masque = tester_occlusion(&cam, [0.0, 0.0, 0.0], prof_obstacle, 0.001);
    assert_eq!(masque, 0.0);

    let devant = tester_occlusion(&cam, [0.0, 8.0, 0.0], prof_obstacle, 0.001);
    assert_eq!(devant, 1.0);

    let surface_obstacle = tester_occlusion(&cam, [0.0, 5.0, 0.0], prof_obstacle, 0.001);
    assert_eq!(surface_obstacle, 1.0);
}

#[test]
fn test_filtrage_pcf() {
    let cam = CameraLumiere::nouvelle([0.0, 10.0, 0.0], [0.0, 0.0, 0.0], 10.0, 1.0, 20.0, true);
    let config = ConfigurationOmbres {
        resolution: 32,
        biais_base: 0.005,
        biais_min: 0.001,
        portee_filtre: 2.0,
        zero_un: true,
    };
    let mut tampon = TamponProfondeur::nouveau(32, 1.0);
    for y in 0..32 {
        for x in 0..16 {
            tampon.ecrire(x, y, 0.25);
        }
    }

    let normale = [0.0, 1.0, 0.0];
    let dans_ombre = filtrage_pcf(&cam, [-6.0, 0.0, 0.0], normale, &tampon, &config);
    assert_eq!(dans_ombre, 0.0);

    let dans_lumiere = filtrage_pcf(&cam, [6.0, 0.0, 0.0], normale, &tampon, &config);
    assert_eq!(dans_lumiere, 1.0);

    let sur_frontiere = filtrage_pcf(&cam, [0.0, 0.0, 0.0], normale, &tampon, &config);
    assert!(sur_frontiere > 0.0);
    assert!(sur_frontiere < 1.0);
}

#[test]
fn test_passe_ombre_et_configuration() {
    let defaut = ConfigurationOmbres::par_defaut();
    assert_eq!(defaut.resolution, 1024);
    assert_eq!(defaut.biais_base, 0.005);
    assert_eq!(defaut.biais_min, 0.001);

    let haute = ConfigurationOmbres::haute_qualite();
    assert_eq!(haute.resolution, 2048);

    let cam = CameraLumiere::principale(false);
    let mut passe = PasseOmbre::nouveau(cam, defaut);
    let config_passe = passe.configuration_passe();
    assert!(config_passe.test_profondeur);
    assert!(config_passe.ecriture_profondeur);
    assert_eq!(config_passe.resolution, 1024);
    assert_eq!(config_passe.format_profondeur, FormatTextureProfondeur::Profondeur24);

    let params = passe.preparer_parametres();
    assert_eq!(params.resolution, 1024.0);
    assert_eq!(params.biais_dynamique, 0.005);

    let instance = Instance {
        position_lacet: [0.0, 0.0, 0.0, 0.0],
        couleur: [1.0, 1.0, 1.0, 1.0],
        forme: [1.5, 2.25, 0.0, 0.0],
    };
    passe.ajouter_instance(instance);
    assert_eq!(passe.instances().len(), 1);
}
