use crate::graphe::GraphePbr;
use crate::noeud::{IdNoeud, Operation};

fn nom_var(id: IdNoeud) -> String {
    format!("n{}", id.0)
}

fn expr_glsl(op: &Operation) -> String {
    match op {
        Operation::Source(entree) => entree.glsl(),
        Operation::Addition(a, b) => format!("({} + {})", nom_var(*a), nom_var(*b)),
        Operation::Soustraction(a, b) => format!("({} - {})", nom_var(*a), nom_var(*b)),
        Operation::Multiplication(a, b) => format!("({} * {})", nom_var(*a), nom_var(*b)),
        Operation::Division(a, b) => format!("({} / {})", nom_var(*a), nom_var(*b)),
        Operation::Puissance(a, b) => format!("pow({}, {})", nom_var(*a), nom_var(*b)),
        Operation::Melange(a, b, t) => format!("mix({}, {}, {})", nom_var(*a), nom_var(*b), nom_var(*t)),
        Operation::Borne(val, min, max) => format!("clamp({}, {}, {})", nom_var(*val), nom_var(*min), nom_var(*max)),
        Operation::Lissage(min, max, val) => format!("smoothstep({}, {}, {})", nom_var(*min), nom_var(*max), nom_var(*val)),
        Operation::ProduitScalaire(a, b) => format!("dot({}, {})", nom_var(*a), nom_var(*b)),
        Operation::Longueur(a) => format!("length({})", nom_var(*a)),
        Operation::Normaliser(a) => format!("normalize({})", nom_var(*a)),
        Operation::Absolu(a) => format!("abs({})", nom_var(*a)),
        Operation::Sinus(a) => format!("sin({})", nom_var(*a)),
        Operation::Cosinus(a) => format!("cos({})", nom_var(*a)),
        Operation::Exponentielle(a) => format!("exp({})", nom_var(*a)),
        Operation::Minimum(a, b) => format!("min({}, {})", nom_var(*a), nom_var(*b)),
        Operation::Maximum(a, b) => format!("max({}, {})", nom_var(*a), nom_var(*b)),
        Operation::Seuil(seuil, val) => format!("step({}, {})", nom_var(*seuil), nom_var(*val)),
        Operation::TextureTableau(uv, couche) => format!("textureLod(u_tableau, vec3({}, {}), 0.0)", nom_var(*uv), nom_var(*couche)),
        Operation::BruitGrain(uv) => format!("fract(sin(dot({}, vec2(12.9898, 78.233))) * 43758.5453)", nom_var(*uv)),
        Operation::BruitPerlin(uv) => format!("bruit({})", nom_var(*uv)),
        Operation::Composante(a, c) => format!("{}.{}", nom_var(*a), c),
        Operation::AssemblerVecteur3(x, y, z) => format!("vec3({}, {}, {})", nom_var(*x), nom_var(*y), nom_var(*z)),
    }
}

pub fn emettre_fonction_surface_glsl(graphe: &GraphePbr) -> Result<String, &'static str> {
    let ordre = graphe.tri_topologique()?;
    let mut code = String::from("ProprietesSurface evaluer_surface() {\n");
    for id in ordre {
        let noeud = graphe.noeud(id).ok_or("noeud introuvable")?;
        code.push_str(&format!("    {} {} = {};\n", noeud.type_sortie.glsl(), nom_var(id), expr_glsl(&noeud.operation)));
    }
    let s = &graphe.surface;
    let norm = s.normale.map_or("v_normale".into(), nom_var);
    let rel = s.relief.map_or("0.0".into(), nom_var);
    let emi = s.emission.map_or("vec3(0.0)".into(), nom_var);
    let alp = s.alpha.map_or("1.0".into(), nom_var);
    code.push_str(&format!("    return ProprietesSurface({}, {}, {}, {}, {}, {}, {});\n}}\n", nom_var(s.albedo), nom_var(s.rugosite), nom_var(s.metallique), norm, rel, emi, alp));
    Ok(code)
}
