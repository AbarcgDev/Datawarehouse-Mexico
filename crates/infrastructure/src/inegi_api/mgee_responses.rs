use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MgeeServiceData {
    cvegeo: String,
    cve_ent: String,
    nomgeo: String,
    nom_abrev: String,
    pob_total: String,
    pob_femenina: String,
    pob_masculina: String,
    total_viviendas_habitadas: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MgeeServiceMetadata {
    #[serde(rename = "Fuente_informacion_estadistica")]
    fuente_informacion_estadistica: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MgeeResponse {
    datos: MgeeServiceData,
    metadatos: MgeeServiceMetadata,
    #[serde(rename = "numReg")]
    num_reg: i32,
}

impl MgeeResponse {
    pub fn new(
        datos: MgeeServiceData,
        metadatos: MgeeServiceMetadata,
        num_reg: i32,
    ) -> MgeeResponse {
        Self {
            datos,
            metadatos,
            num_reg,
        }
    }
}
