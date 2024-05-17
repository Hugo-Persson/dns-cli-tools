pub fn get_last_ip(debug: bool) -> String {
    let home = home::home_dir().expect("Could not get home dir :(, please file bug report");

    let path = home.join(".last_ip.txt");
    if !path.exists() {
        if debug {
            println!("No last ip file found, probably first run");
        }
        return "".to_string();
    }
    std::fs::read_to_string(path).unwrap()
}

pub async fn get_current_ip() -> String {
    return "78.71.54.77".to_string();
    reqwest::get("https://api.ipify.org")
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

pub async fn save_ip(ip: &String) {
    let home = home::home_dir().expect("Could not get home dir :(");

    let path = home.join(".last_ip.txt");
    std::fs::write(path, ip).unwrap();
}

