use crate::graphe::GraphePbr;

pub fn serialiser_graphe(graphe: &GraphePbr) -> Result<String, String> {
    serde_json::to_string(graphe).map_err(|e| e.to_string())
}

pub fn serialiser_graphe_joli(graphe: &GraphePbr) -> Result<String, String> {
    serde_json::to_string_pretty(graphe).map_err(|e| e.to_string())
}

pub fn deserialiser_graphe(json: &str) -> Result<GraphePbr, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}
