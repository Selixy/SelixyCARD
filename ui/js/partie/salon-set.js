export async function enrichirSalon(message, appeler) {
  if (message.t !== 'salon' || typeof message.set !== 'string' || !message.set) return message;
  const set = await appeler('GET', `/api/sets/${message.set}`).catch(() => null);
  return { ...message, set: { id: message.set, dos: set?.dos ?? null } };
}
