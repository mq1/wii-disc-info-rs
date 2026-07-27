// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

use strum_macros::{Display, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, IntoStaticStr)]
pub enum RegionCode {
    #[strum(to_string = "System Wii Channels (i.e. Mii Channel)")]
    SystemWiiChannels,

    #[strum(to_string = "Ufouria: The Saga (NA)")]
    UfouriaTheSagaNA,

    #[strum(to_string = "Germany")]
    Germany,

    #[strum(to_string = "USA")]
    USA,

    #[strum(to_string = "France")]
    France,

    #[strum(to_string = "Netherlands / Europe alternate languages")]
    NetherlandsEuropeAlternateLanguages,

    #[strum(to_string = "Italy")]
    Italy,

    #[strum(to_string = "Japan")]
    Japan,

    #[strum(to_string = "Korea")]
    Korea,

    #[strum(to_string = "Japanese import to Europe, Australia and other PAL regions")]
    JapaneseImportToEuropeAustraliaAndOtherPALRegions,

    #[strum(to_string = "American import to Europe, Australia and other PAL regions")]
    AmericanImportToEuropeAustraliaAndOtherPALRegions,

    #[strum(to_string = "Japanese import to USA and other NTSC regions")]
    JapaneseImportToUSAAndOtherNTSCRegions,

    #[strum(to_string = "Europe and other PAL regions such as Australia")]
    EuropeAndOtherPALRegionsSuchAsAustralia,

    #[strum(to_string = "Japanese Virtual Console import to Korea")]
    JapaneseVirtualConsoleImportToKorea,

    #[strum(to_string = "Russia")]
    Russia,

    #[strum(to_string = "Spain")]
    Spain,

    #[strum(to_string = "American Virtual Console import to Korea")]
    AmericanVirtualConsoleImportToKorea,

    #[strum(to_string = "Australia / Europe alternate languages")]
    AustraliaEuropeAlternateLanguages,

    #[strum(to_string = "Scandinavia")]
    Scandinavia,

    #[strum(to_string = "Republic of China (Taiwan) / Hong Kong / Macau")]
    RepublicOfChinaTaiwanHongKongMacau,

    #[strum(to_string = "Europe alternate languages / US special releases")]
    EuropeAlternateLanguagesUSSpecialReleases,

    #[strum(to_string = "Unknown")]
    Unknown,
}

impl From<char> for RegionCode {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::SystemWiiChannels,
            'B' => Self::UfouriaTheSagaNA,
            'D' => Self::Germany,
            'E' => Self::USA,
            'F' => Self::France,
            'H' => Self::NetherlandsEuropeAlternateLanguages,
            'I' => Self::Italy,
            'J' => Self::Japan,
            'K' => Self::Korea,
            'L' => Self::JapaneseImportToEuropeAustraliaAndOtherPALRegions,
            'M' => Self::AmericanImportToEuropeAustraliaAndOtherPALRegions,
            'N' => Self::JapaneseImportToUSAAndOtherNTSCRegions,
            'P' => Self::EuropeAndOtherPALRegionsSuchAsAustralia,
            'Q' => Self::JapaneseVirtualConsoleImportToKorea,
            'R' => Self::Russia,
            'S' => Self::Spain,
            'T' => Self::AmericanVirtualConsoleImportToKorea,
            'U' => Self::AustraliaEuropeAlternateLanguages,
            'V' => Self::Scandinavia,
            'W' => Self::RepublicOfChinaTaiwanHongKongMacau,
            'X' | 'Y' | 'Z' => Self::EuropeAlternateLanguagesUSSpecialReleases,
            _ => Self::Unknown,
        }
    }
}

impl From<u8> for RegionCode {
    fn from(b: u8) -> Self {
        Self::from(b as char)
    }
}
