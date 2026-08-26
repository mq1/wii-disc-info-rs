// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionCode {
    SystemWiiChannels,
    UfouriaTheSagaNA,
    Germany,
    USA,
    France,
    NetherlandsEuropeAlternateLanguages,
    Italy,
    Japan,
    Korea,
    JapaneseImportToEuropeAustraliaAndOtherPALRegions,
    AmericanImportToEuropeAustraliaAndOtherPALRegions,
    JapaneseImportToUSAAndOtherNTSCRegions,
    EuropeAndOtherPALRegionsSuchAsAustralia,
    JapaneseVirtualConsoleImportToKorea,
    Russia,
    Spain,
    AmericanVirtualConsoleImportToKorea,
    AustraliaEuropeAlternateLanguages,
    Scandinavia,
    RepublicOfChinaTaiwanHongKongMacau,
    EuropeAlternateLanguagesUSSpecialReleases,
    Unknown,
}

impl RegionCode {
    pub const fn from_region_byte(region_byte: u8) -> Self {
        match region_byte {
            b'A' => RegionCode::SystemWiiChannels,
            b'B' => RegionCode::UfouriaTheSagaNA,
            b'D' => RegionCode::Germany,
            b'E' => RegionCode::USA,
            b'F' => RegionCode::France,
            b'H' => RegionCode::NetherlandsEuropeAlternateLanguages,
            b'I' => RegionCode::Italy,
            b'J' => RegionCode::Japan,
            b'K' => RegionCode::Korea,
            b'L' => RegionCode::JapaneseImportToEuropeAustraliaAndOtherPALRegions,
            b'M' => RegionCode::AmericanImportToEuropeAustraliaAndOtherPALRegions,
            b'N' => RegionCode::JapaneseImportToUSAAndOtherNTSCRegions,
            b'P' => RegionCode::EuropeAndOtherPALRegionsSuchAsAustralia,
            b'Q' => RegionCode::JapaneseVirtualConsoleImportToKorea,
            b'R' => RegionCode::Russia,
            b'S' => RegionCode::Spain,
            b'T' => RegionCode::AmericanVirtualConsoleImportToKorea,
            b'U' => RegionCode::AustraliaEuropeAlternateLanguages,
            b'V' => RegionCode::Scandinavia,
            b'W' => RegionCode::RepublicOfChinaTaiwanHongKongMacau,
            b'X' | b'Y' | b'Z' => RegionCode::EuropeAlternateLanguagesUSSpecialReleases,
            _ => RegionCode::Unknown,
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::SystemWiiChannels => "System Wii Channels (i.e. Mii Channel)",
            Self::UfouriaTheSagaNA => "Ufouria: The Saga (NA)",
            Self::Germany => "Germany",
            Self::USA => "USA",
            Self::France => "France",
            Self::NetherlandsEuropeAlternateLanguages => "Netherlands / Europe alternate languages",
            Self::Italy => "Italy",
            Self::Japan => "Japan",
            Self::Korea => "Korea",
            Self::JapaneseImportToEuropeAustraliaAndOtherPALRegions => {
                "Japanese import to Europe, Australia and other PAL regions"
            }
            Self::AmericanImportToEuropeAustraliaAndOtherPALRegions => {
                "American import to Europe, Australia and other PAL regions"
            }
            Self::JapaneseImportToUSAAndOtherNTSCRegions => {
                "Japanese import to USA and other NTSC regions"
            }
            Self::EuropeAndOtherPALRegionsSuchAsAustralia => {
                "Europe and other PAL regions such as Australia"
            }
            Self::JapaneseVirtualConsoleImportToKorea => "Japanese Virtual Console import to Korea",
            Self::Russia => "Russia",
            Self::Spain => "Spain",
            Self::AmericanVirtualConsoleImportToKorea => "American Virtual Console import to Korea",
            Self::AustraliaEuropeAlternateLanguages => "Australia / Europe alternate languages",
            Self::Scandinavia => "Scandinavia",
            Self::RepublicOfChinaTaiwanHongKongMacau => {
                "Republic of China (Taiwan) / Hong Kong / Macau"
            }
            Self::EuropeAlternateLanguagesUSSpecialReleases => {
                "Europe alternate languages / US special releases"
            }
            Self::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for RegionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for RegionCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
