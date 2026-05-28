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

impl std::fmt::Display for RegionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
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
        };

        f.write_str(name)
    }
}

impl From<u8> for RegionCode {
    fn from(b: u8) -> Self {
        match b {
            b'A' => Self::SystemWiiChannels,
            b'B' => Self::UfouriaTheSagaNA,
            b'D' => Self::Germany,
            b'E' => Self::USA,
            b'F' => Self::France,
            b'H' => Self::NetherlandsEuropeAlternateLanguages,
            b'I' => Self::Italy,
            b'J' => Self::Japan,
            b'K' => Self::Korea,
            b'L' => Self::JapaneseImportToEuropeAustraliaAndOtherPALRegions,
            b'M' => Self::AmericanImportToEuropeAustraliaAndOtherPALRegions,
            b'N' => Self::JapaneseImportToUSAAndOtherNTSCRegions,
            b'P' => Self::EuropeAndOtherPALRegionsSuchAsAustralia,
            b'Q' => Self::JapaneseVirtualConsoleImportToKorea,
            b'R' => Self::Russia,
            b'S' => Self::Spain,
            b'T' => Self::AmericanVirtualConsoleImportToKorea,
            b'U' => Self::AustraliaEuropeAlternateLanguages,
            b'V' => Self::Scandinavia,
            b'W' => Self::RepublicOfChinaTaiwanHongKongMacau,
            b'X' | b'Y' | b'Z' => Self::EuropeAlternateLanguagesUSSpecialReleases,
            _ => Self::Unknown,
        }
    }
}
