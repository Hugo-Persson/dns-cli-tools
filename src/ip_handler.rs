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
    Some(std::fs::read_to_string(path).unwrap())
}

fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
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
