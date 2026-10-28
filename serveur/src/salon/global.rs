use super::registre::Registre;
use super::sets::{SetParDefaut, SourceSets};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

static REGISTRE: LazyLock<Mutex<Registre>> = LazyLock::new(|| {
    let graine = SystemTime::now().duration_since(UNIX_EPOCH).map_or(1, |d| d.as_nanos() as u64);
    Mutex::new(Registre::nouveau(Arc::new(SetParDefaut), graine))
});

pub fn registre() -> MutexGuard<'static, Registre> {
    REGISTRE.lock().unwrap_or_else(|empoisonne| empoisonne.into_inner())
}

pub fn installer_source(source: Arc<dyn SourceSets>) {
    registre().source = source;
}
