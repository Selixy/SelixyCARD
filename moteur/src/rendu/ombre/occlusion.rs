use super::camera::CameraLumiere;
use super::configuration::{ConfigurationOmbres, biais_dynamique};
use super::tampon::TamponProfondeur;
use crate::maths::transformer;

pub fn coordonnees_projection(
    camera: &CameraLumiere,
    position_monde: [f32; 3],
) -> Option<[f32; 3]> {
    let vp = camera.matrice_vue_projection_lumiere();
    let clip = transformer(&vp, position_monde);
    if clip[3].abs() < 1e-7 {
        return None;
    }
    let ndc_x = clip[0] / clip[3];
    let ndc_y = clip[1] / clip[3];
    let ndc_z = clip[2] / clip[3];
    let u = ndc_x * 0.5 + 0.5;
    let v = ndc_y * 0.5 + 0.5;
    let z = if camera.zero_un {
        ndc_z
    } else {
        ndc_z * 0.5 + 0.5
    };
    Some([u, v, z])
}

pub fn tester_occlusion(
    camera: &CameraLumiere,
    position_monde: [f32; 3],
    profondeur_lumiere: f32,
    biais: f32,
) -> f32 {
    let Some(coords) = coordonnees_projection(camera, position_monde) else {
        return 1.0;
    };
    if coords[0] < 0.0 || coords[0] > 1.0 || coords[1] < 0.0 || coords[1] > 1.0 || coords[2] > 1.0 {
        return 1.0;
    }
    if coords[2] - biais > profondeur_lumiere {
        0.0
    } else {
        1.0
    }
}

pub fn occlusion(position_monde: [f32; 3], profondeur_lumiere: f32, biais: f32) -> f32 {
    let camera = CameraLumiere::principale(false);
    tester_occlusion(&camera, position_monde, profondeur_lumiere, biais)
}

pub fn filtrage_pcf(
    camera: &CameraLumiere,
    position_monde: [f32; 3],
    normale: [f32; 3],
    tampon: &TamponProfondeur,
    config: &ConfigurationOmbres,
) -> f32 {
    let Some(coords) = coordonnees_projection(camera, position_monde) else {
        return 1.0;
    };
    if coords[0] < 0.0 || coords[0] > 1.0 || coords[1] < 0.0 || coords[1] > 1.0 || coords[2] > 1.0 {
        return 1.0;
    }
    let dir = camera.direction();
    let dir_inv = [-dir[0], -dir[1], -dir[2]];
    let biais = biais_dynamique(normale, dir_inv, config.biais_base, config.biais_min);
    let texel = config.portee_filtre / (tampon.resolution as f32);
    let mut somme = 0.0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            let u = coords[0] + (dx as f32) * texel;
            let v = coords[1] + (dy as f32) * texel;
            let prof = tampon.lire(u, v);
            if coords[2] - biais <= prof {
                somme += 1.0;
            }
        }
    }
    somme / 9.0
}
