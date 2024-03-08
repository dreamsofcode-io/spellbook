use axum::http::StatusCode;

enum Spell {
    Fira,
    Thundara,
    Poisona,
    MagicMissile,
    Revive,
    Cura,
}

pub async fn recent_spells() -> StatusCode {
    StatusCode::OK
}

pub async fn learn_spell() -> StatusCode {
    StatusCode::OK
}

pub async fn forget_spell() -> StatusCode {
    StatusCode::OK
}

pub async fn list_spells() -> StatusCode {
    StatusCode::OK
}

pub async fn list_wizards() -> StatusCode {
    StatusCode::OK
}

pub async fn wizard_for_id() -> StatusCode {
    StatusCode::OK
}
