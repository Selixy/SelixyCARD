use sha2::{Digest, Sha256};

pub fn hexa(octets: &[u8]) -> String {
    octets.iter().map(|octet| format!("{octet:02x}")).collect()
}

pub fn nouveau() -> Result<String, getrandom::Error> {
    let mut octets = [0u8; 32];
    getrandom::fill(&mut octets)?;
    Ok(hexa(&octets))
}

pub fn empreinte(jeton: &str) -> Vec<u8> {
    Sha256::digest(jeton.as_bytes()).to_vec()
}
