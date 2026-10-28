mod contexte;
mod lueur;
mod passe;
mod pipeline;
mod tampons;
mod textures;

pub use contexte::Contexte;
use lueur::Lueur;
pub use pipeline::Pipelines;
pub use tampons::Tampons;
pub use textures::TexturesWgpu;

use super::{CameraLumiere, Etape, Rendu, flottants_instances, planifier};
use crate::maths::Mat4;
use crate::scene::{COUCHE_BLOC, Instance, Selection};
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

pub struct RenduWgpu {
    contexte: Contexte,
    tampons: Tampons,
    textures: TexturesWgpu,
    pipelines: Pipelines,
    camera_lumiere: CameraLumiere,
    lueur: Lueur,
}

impl RenduWgpu {
    pub async fn nouveau(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let contexte = Contexte::initialiser(canvas).await.map_err(|e| JsValue::from_str(&e))?;
        let tampons = Tampons::creer(&contexte.device);
        let mut textures = TexturesWgpu::creer(&contexte.device);
        textures.redimensionner(&contexte.device, canvas.width(), canvas.height());
        let pipelines = Pipelines::creer(&contexte.device, contexte.config.format, &tampons, &textures);
        let camera_lumiere = CameraLumiere::principale(true);
        let mut lueur = Lueur::creer(&contexte.device, contexte.config.format);
        lueur.redimensionner(&contexte.device, &contexte.file, canvas.width(), canvas.height());
        Ok(Self { contexte, tampons, textures, pipelines, camera_lumiere, lueur })
    }
}

impl Rendu for RenduWgpu {
    fn profondeur_zero_un(&self) -> bool {
        true
    }

    fn redimensionner(&mut self, largeur: u32, hauteur: u32) {
        self.contexte.redimensionner(largeur, hauteur);
        self.textures.redimensionner(&self.contexte.device, largeur, hauteur);
        self.lueur.redimensionner(&self.contexte.device, &self.contexte.file, largeur, hauteur);
    }

    fn dessiner(&mut self, vue_proj: &Mat4, oeil: [f32; 3], instances: &[Instance], selections: &[Selection]) {
        let (Some((_, vue_profondeur)), Ok(cible)) = (&self.textures.profondeur, self.contexte.obtenir_texture()) else { return };
        let vue_cible = cible.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let vue_proj_lum = self.camera_lumiere.matrice_vue_projection_lumiere();
        let ombrables: Vec<Instance> = instances.iter().filter(|i| i.forme[3] > -1.5 && i.forme[3] != -3.0).copied().collect();
        let nb_ombrables = ombrables.len() as u32;
        let blocs: Vec<Instance> = instances.iter().filter(|i| i.forme[3] == COUCHE_BLOC).copied().collect();
        let nb_blocs = blocs.len() as u32;
        let ombrables: Vec<Instance> = ombrables.into_iter().chain(blocs).collect();
        let donnees_ombres = flottants_instances(&ombrables);
        self.tampons.ecrire_lumiere(&self.contexte.file, &vue_proj_lum);
        self.tampons.ecrire_instances_ombres(&self.contexte.file, donnees_ombres);
        let (tout, traces) = planifier(instances, selections);
        let donnees_instances = flottants_instances(&tout);
        self.tampons.ecrire_globales(&self.contexte.file, vue_proj, &vue_proj_lum, oeil);
        self.tampons.ecrire_instances(&self.contexte.file, donnees_instances);
        let mut encodeur = self.contexte.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("encodeur_rendu") });
        passe::passer_ombres(&mut encodeur, &self.textures.vue_ombre, &self.pipelines.pipeline_ombre, &self.pipelines.groupe_ombre, &self.tampons.instances_ombres, nb_ombrables, nb_blocs);
        passe::passer_principale(&mut encodeur, &vue_cible, vue_profondeur, &self.pipelines.pipelines_carte, &self.pipelines.groupe_carte, &self.tampons.instances, &traces);
        if let (Some(masque), true) = (self.lueur.vue_masque(), traces.iter().any(|t| t.etape == Etape::Selection)) {
            passe::passer_masque(&mut encodeur, masque, vue_profondeur, &self.pipelines.pipelines_carte, &self.pipelines.groupe_carte, &self.tampons.instances, &traces);
            self.lueur.composer(&mut encodeur, &vue_cible);
        }
        self.contexte.file.submit(std::iter::once(encodeur.finish()));
        cible.present();
    }

    fn charger_texture(&mut self, couche: u32, image: &JsValue) -> Result<(), JsValue> {
        self.textures.charger_couche(&self.contexte.file, couche, image)
    }
}
