use rkg_utils::{footer::ctgp_footer::Category, header::SlotId};

use crate::helpers::array_to_hex_string;
use crate::link_type::LinkType;

pub fn chadsoft_leaderboard_link(
    slot_id: SlotId,
    track_sha1: &[u8],
    category: Category,
    link_type: LinkType,
) -> String {
    // https://chadsoft.co.uk/time-trials/leaderboard/{SLOT_HEX}/{TRACK_SHA1}/{CATEGORY}.html
    let slot_hex = u8::from(slot_id);
    let track_sha1 = array_to_hex_string(track_sha1);
    let category = match category {
        Category::Shortcut | Category::Normal => 0,
        Category::Glitch => 1,
        Category::NoShortcut => 2,
        Category::GlitchTAS
        | Category::NormalTAS
        | Category::ShortcutTAS
        | Category::NoShortcutTAS => 3,
        Category::Shortcut200cc | Category::Normal200cc => 4,
        Category::Glitch200cc => 5,
        Category::NoShortcut200cc => 6,
        Category::Glitch200ccTAS
        | Category::Normal200ccTAS
        | Category::Shortcut200ccTAS
        | Category::NoShortcut200ccTAS => 7,
    };
    let extension = link_type.extension();

    format!(
        "https://chadsoft.co.uk/time-trials/leaderboard/{slot_hex:02X}/{track_sha1}/{category:02}.{extension}"
    )
}

pub fn chadsoft_ghost_link(ghost_sha1: &[u8], link_type: LinkType) -> String {
    // https://chadsoft.co.uk/time-trials/rkgd/{G0}/{G1}/{GHOST_ID}.html
    // G0 - first byte of ghost SHA1 in hex
    // G1 - next byte of ghost SHA1 in hex
    // G2 - remaining bytes of ghost SHA1 in hex

    let byte_1 = ghost_sha1.first().copied().unwrap_or(0);
    let byte_2 = ghost_sha1.get(1).copied().unwrap_or(0);
    let remaining_bytes = array_to_hex_string(ghost_sha1.get(2..).unwrap_or(&[]));
    let extension = link_type.extension();

    format!(
        "https://chadsoft.co.uk/time-trials/rkgd/{byte_1:02X}/{byte_2:02X}/{remaining_bytes}.{extension}"
    )
}

pub fn chadsoft_player_link(player_id: u64, link_type: LinkType) -> String {
    // https://chadsoft.co.uk/time-trials/players/{P0}/{P1}.html
    // P0 - first byte of player ID in hex
    // P1 - remaining bytes of player ID in hex

    let player_id = player_id.to_be_bytes();
    let byte_1 = player_id[0];
    let remaining_bytes = array_to_hex_string(&player_id[1..]);
    let extension = link_type.extension();

    format!("https://chadsoft.co.uk/time-trials/players/{byte_1:02X}/{remaining_bytes}.{extension}")
}

// Chadsoft's leaderboard JSON endpoint embeds every recorded ghost for the track (which can
// run into the megabytes for popular custom tracks) even though the track name/version we
// actually want sits in the first few hundred bytes, right before the `"ghosts"` array. Rather
// than downloading and parsing the entire payload (slow and prone to timing out), the response
// is streamed and truncated as soon as the `"ghosts"` key is seen, then parsed as a small,
// self-contained JSON object.
use futures_util::StreamExt;
use std::time::Duration;

const TRACK_NAME_FETCH_ATTEMPTS: u32 = 3;
const TRACK_NAME_FETCH_TIMEOUT: Duration = Duration::from_secs(10);
/// Safety cap on how many leading bytes of the response are buffered while looking for the
/// `"ghosts"` key, in case an unexpected response shape never contains it.
const HEADER_SCAN_CAP: usize = 64 * 1024;
const GHOSTS_KEY: &[u8] = b",\"ghosts\":";

pub async fn fetch_ctgp_track_name(
    slot_id: SlotId,
    track_sha1: Vec<u8>,
    category: Category,
) -> Option<String> {
    let json_link = chadsoft_leaderboard_link(slot_id, &track_sha1, category, LinkType::Json);
    let client = reqwest::Client::builder()
        .timeout(TRACK_NAME_FETCH_TIMEOUT)
        .build()
        .ok()?;

    let mut json = None;
    for attempt in 0..TRACK_NAME_FETCH_ATTEMPTS {
        json = fetch_leaderboard_header(&client, &json_link).await;
        if json.is_some() || attempt + 1 == TRACK_NAME_FETCH_ATTEMPTS {
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    let json = json?;

    let name = json["name"]
        .as_str()
        .map(String::from)
        .unwrap_or_else(|| array_to_hex_string(&track_sha1));

    Some(match json["version"].as_str() {
        Some(version) => format!("{name} ({version})"),
        None => name,
    })
}

/// Fetches just enough of the leaderboard JSON response to read its top-level fields, without
/// downloading the (potentially huge) trailing `"ghosts"` array.
async fn fetch_leaderboard_header(client: &reqwest::Client, url: &str) -> Option<serde_json::Value> {
    let response = client.get(url).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }

    let mut buffer: Vec<u8> = Vec::new();
    let mut stream = response.bytes_stream();
    while buffer.len() < HEADER_SCAN_CAP {
        let chunk = stream.next().await?.ok()?;
        buffer.extend_from_slice(&chunk);

        if let Some(idx) = buffer
            .windows(GHOSTS_KEY.len())
            .position(|window| window == GHOSTS_KEY)
        {
            buffer.truncate(idx);
            buffer.push(b'}');
            return serde_json::from_slice(&buffer).ok();
        }
    }

    None
}
