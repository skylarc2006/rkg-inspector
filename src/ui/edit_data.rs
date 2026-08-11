use std::sync::OnceLock;

use rkg_utils::Header;
use rkg_utils::header::{
    Controller, Date, GhostType, InGameTime, Location, SlotId, TransmissionMod,
    combo::{Character, Vehicle},
    location::constants::{Country, Subregion, Version},
};

pub struct EditBuffers {
    pub finish_time: String,
    pub lap_splits: Vec<String>,
    pub date: String,
}

impl EditBuffers {
    pub fn from_header(header: &Header) -> Self {
        Self {
            finish_time: header.finish_time().to_string(),
            lap_splits: header
                .lap_split_times()
                .iter()
                .map(ToString::to_string)
                .collect(),
            date: header.date_set().to_string(),
        }
    }
}

pub struct LocationOption {
    pub subregion: Subregion,
    pub location: Location,
}

pub struct CountryLocations {
    pub country: Country,
    pub options: Vec<LocationOption>,
}

/// Every country/subregion combination the ghost format recognizes, grouped by
/// country, for the location editor's pair of dropdowns.
pub fn location_table() -> &'static [CountryLocations] {
    static TABLE: OnceLock<Vec<CountryLocations>> = OnceLock::new();
    TABLE.get_or_init(build_location_table)
}

fn build_location_table() -> Vec<CountryLocations> {
    const VERSIONS: [Version; 5] = [
        Version::Vanilla,
        Version::ER10,
        Version::ER11,
        Version::ER12,
        Version::ER13,
    ];

    let mut table = Vec::new();

    for country_id in 0u8..=255 {
        let Ok(country) = Country::try_from(country_id) else {
            continue;
        };

        let mut options: Vec<LocationOption> = Vec::new();
        for subregion_id in 0u8..=255 {
            for version in VERSIONS {
                let Some(location) = Location::find_exact(country_id, subregion_id, version)
                else {
                    continue;
                };
                if !options.iter().any(|o| o.subregion == location.subregion()) {
                    options.push(LocationOption {
                        subregion: location.subregion(),
                        location,
                    });
                }
            }
        }

        if options.is_empty() {
            continue;
        }

        options.sort_by(|a, b| a.subregion.to_string().cmp(&b.subregion.to_string()));
        table.push(CountryLocations { country, options });
    }

    table.push(CountryLocations {
        country: Country::NotSet,
        options: vec![LocationOption {
            subregion: Location::default().subregion(),
            location: Location::default(),
        }],
    });

    table.sort_by(|a, b| a.country.to_string().cmp(&b.country.to_string()));
    table
}

pub fn parse_in_game_time(s: &str) -> Option<InGameTime> {
    let (minutes, rest) = s.trim().split_once(':')?;
    let (seconds, millis) = rest.split_once('.')?;
    InGameTime::new(
        minutes.trim().parse().ok()?,
        seconds.trim().parse().ok()?,
        millis.trim().parse().ok()?,
    )
    .ok()
}

pub fn parse_date(s: &str) -> Option<Date> {
    let mut parts = s.trim().splitn(3, '-');
    let year = parts.next()?.trim().parse().ok()?;
    let month = parts.next()?.trim().parse().ok()?;
    let day = parts.next()?.trim().parse().ok()?;
    Date::new(year, month, day).ok()
}

pub const SLOT_IDS: [SlotId; 46] = [
    SlotId::LuigiCircuit,
    SlotId::MooMooMeadows,
    SlotId::MushroomGorge,
    SlotId::ToadsFactory,
    SlotId::MarioCircuit,
    SlotId::CoconutMall,
    SlotId::DKSnowboardCross,
    SlotId::WariosGoldMine,
    SlotId::DaisyCircuit,
    SlotId::KoopaCape,
    SlotId::MapleTreeway,
    SlotId::GrumbleVolcano,
    SlotId::DryDryRuins,
    SlotId::MoonviewHighway,
    SlotId::BowsersCastle,
    SlotId::RainbowRoad,
    SlotId::GCNPeachBeach,
    SlotId::DSYoshiFalls,
    SlotId::SNESGhostValley2,
    SlotId::N64MarioRaceway,
    SlotId::N64SherbetLand,
    SlotId::GBAShyGuyBeach,
    SlotId::DSDelfinoSquare,
    SlotId::GCNWaluigiStadium,
    SlotId::DSDesertHills,
    SlotId::GBABowserCastle3,
    SlotId::N64DKJungleParkway,
    SlotId::GCNMarioCircuit,
    SlotId::SNESMarioCircuit3,
    SlotId::DSPeachGardens,
    SlotId::GCNDKMountain,
    SlotId::N64BowsersCastle,
    SlotId::DelfinoPier,
    SlotId::BlockPlaza,
    SlotId::ChainChompWheel,
    SlotId::FunkyStadium,
    SlotId::ThwompDesert,
    SlotId::GCNCookieLand,
    SlotId::DSTwilightHouse,
    SlotId::SNESBattleCourse4,
    SlotId::GBABattleCourse3,
    SlotId::N64Skscraper,
    SlotId::GalaxyColosseum,
    SlotId::WinningScene,
    SlotId::LosingScene,
    SlotId::Credits,
];

