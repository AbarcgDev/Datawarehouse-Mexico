use serde::{Deserialize, Serialize};

use crate::inegi_api::mgee_responses::{MgeeServiceData, MgeeServiceMetadata};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ApiResponse {
    MgeeSuccesResponse {
        datos: MgeeServiceData,
        metadatos: MgeeServiceMetadata,
        #[serde(rename = "numReg")]
        num_reg: i32,
    },
    MgeeErrorResponse {
        result: String,
        mensaje: String,
    },
}
