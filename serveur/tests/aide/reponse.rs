use axum::http::{HeaderMap, header};
use serde_json::Value;

pub struct Reponse {
    pub statut: u16,
    pub entetes: HeaderMap,
    pub octets: Vec<u8>,
}

impl Reponse {
    pub fn json(&self) -> Value {
        serde_json::from_slice(&self.octets).unwrap_or(Value::Null)
    }

    pub fn cookie(&self) -> Option<String> {
        let brut = self.entetes.get(header::SET_COOKIE)?.to_str().ok()?;
        Some(brut.split(';').next()?.to_string())
    }
}
