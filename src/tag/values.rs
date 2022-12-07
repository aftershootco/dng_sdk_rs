#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LightSource {
    Unknown = 0,
    Daylight = 1,
    Fluorescent = 2,
    Tungsten = 3,
    Flash = 4,
    FineWeather = 9,
    CloudyWeather = 10,
    Shade = 11,
    DaylightFluorescent = 12,  // D 5700 - 7100K
    DayWhiteFluorescent = 13,  // N 4600 - 5500K
    CoolWhiteFluorescent = 14, // W 3800 - 4500K
    WhiteFluorescent = 15,     // WW 3250 - 3800K
    WarmWhiteFluorescent = 16, // L 2600 - 3250K
    StandardLightA = 17,
    StandardLightB = 18,
    StandardLightC = 19,
    D55 = 20,
    D65 = 21,
    D75 = 22,
    D50 = 23,
    ISOStudioTungsten = 24,
    Other = 255,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Magic {
    Tiff = 42,                // Tiff (and Dng)
    BigTiff = 43,             // BigTiff (and BigDNG)
    ExtendedProfile = 0x4352, // 'CR'
    // Other raw formats - included here so the DNG SDK can parse them.
    Panasonic = 85,
    OlympusA = 0x4F52,
    OlympusB = 0x5352,
}
