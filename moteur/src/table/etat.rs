use crate::json::{self, Valeur};

#[derive(Clone, PartialEq)]
pub struct EtatTable {
    pub main: Vec<String>,
    pub defausse: Option<String>,
    pub adversaires: Vec<u32>,
    pub pioche: u32,
    pub jouables: Vec<usize>,
    pub premier_plan: i32,
    pub courant: i32,
    pub couleur: Option<String>,
}

impl Default for EtatTable {
    fn default() -> Self {
        EtatTable { main: Vec::new(), defausse: None, adversaires: Vec::new(), pioche: 0, jouables: Vec::new(), premier_plan: -1, courant: -1, couleur: None }
    }
}

impl EtatTable {
    pub fn depuis_json(source: &str) -> Option<EtatTable> {
        let racine = json::analyser(source)?;
        let liste = |nom: &str| racine.champ(nom).map_or(&[][..], Valeur::liste);
        let entier = |nom: &str| racine.champ(nom).and_then(Valeur::nombre).map_or(-1, |n| n as i32);
        Some(EtatTable {
            main: liste("main").iter().filter_map(|v| v.texte().map(str::to_owned)).collect(),
            defausse: racine.champ("defausse").and_then(Valeur::texte).map(str::to_owned),
            adversaires: liste("adversaires").iter().filter_map(|v| v.nombre().map(|n| n as u32)).collect(),
            pioche: racine.champ("pioche").and_then(Valeur::nombre).unwrap_or(0.0) as u32,
            jouables: liste("jouables").iter().filter_map(|v| v.nombre().map(|n| n as usize)).collect(),
            premier_plan: entier("premier_plan"),
            courant: entier("courant"),
            couleur: racine.champ("couleur").and_then(Valeur::texte).map(str::to_owned),
        })
    }

    pub fn est_au_premier_plan(&self, indice: usize) -> bool {
        self.premier_plan == indice as i32
    }
}
