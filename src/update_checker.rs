use crate::ui::constants::Version;

const RELEASES_API_URL: &str =
    "https://api.github.com/repos/skylarc2006/rkg-inspector/releases/latest";
pub const RELEASES_PAGE_URL: &str = "https://github.com/skylarc2006/rkg-inspector/releases";
const REQUEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

pub async fn fetch_latest_version() -> Option<Version> {
    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .user_agent("rkg-inspector-update-checker")
        .build()
        .ok()?;

    let response = client.get(RELEASES_API_URL).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }

    let json: serde_json::Value = response.json().await.ok()?;
    parse_version(json["tag_name"].as_str()?)
}

fn parse_version(tag: &str) -> Option<Version> {
    let tag = tag.strip_prefix('v').unwrap_or(tag);
    let mut parts = tag.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;
    Some(Version {
        major,
        minor,
        patch,
    })
}
