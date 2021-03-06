/*
 * IoT Edge Module Workload API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2018-06-28
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_derive::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptResponse {
    /// The encrypted form of the data encoded in base 64.
    #[serde(rename = "ciphertext")]
    ciphertext: String,
}

impl EncryptResponse {
    pub fn new(ciphertext: String) -> Self {
        EncryptResponse { ciphertext }
    }

    pub fn set_ciphertext(&mut self, ciphertext: String) {
        self.ciphertext = ciphertext;
    }

    pub fn with_ciphertext(mut self, ciphertext: String) -> Self {
        self.ciphertext = ciphertext;
        self
    }

    pub fn ciphertext(&self) -> &String {
        &self.ciphertext
    }
}
