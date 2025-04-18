use crate::utils::type_conversion::serialize_vec_to_comma_separated;
use crate::{bool_from_int, csv_str_to_vec, parse_secret_data};
use serde::{Deserialize, Serialize};

impl From<ZohoStyleCsv> for ProtonStyleCsv {
    fn from(z: ZohoStyleCsv) -> Self {
        let email = if z.secret_data.username.contains('@') {
            Some(z.secret_data.username.clone())
        } else {
            None
        };

        let note = if z.notes.trim().is_empty() {
            None
        } else {
            Some(z.notes)
        };

        let vault = if z.folder_name.trim().is_empty() {
            None
        } else {
            Some(z.folder_name)
        };

        ProtonStyleCsv {
            name: z.password_name,
            url: z.password_url,
            email,
            username: z.secret_data.username,
            password: z.secret_data.password,
            note,
            totp: z.totp,
            vault,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SecretData {
    #[serde(rename = "SecretType")]
    pub _secret_type: String,
    #[serde(rename = "User Name")]
    pub username: String,
    #[serde(rename = "Password")]
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ZohoStyleCsv {
    #[serde(rename = "Password Name")]
    pub password_name: String,

    #[serde(rename = "Description")]
    pub _description: String,

    #[serde(rename = "Password URL", deserialize_with = "csv_str_to_vec")]
    pub password_url: Vec<String>,

    #[serde(rename = "SecretData", deserialize_with = "parse_secret_data")]
    pub secret_data: SecretData,

    #[serde(rename = "Notes")]
    pub notes: String,

    #[serde(rename = "CustomData")]
    pub _custom_data: String,

    #[serde(rename = "Tags", deserialize_with = "csv_str_to_vec")]
    pub _tags: Vec<String>,

    #[serde(rename = "Classification")]
    #[serde(skip_deserializing)]
    pub _classification: Option<String>,

    #[serde(rename = "Favorite", deserialize_with = "bool_from_int")]
    pub _favorite: bool,

    #[serde(rename = "TOTP")]
    pub totp: Option<String>,

    #[serde(rename = "Folder Name")]
    pub folder_name: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ProtonStyleCsv {
    pub name: String,
    #[serde(serialize_with = "serialize_vec_to_comma_separated")]
    pub url: Vec<String>,
    pub email: Option<String>,
    pub username: String,
    pub password: String,
    pub note: Option<String>,
    pub totp: Option<String>,
    pub vault: Option<String>,
}
