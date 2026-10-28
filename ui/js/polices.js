const POLICES = ['400 40px "Caveat Brush"', '700 40px "Caveat"', '400 16px "Lora"'];

export function chargerPolices() {
  if (typeof document === 'undefined' || !document.fonts) return Promise.resolve([]);
  return Promise.all(POLICES.map((police) => document.fonts.load(police).catch(() => null)));
}

export function attendrePolices(delai = 400) {
  if (typeof document === 'undefined') return Promise.resolve();
  const appliquer = () => {
    if (document.body) {
      document.body.classList.add('polices-pretes');
    } else {
      document.addEventListener('DOMContentLoaded', () => {
        document.body?.classList.add('polices-pretes');
      }, { once: true });
    }
  };
  if (document.body?.classList.contains('polices-pretes')) return Promise.resolve();
  return new Promise((resoudre) => {
    let termine = false;
    const clore = () => {
      if (termine) return;
      termine = true;
      clearTimeout(chrono);
      appliquer();
      resoudre();
    };
    const chrono = setTimeout(clore, delai);
    const pret = document.fonts?.ready ?? Promise.resolve();
    pret.then(clore).catch(clore);
  });
}
