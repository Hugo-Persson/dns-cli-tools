use std::net::IpAddr;
pub fn get_last_ip(debug: bool) -> Option<String> {
    let home = home::home_dir().expect("Could not get home dir :(, please file bug report");

    let path = home.join(".last_ip.txt");
    if !path.exists() {
        if debug {
            println!("No last ip file found, probably first run");
        }
        return None;
    }
    parse_stored_ip(std::fs::read_to_string(path).unwrap())
}

fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

fn parse_stored_ip(ip: String) -> Option<String> {
    let trimmed = ip.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}

pub async fn get_current_ip() -> Result<String, Box<dyn std::error::Error>> {
    let ip = reqwest::get("https://api.ipify.org").await?.text().await?;
    if !is_valid_ip(&ip) {
        return Err("Could not get a valid IP".into());
    }
    Ok(ip)
}

pub async fn save_ip(ip: &String) {
    let home = home::home_dir().expect("Could not get home dir :(");

    let path = home.join(".last_ip.txt");
    std::fs::write(path, ip).unwrap();
}

#[cfg(test)]
mod tests {
    use super::parse_stored_ip;

    #[test]
    fn parse_stored_ip_trims_whitespace() {
        assert_eq!(
            parse_stored_ip("  203.0.113.7\n".to_string()),
            Some("203.0.113.7".to_string())
        );
    }

    #[test]
    fn parse_stored_ip_empty_becomes_none() {
        assert_eq!(parse_stored_ip("   \n\t".to_string()), None);
    }
}
