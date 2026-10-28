import { normaliserCode } from './adresses.js';
import { choisirSet, creer, lancer, rejoindre, reglages } from './messages.js';
import { reglesAffichees } from './vue.js';

export function initialiserPseudo(vue, deps) {
  vue.code.value = normaliserCode(new URLSearchParams(deps.location.search).get('code'));
  try {
    vue.pseudo.value = deps.stockage?.getItem('pseudo') ?? '';
  } catch {
    vue.pseudo.value = '';
  }
}

export function brancherEvenements(session, entrer) {
  const { vue } = session;
  vue.creer.addEventListener('click', () => entrer(session, (pseudo) => creer(pseudo, null)));
  vue.rejoindre.addEventListener('click', () => entrer(session, (pseudo) => rejoindre(pseudo, vue.code.value)));
  vue.regles.addEventListener('change', () => session.lien.envoyer(reglages(reglesAffichees(vue))));
  vue.set.addEventListener('change', () => session.lien.envoyer(choisirSet(vue.set.value)));
  vue.lancer.addEventListener('click', () => session.lien.envoyer(lancer()));
  vue['ajouter-robot']?.addEventListener('click', () => session.hoteRobots.ajouter());
}
