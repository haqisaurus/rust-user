pub fn detect_os(user_agent: &str) -> &str {
    let ua = user_agent.to_lowercase();

    if ua.contains("android") {
        "Android"
    } else if ua.contains("iphone") || ua.contains("ipad") || ua.contains("ios") {
        "iOS"
    } else if ua.contains("windows nt") {
        "Windows"
    } else if ua.contains("mac os x") {
        "macOS"
    } else if ua.contains("linux") {
        "Linux"
    } else {
        "Unknown"
    }
}