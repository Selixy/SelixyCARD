mod annonces;
mod codes;
mod connexion;
mod diffusion;
mod global;
mod identite;
mod lobby;
mod messages;
mod registre;
mod routage;
mod salle;
mod sets;
mod sets_postgres;
mod vues;

#[cfg(test)]
mod essais;

pub use connexion::{accepter, accepter_connecte};
pub use global::installer_source;
pub use sets::SourceSets;
pub use sets_postgres::SetsPostgres;
