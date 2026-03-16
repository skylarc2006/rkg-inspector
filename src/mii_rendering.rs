//! Adapted from <https://github.com/TeamWheelWizard/WheelWizard/blob/Offline-mii-rendering/WheelWizard/Features/MiiImages/MiiStudioDataSerializer.cs>

/// Maps a Wii Mii facial feature index (0–11) to a Mii Studio makeup value.
const MAKEUP_MAP: [u8; 12] = [0, 1, 6, 9, 0, 0, 0, 0, 0, 10, 0, 0];

/// Maps a Wii Mii facial feature index (0–11) to a Mii Studio wrinkles value
const WRINKLES_MAP: [u8; 12] = [0, 0, 0, 0, 5, 2, 3, 7, 8, 0, 9, 11];

/// Construct Mii Studio API URL from Mii bytes
pub fn get_mii_studio_url(mii_data: &[u8]) -> String {
    let data = encode_studio_data(&generate_studio_data_array(mii_data));

    format!("https://studio.mii.nintendo.com/miis/image.png?data={}&type=face&expression=normal&width=270", data)
}

/// Encodes the studio data array into the hex string format required by the API.
fn encode_studio_data(studio_data: &[u8]) -> String {
    let mut n: u8 = 0;
    let mut dest = String::with_capacity((studio_data.len() + 1) * 2);
    dest.push_str("00");

    for &b in studio_data {
        let eo = (7u8.wrapping_add(b ^ n)) & 0xFF;
        n = eo;
        dest.push_str(&format!("{:02x}", eo));
    }

    dest
}

fn read_u16_be(buf: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([buf[offset], buf[offset + 1]])
}

fn read_u32_be(buf: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]])
}

