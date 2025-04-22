use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GeoZone {
    clave_geografica: String,
    nombre_zona_geografica: String,
    nombre_corto: String,
}
