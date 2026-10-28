use crate::json::{self, Valeur};
use crate::scene::geometrie::PLAT;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mobilite {
    Statique,
    Mobile,
}

#[derive(Clone)]
pub struct Element {
    pub cle: String,
    pub pos: [f32; 3],
    pub lacet: f32,
    pub tangage: f32,
    pub taille: [f32; 2],
    pub flotte: f32,
    pub phase: f32,
    pub feuille: bool,
    pub mobilite: Mobilite,
}

impl Element {
    pub fn est_mobile(&self) -> bool {
        self.mobilite == Mobilite::Mobile
    }
}

#[derive(Clone, Default)]
pub struct Decor {
    pub vise: [f32; 2],
    pub distance: f32,
    pub elevation: f32,
    pub elements: Vec<Element>,
}

fn nombre(v: &Valeur, nom: &str, defaut: f32) -> f32 {
    v.champ(nom).and_then(Valeur::nombre).map_or(defaut, |n| n as f32)
}

fn element(v: &Valeur) -> Option<Element> {
    let cle = v.champ("cle")?.texte()?.to_owned();
    let flotte = nombre(v, "flotte", 0.0);
    let feuille = nombre(v, "feuille", 0.0) > 0.5;
    let defaut_mobile = if flotte > 0.0 || !feuille { 1.0 } else { 0.0 };
    let mobilite = if nombre(v, "mobile", defaut_mobile) > 0.5 {
        Mobilite::Mobile
    } else {
        Mobilite::Statique
    };
    let y = nombre(v, "y", 0.03);
    let y_garanti = if feuille && flotte <= 0.0 { y.max(0.015) } else { y };
    Some(Element {
        cle,
        pos: [nombre(v, "x", 0.0), y_garanti, nombre(v, "z", 0.0)],
        lacet: nombre(v, "lacet", 0.0),
        tangage: nombre(v, "tangage", PLAT),
        taille: [nombre(v, "l", 1.5), nombre(v, "h", 2.25)],
        flotte,
        phase: nombre(v, "phase", 0.0),
        feuille,
        mobilite,
    })
}

impl Decor {
    pub fn depuis_json(source: &str) -> Option<Decor> {
        let racine = json::analyser(source)?;
        let vise = racine.champ("vise").map_or(&[][..], Valeur::liste);
        let coord = |i: usize| vise.get(i).and_then(Valeur::nombre).unwrap_or(0.0) as f32;
        Some(Decor {
            vise: [coord(0), coord(1)],
            distance: nombre(&racine, "distance", 16.0),
            elevation: nombre(&racine, "elevation", 1.0),
            elements: racine.champ("elements")?.liste().iter().filter_map(element).collect(),
        })
    }
}