pub const CONTROLLERS: [Controller; 4] = [
    Controller::WiiWheel,
    Controller::Nunchuk,
    Controller::Classic,
    Controller::Gamecube,
];

pub const TRANSMISSION_MODS: [TransmissionMod; 4] = [
    TransmissionMod::Vanilla,
    TransmissionMod::AllInside,
    TransmissionMod::AllBikeInside,
    TransmissionMod::AllOutside,
];

pub const GHOST_TYPES: [GhostType; 38] = [
    GhostType::PlayerBest,
    GhostType::WorldRecord,
    GhostType::ContinentalRecord,
    GhostType::Rival,
    GhostType::Special,
    GhostType::GhostRace,
    GhostType::Friend1,
    GhostType::Friend2,
    GhostType::Friend3,
    GhostType::Friend4,
    GhostType::Friend5,
    GhostType::Friend6,
    GhostType::Friend7,
    GhostType::Friend8,
    GhostType::Friend9,
    GhostType::Friend10,
    GhostType::Friend11,
    GhostType::Friend12,
    GhostType::Friend13,
    GhostType::Friend14,
    GhostType::Friend15,
    GhostType::Friend16,
    GhostType::Friend17,
    GhostType::Friend18,
    GhostType::Friend19,
    GhostType::Friend20,
    GhostType::Friend21,
    GhostType::Friend22,
    GhostType::Friend23,
    GhostType::Friend24,
    GhostType::Friend25,
    GhostType::Friend26,
    GhostType::Friend27,
    GhostType::Friend28,
    GhostType::Friend29,
    GhostType::Friend30,
    GhostType::NormalStaff,
    GhostType::ExpertStaff,
];

pub const CHARACTERS: [Character; 48] = [
    Character::BabyMario,
    Character::BabyLuigi,
    Character::BabyPeach,
    Character::BabyDaisy,
    Character::Toad,
    Character::Toadette,
    Character::KoopaTroopa,
    Character::DryBones,
    Character::Mario,
    Character::Luigi,
    Character::Peach,
    Character::Daisy,
    Character::Yoshi,
    Character::Birdo,
    Character::DiddyKong,
    Character::BowserJr,
    Character::Wario,
    Character::Waluigi,
    Character::DonkeyKong,
    Character::Bowser,
    Character::KingBoo,
    Character::Rosalina,
    Character::FunkyKong,
    Character::DryBowser,
    Character::SmallMiiOutfitAMale,
    Character::SmallMiiOutfitAFemale,
    Character::SmallMiiOutfitBMale,
    Character::SmallMiiOutfitBFemale,
    Character::SmallMiiOutfitCMale,
    Character::SmallMiiOutfitCFemale,
    Character::MediumMiiOutfitAMale,
    Character::MediumMiiOutfitAFemale,
    Character::MediumMiiOutfitBMale,
    Character::MediumMiiOutfitBFemale,
    Character::MediumMiiOutfitCMale,
    Character::MediumMiiOutfitCFemale,
    Character::LargeMiiOutfitAMale,
    Character::LargeMiiOutfitAFemale,
    Character::LargeMiiOutfitBMale,
    Character::LargeMiiOutfitBFemale,
    Character::LargeMiiOutfitCMale,
    Character::LargeMiiOutfitCFemale,
    Character::SmallMii,
    Character::MediumMii,
    Character::LargeMii,
    Character::MenuPeach,
    Character::MenuDaisy,
    Character::MenuRosalina,
];

pub const VEHICLES: [Vehicle; 36] = [
    Vehicle::StandardKartS,
    Vehicle::StandardKartM,
    Vehicle::StandardKartL,
    Vehicle::BoosterSeat,
    Vehicle::ClassicDragster,
    Vehicle::Offroader,
    Vehicle::MiniBeast,
    Vehicle::WildWing,
    Vehicle::FlameFlyer,
    Vehicle::CheepCharger,
    Vehicle::SuperBlooper,
    Vehicle::PiranhaProwler,
    Vehicle::TinyTitan,
    Vehicle::Daytripper,
    Vehicle::Jetsetter,
    Vehicle::BlueFalcon,
    Vehicle::Sprinter,
    Vehicle::Honeycoupe,
    Vehicle::StandardBikeS,
    Vehicle::StandardBikeM,
    Vehicle::StandardBikeL,
    Vehicle::BulletBike,
    Vehicle::MachBike,
    Vehicle::FlameRunner,
    Vehicle::BitBike,
    Vehicle::Sugarscoot,
    Vehicle::WarioBike,
    Vehicle::Quacker,
    Vehicle::ZipZip,
    Vehicle::ShootingStar,
    Vehicle::Magikruiser,
    Vehicle::Sneakster,
    Vehicle::Spear,
    Vehicle::JetBubble,
    Vehicle::DolphinDasher,
    Vehicle::Phantom,
];
