pub enum Valeur {
    Nul,
    Nombre(f64),
    Texte(String),
    Liste(Vec<Valeur>),
    Objet(Vec<(String, Valeur)>),
}

impl Valeur {
    pub fn champ(&self, cle: &str) -> Option<&Valeur> {
        match self {
            Valeur::Objet(champs) => champs.iter().find(|(k, _)| k == cle).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn liste(&self) -> &[Valeur] {
        match self {
            Valeur::Liste(elements) => elements,
            _ => &[],
        }
    }

    pub fn texte(&self) -> Option<&str> {
        match self {
            Valeur::Texte(t) => Some(t),
            _ => None,
        }
    }

    pub fn nombre(&self) -> Option<f64> {
        match self {
            Valeur::Nombre(n) => Some(*n),
            _ => None,
        }
    }
}
