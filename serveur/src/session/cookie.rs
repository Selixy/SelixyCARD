use axum_extra::extract::cookie::{Cookie, SameSite};

pub const NOM: &str = "session";
pub const JOURS: i64 = 30;

pub fn creer(jeton: String) -> Cookie<'static> {
    Cookie::build((NOM, jeton))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(time::Duration::days(JOURS))
        .build()
}

pub fn effacer() -> Cookie<'static> {
    Cookie::build(NOM).path("/").build()
}
