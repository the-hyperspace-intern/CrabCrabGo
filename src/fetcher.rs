pub fn ddg_html(target: &str) -> Result<String, Box<std::error::Error>> {
    let url = format!("https://duckduckgo.com/html/?q={}", target);
    let mut resp = reqwest::get(&url)?;
    let raw = resp.text()?;

    Ok(raw)
}