use super::ombre::{CameraLumiere, ConfigurationOmbres, matrice_biais};
use crate::maths::Mat4;
use crate::scene::Instance;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FormatTextureProfondeur {
    Profondeur16,
    Profondeur24,
    Profondeur32Flottant,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConfigurationPasseOmbre {
    pub test_profondeur: bool,
    pub ecriture_profondeur: bool,
    pub resolution: u32,
    pub format_profondeur: FormatTextureProfondeur,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParametresOmbre {
    pub vue_proj_lumiere: Mat4,
    pub matrice_biais: Mat4,
    pub resolution: f32,
    pub biais_dynamique: f32,
}

pub struct PasseOmbre {
    pub camera_lumiere: CameraLumiere,
    pub configuration: ConfigurationOmbres,
    pub instances_ombrables: Vec<Instance>,
}

impl PasseOmbre {
    pub fn nouveau(camera_lumiere: CameraLumiere, configuration: ConfigurationOmbres) -> Self {
        Self {
            camera_lumiere,
            configuration,
            instances_ombrables: Vec::new(),
        }
    }

    pub fn configuration_passe(&self) -> ConfigurationPasseOmbre {
        ConfigurationPasseOmbre {
            test_profondeur: true,
            ecriture_profondeur: true,
            resolution: self.configuration.resolution,
            format_profondeur: FormatTextureProfondeur::Profondeur24,
        }
    }

    pub fn preparer_parametres(&self) -> ParametresOmbre {
        let vue_proj_lumiere = self.camera_lumiere.matrice_vue_projection_lumiere();
        let biais = matrice_biais(self.camera_lumiere.zero_un);
        ParametresOmbre {
            vue_proj_lumiere,
            matrice_biais: biais,
            resolution: self.configuration.resolution as f32,
            biais_dynamique: self.configuration.biais_base,
        }
    }

    pub fn ajouter_instance(&mut self, instance: Instance) {
        self.instances_ombrables.push(instance);
    }

    pub fn instances(&self) -> &[Instance] {
        &self.instances_ombrables
    }
}
