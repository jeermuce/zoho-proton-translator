use crate::{ProtonStyleCsv, ZohoStyleCsv};

pub fn serialize_vec_to_comma_separated<S>(
    value: &[String],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = value.join(",");
    serializer.serialize_str(&s)
}

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
