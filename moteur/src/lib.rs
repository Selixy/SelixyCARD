pub mod decor;
pub mod json;
pub mod maths;
pub mod rendu;
pub mod scene;
pub mod table;
pub mod textures;
mod api_decor;
mod api_table;
mod boucle;

use rendu::Rendu;
use textures::Textures;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct Moteur {
    rendu: Box<dyn Rendu>,
    table: table::Table,
    decor: decor::Animation,
    textures: Textures,
    aspect: f32,
    largeur: f32,
    hauteur: f32,
    survol: i32,
    theme: f32,
}

#[wasm_bindgen]
impl Moteur {
    pub async fn creer(canvas: HtmlCanvasElement) -> Result<Moteur, JsValue> {
        let rendu = rendu::wgpu::RenduWgpu::nouveau(&canvas).await?;
        let mut moteur = Self::avec(Box::new(rendu));
        moteur.redimensionner(canvas.width(), canvas.height());
        Ok(moteur)
    }

    pub async fn webgl2(canvas: HtmlCanvasElement) -> Result<Moteur, JsValue> {
        Self::creer(canvas).await
    }

    pub async fn webgpu(canvas: HtmlCanvasElement, _device: JsValue, _format: &str) -> Result<Moteur, JsValue> {
        Self::creer(canvas).await
    }

    fn avec(rendu: Box<dyn Rendu>) -> Moteur {
        Moteur { rendu, table: Default::default(), decor: Default::default(), textures: Default::default(), aspect: 1.0, largeur: 1.0, hauteur: 1.0, survol: -1, theme: scene::THEME_ARDOISE }
    }

    pub fn redimensionner(&mut self, largeur: u32, hauteur: u32) {
        self.largeur = largeur.max(1) as f32;
        self.hauteur = hauteur.max(1) as f32;
        self.aspect = self.largeur / self.hauteur;
        self.rendu.redimensionner(largeur, hauteur);
    }
}
