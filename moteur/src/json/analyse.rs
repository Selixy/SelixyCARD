use super::valeur::Valeur;

pub struct Curseur<'a> {
    pub octets: &'a [u8],
    pub position: usize,
}

pub fn analyser(source: &str) -> Option<Valeur> {
    let mut curseur = Curseur { octets: source.as_bytes(), position: 0 };
    let valeur = curseur.valeur()?;
    curseur.blancs();
    (curseur.position == curseur.octets.len()).then_some(valeur)
}

impl Curseur<'_> {
    pub fn blancs(&mut self) {
        while self.octets.get(self.position).is_some_and(|b| b.is_ascii_whitespace()) {
            self.position += 1;
        }
    }

    pub fn valeur(&mut self) -> Option<Valeur> {
        self.blancs();
        match *self.octets.get(self.position)? {
            b'"' => self.texte().map(Valeur::Texte),
            b'[' => self.liste(),
            b'{' => self.objet(),
            _ => {
                for (mot, valeur) in [("null", Valeur::Nul), ("true", Valeur::Nombre(1.0)), ("false", Valeur::Nombre(0.0))] {
                    if self.octets[self.position..].starts_with(mot.as_bytes()) {
                        self.position += mot.len();
                        return Some(valeur);
                    }
                }
                self.nombre().map(Valeur::Nombre)
            }
        }
    }

    pub fn texte(&mut self) -> Option<String> {
        self.position += 1;
        let mut sortie = Vec::new();
        loop {
            let octet = *self.octets.get(self.position)?;
            self.position += 1;
            match octet {
                b'"' => return String::from_utf8(sortie).ok(),
                b'\\' => {
                    let echappe = *self.octets.get(self.position)?;
                    self.position += 1;
                    sortie.push(match echappe {
                        b'n' => b'\n',
                        b't' => b'\t',
                        b'r' => b'\r',
                        autre => autre,
                    });
                }
                _ => sortie.push(octet),
            }
        }
    }

    pub fn nombre(&mut self) -> Option<f64> {
        let debut = self.position;
        while self.octets.get(self.position).is_some_and(|b| b"+-.eE0123456789".contains(b)) {
            self.position += 1;
        }
        std::str::from_utf8(&self.octets[debut..self.position]).ok()?.parse().ok()
    }
}
