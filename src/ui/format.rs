use rkg_utils::{footer::ctgp_footer::Region, header::mii::FavoriteColor};

pub fn favorite_color_string(f: FavoriteColor) -> &'static str {
    match f {
        FavoriteColor::Red => "Red",
        FavoriteColor::Orange => "Orange",
        FavoriteColor::Yellow => "Yellow",
        FavoriteColor::LimeGreen => "Light Green",
        FavoriteColor::ForestGreen => "Green",
        FavoriteColor::RoyalBlue => "Blue",
        FavoriteColor::SkyBlue => "Light Blue",
        FavoriteColor::Pink => "Pink",
        FavoriteColor::Purple => "Purple",
        FavoriteColor::Brown => "Brown",
        FavoriteColor::White => "White",
        FavoriteColor::Black => "Black",
    }
}

pub fn disc_region_string(disc_region: Region) -> &'static str {
    match disc_region {
        Region::NtscU => "E (NTSC-U)",
        Region::Pal => "P (PAL)",
        Region::NtscJ => "J (NTSC-J)",
        Region::Unknown => "Unknown/invalid",
    }
}
