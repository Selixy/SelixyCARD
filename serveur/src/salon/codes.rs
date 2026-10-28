use super::registre::Registre;
use std::time::Duration;

const ALPHABET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";
const DELAI_ABANDON: Duration = Duration::from_secs(600);

impl Registre {
    pub fn nouveau_code(&mut self) -> String {
        loop {
            let code: String = (0..5).map(|_| ALPHABET[self.alea.borne(ALPHABET.len())] as char).collect();
            if !self.salons.contains_key(&code) {
                return code;
            }
        }
    }

    pub fn purger(&mut self) {
        self.salons.retain(|_, salon| salon.vide_depuis.is_none_or(|depuis| depuis.elapsed() < DELAI_ABANDON));
    }
}
