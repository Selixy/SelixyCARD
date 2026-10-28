use moteur::decor::Animation;
use moteur::decor::etat::{Decor, Element, Mobilite};
use moteur::textures::Textures;

fn decor_test() -> Decor {
    Decor {
        vise: [0.0, 0.0],
        distance: 16.0,
        elevation: 1.0,
        elements: vec![
            Element {
                cle: "carte:0:rouge-7-a".into(),
                pos: [2.0, 0.12, 1.0],
                lacet: 0.0,
                tangage: -1.57,
                taille: [1.5, 2.25],
                flotte: 0.2,
                phase: 0.0,
                feuille: false,
                mobilite: Mobilite::Mobile,
            },
            Element {
                cle: "carte:1:vert-5-a".into(),
                pos: [4.0, 0.12, 1.0],
                lacet: 0.0,
                tangage: -1.57,
                taille: [1.5, 2.25],
                flotte: 0.2,
                phase: 1.7,
                feuille: false,
                mobilite: Mobilite::Mobile,
            },
        ],
    }
}

#[test]
fn carte_monte_au_chargement_puis_se_pose() {
    let mut anim = Animation::default();
    anim.definir(decor_test());
    let textures = Textures::default();
    let inst_depart = anim.instances(0.0, &textures);
    let y_depart = inst_depart[1].position_lacet[1];
    let inst_pose = anim.instances(1.5, &textures);
    let y_pose = inst_pose[1].position_lacet[1];
    assert!(y_depart > y_pose + 1.0);
    assert!((y_pose - 0.22).abs() < 0.25);
    let alpha_depart = inst_depart[1].couleur[3];
    let alpha_pose = inst_pose[1].couleur[3];
    assert_eq!(alpha_depart, 0.0);
    assert!((alpha_pose - 1.0).abs() < 0.01);
    let y_ombre_depart = inst_depart[0].position_lacet[1];
    let y_ombre_pose = inst_pose[0].position_lacet[1];
    assert!((y_ombre_depart - 0.006).abs() < 1e-4);
    assert!((y_ombre_pose - 0.006).abs() < 1e-4);
    assert!(inst_depart[0].position_lacet[0] < inst_pose[0].position_lacet[0]);
}

#[test]
fn cascade_echelonnee_entre_cartes() {
    let mut anim = Animation::default();
    anim.definir(decor_test());
    let textures = Textures::default();
    anim.instances(0.12, &textures);
    let inst = anim.instances(0.0, &textures);
    let y_c0 = inst[1].position_lacet[1];
    let y_c1 = inst[3].position_lacet[1];
    assert!(y_c0 < y_c1);
}

#[test]
fn reinitialiser_rejoue_l_apparition() {
    let mut anim = Animation::default();
    anim.definir(decor_test());
    let textures = Textures::default();
    anim.instances(1.5, &textures);
    let y_pose = anim.instances(0.0, &textures)[1].position_lacet[1];
    anim.reinitialiser_apparition();
    let y_reparti = anim.instances(0.0, &textures)[1].position_lacet[1];
    assert!(y_reparti > y_pose + 1.0);
}

#[test]
fn element_statique_au_sol_sans_ombre_projetee() {
    let mut anim = Animation::default();
    anim.definir(Decor {
        vise: [0.0, 0.0],
        distance: 16.0,
        elevation: 1.0,
        elements: vec![Element {
            cle: "trace_fleche".into(),
            pos: [1.0, 0.012, 2.0],
            lacet: 0.0,
            tangage: -1.57,
            taille: [3.4, 1.6],
            flotte: 0.0,
            phase: 0.0,
            feuille: true,
            mobilite: Mobilite::Statique,
        }],
    });
    let instances = anim.instances(0.0, &Textures::default());
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].forme[3], -2.0);
}
