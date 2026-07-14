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
    fn as_str(&self) -> &'static str {
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
