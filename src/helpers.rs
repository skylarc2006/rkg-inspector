use rkg_utils::{
    footer::ctgp_footer::{Category, Region},
    header::{SlotId, mii::FavoriteColor},
};

use crate::link_type::LinkType;

pub fn track_abbreviation(slot_id: SlotId) -> String {
    match slot_id {
        SlotId::LuigiCircuit => String::from("LC"),
        SlotId::MooMooMeadows => String::from("MMM"),
        SlotId::MushroomGorge => String::from("MG"),
        SlotId::ToadsFactory => String::from("TF"),
        SlotId::MarioCircuit => String::from("MC"),
        SlotId::CoconutMall => String::from("CM"),
        SlotId::DKSnowboardCross => String::from("DKSC"),
        SlotId::WariosGoldMine => String::from("WGM"),
        SlotId::DaisyCircuit => String::from("DC"),
        SlotId::KoopaCape => String::from("KC"),
        SlotId::MapleTreeway => String::from("MT"),
        SlotId::GrumbleVolcano => String::from("GV"),
        SlotId::DryDryRuins => String::from("DDR"),
        SlotId::MoonviewHighway => String::from("MH"),
        SlotId::BowsersCastle => String::from("BC"),
        SlotId::RainbowRoad => String::from("RR"),
        SlotId::GCNPeachBeach => String::from("rPB"),
        SlotId::DSYoshiFalls => String::from("rYF"),
        SlotId::SNESGhostValley2 => String::from("rGV2"),
        SlotId::N64MarioRaceway => String::from("rMR"),
        SlotId::N64SherbetLand => String::from("rSL"),
        SlotId::GBAShyGuyBeach => String::from("rSGB"),
        SlotId::DSDelfinoSquare => String::from("rDS"),
        SlotId::GCNWaluigiStadium => String::from("rWS"),
        SlotId::DSDesertHills => String::from("rDH"),
        SlotId::GBABowserCastle3 => String::from("rBC3"),
        SlotId::N64DKJungleParkway => String::from("rDKJP"),
        SlotId::GCNMarioCircuit => String::from("rMC"),
        SlotId::SNESMarioCircuit3 => String::from("rMC3"),
        SlotId::DSPeachGardens => String::from("rPG"),
        SlotId::GCNDKMountain => String::from("rDKM"),
        SlotId::N64BowsersCastle => String::from("rBC"),
        SlotId::BlockPlaza => String::from("bBP"),
        SlotId::DelfinoPier => String::from("bDP"),
        SlotId::FunkyStadium => String::from("bFS"),
        SlotId::ChainChompWheel => String::from("bCCW"),
        SlotId::ThwompDesert => String::from("bTD"),
        SlotId::SNESBattleCourse4 => String::from("brBC4"),
        SlotId::GBABattleCourse3 => String::from("brBC3"),
        SlotId::N64Skscraper => String::from("brS"),
        SlotId::GCNCookieLand => String::from("brCL"),
        SlotId::DSTwilightHouse => String::from("brTH"),
        SlotId::GalaxyColosseum => String::from("GC"),
        SlotId::WinningScene => String::from("WS"),
        SlotId::LosingScene => String::from("LS"),
        SlotId::Credits => String::from("C"),
    }
}

pub fn favorite_color_string(f: FavoriteColor) -> String {
    match f {
        FavoriteColor::Red => String::from("Red"),
        FavoriteColor::Orange => String::from("Orange"),
        FavoriteColor::Yellow => String::from("Yellow"),
        FavoriteColor::LimeGreen => String::from("Light Green"),
        FavoriteColor::ForestGreen => String::from("Green"),
        FavoriteColor::RoyalBlue => String::from("Blue"),
        FavoriteColor::SkyBlue => String::from("Light Blue"),
        FavoriteColor::Pink => String::from("Pink"),
        FavoriteColor::Purple => String::from("Purple"),
        FavoriteColor::Brown => String::from("Brown"),
        FavoriteColor::White => String::from("White"),
        FavoriteColor::Black => String::from("Black"),
    }
}

pub fn array_to_hex_string(arr: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(arr.len() * 2);
    for byte in arr {
        write!(s, "{:02X}", byte).unwrap();
    }
    s
}

pub fn disc_region_string(disc_region: &Region) -> &str {
    match disc_region {
        Region::NtscU => "E (NTSC-U)",
        Region::Pal => "P (PAL)",
        Region::NtscJ => "J (NTSC-J)",
        Region::Unknown => "Unknown/invalid",
    }
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
    let link_type = if link_type == LinkType::Json {
        "json"
    } else {
        "html"
    };

    format!(
        "https://chadsoft.co.uk/time-trials/leaderboard/{slot_hex:02X}/{track_sha1}/{category:02}.{link_type}"
    )
}

pub fn chadsoft_ghost_link(ghost_sha1: &[u8], link_type: LinkType) -> String {
    // https://chadsoft.co.uk/time-trials/rkgd/{G0}/{G1}/{GHOST_ID}.html
    // G0 - first byte of ghost SHA1 in hex
    // G1 - next byte of ghost SHA1 in hex
    // G2 - remaining bytes of ghost SHA1 in hex

    let byte_1 = ghost_sha1[0];
    let byte_2 = ghost_sha1[1];
    let remaining_bytes = array_to_hex_string(&ghost_sha1[2..]);
    let link_type = match link_type {
        LinkType::Html => "html",
        LinkType::Json => "json",
        LinkType::Rkg => "rkg",
        LinkType::Mii => "mii",
    };

    format!(
        "https://chadsoft.co.uk/time-trials/rkgd/{byte_1:02X}/{byte_2:02X}/{remaining_bytes}.{link_type}"
    )
}

pub fn chadsoft_player_link(player_id: u64, link_type: LinkType) -> String {
    // https://chadsoft.co.uk/time-trials/players/{P0}/{P1}.html
    // P0 - first byte of player ID in hex
    // P1 - remaining bytes of player ID in hex

    let player_id = player_id.to_be_bytes();
    let byte_1 = player_id[0];
    let remaining_bytes = array_to_hex_string(&player_id[1..]);
    let link_type = if link_type == LinkType::Json {
        "json"
    } else {
        "html"
    };

    format!("https://chadsoft.co.uk/time-trials/players/{byte_1:02X}/{remaining_bytes}.{link_type}")
}

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
