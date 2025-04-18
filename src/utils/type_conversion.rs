use serde::Deserialize;
use serde::Deserializer;

pub fn csv_str_to_vec<'de, D>(de: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;

    if s.trim().is_empty() {
        Ok(vec![])
    } else {
        Ok(s.split(',').map(str::to_string).collect())
    }
}

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let v: u8 = Deserialize::deserialize(deserializer)?;

    match v {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(serde::de::Error::custom("expected 0 or 1")),
    }
}

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
