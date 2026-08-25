use rkg_utils::{footer::ctgp_footer::Category, header::SlotId};

use crate::helpers::array_to_hex_string;
use crate::link_type::LinkType;

const BASE_TRACK_SHA1S: [&str; 32] = [
	"B9821B14A89381F9C015669353CB24D7DB1BB25D",
	"E4BF364CB0C5899907585D731621CA930A4EF85C",
	"72D0241C75BE4A5EBD242B9D8D89B1D6FD56BE8F",
	"B02ED72E00B400647BDA6845BE387C47D251F9D1",
	"38486C4F706395772BD988C1AC5FA30D27CAE098",
	"BC038E163D21D9A1181B60CF90B4D03EFAD9E0C5",
	"4EC538065FDC8ACF49674300CBDEC5B80CC05A0D",
	"F9A62BEF04CC8F499633E4023ACC7675A92771F0",
	"8C854B087417A92425110CC71E23C944D6997806",
	"A4BEA41BE83D816F793F3FAD97D268F71AD99BF9",
	"E8ED31605CC7D6660691998F024EED6BA8B4A33F",
	"B036864CF0016BE0581449EF29FB52B2E58D78A4",
	"1941A29AD2E7B7BBA8A29E6440C95EF5CF76B01D",
	"8014488A60F4428EEF52D01F8C5861CA9565E1CA",
	"418099824AF6BF1CD7F8BB44F61E3A9CC3007DAE",
	"ACC0883AE0CE7879C6EFBA20CFE5B5909BF7841B",
	"52F01AE3AED1E0FA4C7459A648494863E83A548C",
	"1AE1A7D894960B38E09E7494373378D87305A163",
	"48EBD9D64413C2B98D2B92E5EFC9B15ECD76FEE6",
	"7752BB51EDBC4A95377C0A05B0E0DA1503786625",
	"90720A7D57A7C76E2347782F6BDE5D22342FB7DD",
	"B13C515475D7DA207DFD5BADD886986147B906FF",
	"0E380357AFFCFD8722329994885699D9927F8276",
	"15B303B288F4707E5D0AF28367C8CE51CDEAB490",
	"692D566B05434D8C66A55BDFF486698E0FC96095",
	"49514E8F74FEA50E77273C0297086D67E58123E8",
	"BA9BCFB3731A6CB17DBA219A8D37EA4D52332256",
	"FFE518915E5FAAA889057C8A3D3E439868574508",
	"071D697C4DDB66D3B210F36C7BF878502E79845B",
	"077111B996E5C4F47D20EC29C2938504B53A8E76",
	"1896AEA49617A571C66FF778D8F2ABBE9E5D7479",
	"D1A453B43D6920A78565E65A4597E353B177ABD0",
];

/// Whether `track_sha1` matches one of the base game tracks, in which case the track name is
/// already known locally and does not need to be fetched from Chadsoft.
pub fn is_base_track_sha1(track_sha1: &[u8]) -> bool {
    let track_sha1 = array_to_hex_string(track_sha1);
    BASE_TRACK_SHA1S.contains(&track_sha1.as_str())
}

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
