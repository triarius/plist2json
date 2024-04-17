use base64::prelude::*;
use color_eyre::eyre::{eyre, OptionExt, Result};
use serde_json::Number;
use std::str::FromStr;

pub(crate) fn convert(input: &plist::Value) -> Result<serde_json::Value> {
    let v = match input {
        plist::Value::Array(xs) => serde_json::Value::Array(
            xs.iter()
                .map(convert)
                .collect::<Result<Vec<serde_json::Value>>>()?,
        ),
        plist::Value::Dictionary(d) => serde_json::Value::Object(d.iter().try_fold(
            serde_json::Map::with_capacity(d.len()),
            |mut acc, (k, v)| {
                acc.insert(k.to_owned(), convert(v)?);
                color_eyre::eyre::Ok(acc)
            },
        )?),
        plist::Value::Boolean(x) => serde_json::Value::Bool(*x),
        plist::Value::Data(buf) => serde_json::Value::String(BASE64_STANDARD.encode(buf)),
        plist::Value::Date(d) => serde_json::Value::String(d.to_xml_format()),
        plist::Value::Real(x) => serde_json::Value::Number(Number::from_str(&format!("{x}"))?),
        plist::Value::Integer(i) => to_number(
            i.as_unsigned()
                .ok_or_eyre(format!("failed to parse {i} as a number"))?,
        )?,
        plist::Value::String(s) => serde_json::Value::String(s.to_owned()),
        plist::Value::Uid(u) => to_number(u.get())?,
        // Upstream marks the Value enum as non-exhastive so we have to have this
        _ => Err(eyre!("Unhandled plist type."))?,
    };

    Ok(v)
}

fn to_number(input: u64) -> Result<serde_json::Value> {
    let number = Number::from_str(&format!("{input}"))?;
    Ok(serde_json::Value::Number(number))
}
