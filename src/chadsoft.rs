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

// There is code fully implemented for custom track name fetching via Chadsoft's JSON API,
// but unfortunately is currently unusable as Chadsoft's JSON API is extremely unreliable
// or non-functional. Not currently wired up to any message/button.
pub async fn fetch_ctgp_track_name(
    slot_id: SlotId,
    track_sha1: Vec<u8>,
    category: Category,
) -> Option<String> {
    let json_link = chadsoft_leaderboard_link(slot_id, &track_sha1, category, LinkType::Json);
    let json: serde_json::Value = reqwest::get(json_link).await.ok()?.json().await.ok()?;

    let mut track_name = String::new();
    if let Some(t) = json["name"].as_str() {
        track_name.push_str(t);
    } else {
        track_name.push_str(&array_to_hex_string(&track_sha1));
    }

    if let Some(v) = json["version"].as_str() {
        track_name.push_str(format!("({})", v).as_str());
    }

    Some(track_name)
}
