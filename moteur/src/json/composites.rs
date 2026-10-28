use super::analyse::Curseur;
use super::valeur::Valeur;

impl Curseur<'_> {
    pub fn liste(&mut self) -> Option<Valeur> {
        self.position += 1;
        let mut elements = Vec::new();
        loop {
            self.blancs();
            match *self.octets.get(self.position)? {
                b']' => {
                    self.position += 1;
                    return Some(Valeur::Liste(elements));
                }
                b',' => self.position += 1,
                _ => elements.push(self.valeur()?),
            }
        }
    }

    pub fn objet(&mut self) -> Option<Valeur> {
        self.position += 1;
        let mut champs = Vec::new();
        loop {
            self.blancs();
            match *self.octets.get(self.position)? {
                b'}' => {
                    self.position += 1;
                    return Some(Valeur::Objet(champs));
                }
                b',' => self.position += 1,
                b'"' => {
                    let cle = self.texte()?;
                    self.blancs();
                    if *self.octets.get(self.position)? != b':' {
                        return None;
                    }
                    self.position += 1;
                    champs.push((cle, self.valeur()?));
                }
                _ => return None,
            }
        }
    }
}
