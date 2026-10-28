const DELAI_MS = 1500;

export function creerRelanceur(action) {
  const etat = { timer: null, actif: true };
  return {
    lancer() {
      if (!etat.actif) return;
      if (etat.timer) clearTimeout(etat.timer);
      etat.timer = null;
      action();
    },
    planifier() {
      if (!etat.actif || etat.timer) return;
      etat.timer = setTimeout(() => {
        etat.timer = null;
        action();
      }, DELAI_MS);
    },
    arreter() {
      etat.actif = false;
      if (etat.timer) clearTimeout(etat.timer);
      etat.timer = null;
    },
    etat,
  };
}
