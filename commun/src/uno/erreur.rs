#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erreur {
    PartieTerminee,
    JoueursInvalides,
    JoueurInconnu,
    PasTonTour,
    CarteAbsente,
    CarteInjouable,
    SeulementPiochee,
    PenaliteEnCours,
    CouleurRequise,
    CibleRequise,
    CibleInvalide,
    RienAContester,
    TropDeCartes,
    SignalementInvalide,
    OptionDesactivee,
}

impl Erreur {
    pub fn message(self) -> &'static str {
        match self {
            Erreur::PartieTerminee => "La partie est terminée.",
            Erreur::JoueursInvalides => "Il faut entre 2 et 10 joueurs aux pseudos distincts.",
            Erreur::JoueurInconnu => "Joueur inconnu.",
            Erreur::PasTonTour => "Ce n'est pas ton tour.",
            Erreur::CarteAbsente => "Cette carte n'est pas dans ta main.",
            Erreur::CarteInjouable => "Cette carte ne peut pas être jouée maintenant.",
            Erreur::SeulementPiochee => "Tu ne peux jouer que la carte que tu viens de piocher.",
            Erreur::PenaliteEnCours => "Tu dois répondre à la pénalité ou piocher.",
            Erreur::CouleurRequise => "Choisis une couleur pour ce joker.",
            Erreur::CibleRequise => "Choisis un joueur pour échanger ta main.",
            Erreur::CibleInvalide => "Ce joueur ne peut pas être choisi.",
            Erreur::RienAContester => "Il n'y a rien à contester.",
            Erreur::TropDeCartes => "Tu ne peux pas dire UNO avec autant de cartes.",
            Erreur::SignalementInvalide => "Ce signalement n'est pas valide.",
            Erreur::OptionDesactivee => "Cette règle n'est pas activée.",
        }
    }
}
