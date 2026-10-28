export function echelleAffichage(image, canvas) {
  return Math.min(canvas.width / image.width, canvas.height / image.height);
}

export function dessinerRecadrage(canvas, image, cadre) {
  const ctx = canvas.getContext('2d');
  const e = echelleAffichage(image, canvas);
  const ox = (canvas.width - image.width * e) / 2;
  const oy = (canvas.height - image.height * e) / 2;
  const f = { x: ox + cadre.x * e, y: oy + cadre.y * e, l: cadre.l * e, h: cadre.l * 1.5 * e };
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.drawImage(image, ox, oy, image.width * e, image.height * e);
  ctx.fillStyle = 'rgba(60,44,30,.58)';
  ctx.beginPath();
  ctx.rect(ox, oy, image.width * e, image.height * e);
  ctx.rect(f.x + f.l, f.y, -f.l, f.h);
  ctx.fill('evenodd');
  ctx.strokeStyle = '#fbf5e6';
  ctx.setLineDash([10, 6]);
  ctx.lineWidth = 2;
  ctx.strokeRect(f.x, f.y, f.l, f.h);
  ctx.setLineDash([]);
  ctx.strokeStyle = 'rgba(251,245,230,.4)';
  ctx.lineWidth = 1;
  for (let i = 1; i < 3; i++) {
    ctx.strokeRect(f.x + (f.l * i) / 3, f.y, 0, f.h);
    ctx.strokeRect(f.x, f.y + (f.h * i) / 3, f.l, 0);
  }
}
