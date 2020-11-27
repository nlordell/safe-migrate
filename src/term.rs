use anyhow::{anyhow, ensure, Result};
use std::io::{self, BufRead, Write};

/// Reads a user password.
pub fn read_password(prompt: impl AsRef<str>) -> Result<String> {
    let password = rpassword::prompt_password_stdout(&format!("{}: ", prompt.as_ref()))?;
    ensure!(!password.is_empty(), "empty password");

    Ok(password)
}

/// Reads a user confirmation.
pub fn confirm(prompt: impl AsRef<str>) -> Result<()> {
    let mut stdout = io::stdout();
    write!(stdout, "{}? ", prompt.as_ref())?;
    stdout.flush()?;

    let response = io::stdin()
        .lock()
        .lines()
        .next()
        .ok_or_else(|| anyhow!("error reading from stdin"))??;
    ensure!(response == "yes", "aborted");

    Ok(())
}
