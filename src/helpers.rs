use rkg_utils::header::SlotId;

pub fn track_abbreviation(slot_id: SlotId) -> &'static str {
    match slot_id {
        SlotId::LuigiCircuit => "LC",
        SlotId::MooMooMeadows => "MMM",
        SlotId::MushroomGorge => "MG",
        SlotId::ToadsFactory => "TF",
        SlotId::MarioCircuit => "MC",
        SlotId::CoconutMall => "CM",
        SlotId::DKSnowboardCross => "DKSC",
        SlotId::WariosGoldMine => "WGM",
        SlotId::DaisyCircuit => "DC",
        SlotId::KoopaCape => "KC",
        SlotId::MapleTreeway => "MT",
        SlotId::GrumbleVolcano => "GV",
        SlotId::DryDryRuins => "DDR",
        SlotId::MoonviewHighway => "MH",
        SlotId::BowsersCastle => "BC",
        SlotId::RainbowRoad => "RR",
        SlotId::GCNPeachBeach => "rPB",
        SlotId::DSYoshiFalls => "rYF",
        SlotId::SNESGhostValley2 => "rGV2",
        SlotId::N64MarioRaceway => "rMR",
        SlotId::N64SherbetLand => "rSL",
        SlotId::GBAShyGuyBeach => "rSGB",
        SlotId::DSDelfinoSquare => "rDS",
        SlotId::GCNWaluigiStadium => "rWS",
        SlotId::DSDesertHills => "rDH",
        SlotId::GBABowserCastle3 => "rBC3",
        SlotId::N64DKJungleParkway => "rDKJP",
        SlotId::GCNMarioCircuit => "rMC",
        SlotId::SNESMarioCircuit3 => "rMC3",
        SlotId::DSPeachGardens => "rPG",
        SlotId::GCNDKMountain => "rDKM",
        SlotId::N64BowsersCastle => "rBC",
        SlotId::BlockPlaza => "bBP",
        SlotId::DelfinoPier => "bDP",
        SlotId::FunkyStadium => "bFS",
        SlotId::ChainChompWheel => "bCCW",
        SlotId::ThwompDesert => "bTD",
        SlotId::SNESBattleCourse4 => "brBC4",
        SlotId::GBABattleCourse3 => "brBC3",
        SlotId::N64Skscraper => "brS",
        SlotId::GCNCookieLand => "brCL",
        SlotId::DSTwilightHouse => "brTH",
        SlotId::GalaxyColosseum => "GC",
        SlotId::WinningScene => "WS",
        SlotId::LosingScene => "LS",
        SlotId::Credits => "C",
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
