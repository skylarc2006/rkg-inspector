use rkg_utils::{
    footer::ctgp_footer::region::Region,
    header::{mii::favorite_color::FavoriteColor, slot_id::SlotId},
};

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
