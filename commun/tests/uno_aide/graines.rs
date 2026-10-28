use commun::uno::{Options, Partie, TypeCarte};

pub fn pseudos(n: usize) -> Vec<String> {
    ["a", "b", "c", "d", "e"][..n].iter().map(|p| p.to_string()).collect()
}

pub fn graine_avec(genre: TypeCarte, joueurs: usize) -> Partie {
    (0..5000)
        .map(|g| Partie::nouvelle(&pseudos(joueurs), &[], Options::default(), g).unwrap())
        .find(|p| p.haut().genre == genre)
        .expect("une graine convient")
}
