use super::{FLOTTANTS_PAR_INSTANCE, MAX_INSTANCES};
use crate::scene::Instance;

pub fn flottants_instances(instances: &[Instance]) -> &[f32] {
    let n = instances.len().min(MAX_INSTANCES);
    unsafe { std::slice::from_raw_parts(instances.as_ptr() as *const f32, n * FLOTTANTS_PAR_INSTANCE) }
}

pub fn octets(f: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(f.as_ptr() as *const u8, f.len() * 4) }
}
