use anyhow::{ensure, Result};
use curl::easy::{Easy, List};
use serde::{de::DeserializeOwned, Serialize};
use std::{io::Read, str};

/// Perform an HTTP GET request and return some JSON.
pub fn get_json<T>(url: impl AsRef<str>) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut easy = Easy::new();
    easy.url(url.as_ref())?;

    let mut buffer = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    check_response_code(&mut easy, &buffer)?;

    let result = serde_json::from_slice(&buffer)?;
    Ok(result)
}

/// Perform an HTTP POST request and return some JSON.
pub fn post_json<T, U>(url: impl AsRef<str>, body: &T) -> Result<U>
where
    T: Serialize,
    U: DeserializeOwned,
{
    let mut easy = Easy::new();
    easy.url(url.as_ref())?;
    easy.post(true)?;

    let body = serde_json::to_vec(body)?;
    easy.post_field_size(body.len() as _)?;
    easy.http_headers({
        let mut list = List::new();
        list.append("Content-Type: application/json").unwrap();
        list
    })?;

    let mut body = &body[..];
    let mut buffer = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| Ok(body.read(buf).unwrap_or(0)))?;
        transfer.write_function(|buf| {
            buffer.extend_from_slice(buf);
            Ok(buf.len())
        })?;
        transfer.perform()?;
    }

    check_response_code(&mut easy, &buffer)?;

    let result = serde_json::from_slice(&buffer)?;
    Ok(result)
}

fn check_response_code(easy: &mut Easy, response: &[u8]) -> Result<()> {
    let code = easy.response_code()?;
    ensure!(
        code >= 200 && code < 400,
        "HTTP {}: {}",
        code,
        str::from_utf8(&response)?,
    );

    Ok(())
}