/// Parses the Wii Mii data and generates the 46-byte studio data array.
fn generate_studio_data_array(buf: &[u8]) -> [u8; 46] {
    let mut studio = [0u8; 46];

    // --- Basic Info ---
    let tmp_u16_0 = read_u16_be(buf, 0);
    let is_girl = ((tmp_u16_0 >> 14) & 1) == 1;
    let fav_color = ((tmp_u16_0 >> 1) & 0xF) as u8;
    let height = buf[0x16];
    let weight = buf[0x17];

    studio[0x16] = if is_girl { 1 } else { 0 }; // Gender
    studio[0x15] = fav_color;                    // Favorite Color
    studio[0x1E] = height;                       // Height
    studio[2] = weight;                          // Weight

    // --- Face ---
    let tmp_u16_20 = read_u16_be(buf, 0x20);
    let face_shape = (tmp_u16_20 >> 13) as u8;
    let skin_color = ((tmp_u16_20 >> 10) & 7) as u8;
    let facial_feature = ((tmp_u16_20 >> 6) & 0xF) as usize;
    let makeup = MAKEUP_MAP.get(facial_feature).copied().unwrap_or(0);
    let wrinkles = WRINKLES_MAP.get(facial_feature).copied().unwrap_or(0);

    studio[0x13] = face_shape;
    studio[0x11] = skin_color;
    studio[0x14] = wrinkles;
    studio[0x12] = makeup;

    // --- Hair ---
    let tmp_u16_22 = read_u16_be(buf, 0x22);
    let hair_style = (tmp_u16_22 >> 9) as u8;
    let hair_color = ((tmp_u16_22 >> 6) & 7) as u8;
    let flip_hair = ((tmp_u16_22 >> 5) & 1) as u8;

    studio[0x1D] = hair_style;
    studio[0x1B] = if hair_color == 0 { 8 } else { hair_color }; // Map color 0 to 8
    studio[0x1C] = flip_hair;

    // --- Eyebrows ---
    let tmp_u32_24 = read_u32_be(buf, 0x24);
    let eyebrow_style = (tmp_u32_24 >> 27) as u8;
    let eyebrow_rotation = ((tmp_u32_24 >> 22) & 0xF) as u8;
    let eyebrow_color = ((tmp_u32_24 >> 13) & 7) as u8;
    let eyebrow_scale = ((tmp_u32_24 >> 9) & 0xF) as u8;
    let eyebrow_y_scale: u8 = 3; // Hardcoded in JS
    let eyebrow_y_position = ((tmp_u32_24 >> 4) & 0x1F) as u8;
    let eyebrow_x_spacing = (tmp_u32_24 & 0xF) as u8;

    studio[0xE] = eyebrow_style;
    studio[0xC] = eyebrow_rotation;
    studio[0xB] = if eyebrow_color == 0 { 8 } else { eyebrow_color }; // Map color 0 to 8
    studio[0xD] = eyebrow_scale;
    studio[0xA] = eyebrow_y_scale;
    studio[0x10] = eyebrow_y_position;
    studio[0xF] = eyebrow_x_spacing;

    // --- Eyes ---
    let tmp_u32_28 = read_u32_be(buf, 0x28);
    let eye_style = (tmp_u32_28 >> 26) as u8;
    let eye_rotation = ((tmp_u32_28 >> 21) & 7) as u8;
    let eye_y_position = ((tmp_u32_28 >> 16) & 0x1F) as u8;
    let eye_color = ((tmp_u32_28 >> 13) & 7) as u8;
    let eye_scale = ((tmp_u32_28 >> 9) & 7) as u8;
    let eye_y_scale: u8 = 3; // Hardcoded in JS
    let eye_x_spacing = ((tmp_u32_28 >> 5) & 0xF) as u8;

    studio[7] = eye_style;
    studio[5] = eye_rotation;
    studio[9] = eye_y_position;
    studio[4] = eye_color + 8; // Map color 0-7 to 8-15
    studio[6] = eye_scale;
    studio[3] = eye_y_scale;
    studio[8] = eye_x_spacing;

    // --- Nose ---
    let tmp_u16_2c = read_u16_be(buf, 0x2C);
    let nose_style = (tmp_u16_2c >> 12) as u8;
    let nose_scale = ((tmp_u16_2c >> 8) & 0xF) as u8;
    let nose_y_position = ((tmp_u16_2c >> 3) & 0x1F) as u8;

    studio[0x2C] = nose_style;
    studio[0x2B] = nose_scale;
    studio[0x2D] = nose_y_position;

    // --- Mouth ---
    let tmp_u16_2e = read_u16_be(buf, 0x2E);
    let mouth_style = (tmp_u16_2e >> 11) as u8;
    let mouth_color = ((tmp_u16_2e >> 9) & 3) as u8;
    let mouth_scale = ((tmp_u16_2e >> 5) & 0xF) as u8;
    let mouth_y_scale: u8 = 3; // Hardcoded in JS
    let mouth_y_position = (tmp_u16_2e & 0x1F) as u8;

    studio[0x26] = mouth_style;
    studio[0x24] = if mouth_color < 4 { mouth_color + 19 } else { 0 }; // Map 0-3 to 19-22
    studio[0x25] = mouth_scale;
    studio[0x23] = mouth_y_scale;
    studio[0x27] = mouth_y_position;

    // --- Beard / Mustache ---
    let tmp_u16_32 = read_u16_be(buf, 0x32);
    let mustache_style = (tmp_u16_32 >> 14) as u8;
    let beard_style = ((tmp_u16_32 >> 12) & 3) as u8;
    let facial_hair_color = ((tmp_u16_32 >> 9) & 7) as u8;
    let mustache_scale = ((tmp_u16_32 >> 5) & 0xF) as u8;
    let mustache_y_position = (tmp_u16_32 & 0x1F) as u8;

    studio[0x29] = mustache_style;
    studio[1] = beard_style;
    studio[0] = if facial_hair_color == 0 { 8 } else { facial_hair_color }; // Map color 0 to 8
    studio[0x28] = mustache_scale;
    studio[0x2A] = mustache_y_position;

    // --- Glasses ---
    let tmp_u16_30 = read_u16_be(buf, 0x30);
    let glasses_style = (tmp_u16_30 >> 12) as u8;
    let glasses_color = ((tmp_u16_30 >> 9) & 7) as u8;
    let glasses_scale = ((tmp_u16_30 >> 5) & 7) as u8;
    let glasses_y_position = (tmp_u16_30 & 0x1F) as u8;

    let mapped_glasses_color = if glasses_color == 0 {
        8 // black -> 8
    } else if glasses_color < 6 {
        glasses_color + 13 // 1-5 -> 14-18
    } else {
        0 // 6, 7 -> 0
    };

    studio[0x19] = glasses_style;
    studio[0x17] = mapped_glasses_color;
    studio[0x18] = glasses_scale;
    studio[0x1A] = glasses_y_position;

    // --- Mole ---
    let tmp_u16_34 = read_u16_be(buf, 0x34);
    let enable_mole = (tmp_u16_34 >> 15) as u8;
    let mole_scale = ((tmp_u16_34 >> 11) & 0xF) as u8;
    let mole_y_position = ((tmp_u16_34 >> 6) & 0x1F) as u8;
    let mole_x_position = ((tmp_u16_34 >> 1) & 0x1F) as u8;

    studio[0x20] = enable_mole;
    studio[0x1F] = mole_scale;
    studio[0x22] = mole_y_position;
    studio[0x21] = mole_x_position;

    studio
}
