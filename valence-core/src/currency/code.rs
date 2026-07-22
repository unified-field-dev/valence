//! ISO 4217 alphabetic currency codes.
//!
//! Maintained in Rust (not generated from CSV). Additive ISO updates land as crate releases.
//!
//! Wire / serde form is the uppercase alphabetic string (e.g. `"USD"`).

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// ISO 4217 alphabetic currency code.
///
/// Parsing accepts **canonical uppercase** spellings only (e.g. `"USD"`, not `"usd"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrencyCode {
    /// ISO 4217 `AED`.
    Aed,
    /// ISO 4217 `AFN`.
    Afn,
    /// ISO 4217 `ALL`.
    All,
    /// ISO 4217 `AMD`.
    Amd,
    /// ISO 4217 `ANG`.
    Ang,
    /// ISO 4217 `AOA`.
    Aoa,
    /// ISO 4217 `ARS`.
    Ars,
    /// ISO 4217 `AUD`.
    Aud,
    /// ISO 4217 `AWG`.
    Awg,
    /// ISO 4217 `AZN`.
    Azn,
    /// ISO 4217 `BAM`.
    Bam,
    /// ISO 4217 `BBD`.
    Bbd,
    /// ISO 4217 `BDT`.
    Bdt,
    /// ISO 4217 `BGN`.
    Bgn,
    /// ISO 4217 `BHD`.
    Bhd,
    /// ISO 4217 `BIF`.
    Bif,
    /// ISO 4217 `BMD`.
    Bmd,
    /// ISO 4217 `BND`.
    Bnd,
    /// ISO 4217 `BOB`.
    Bob,
    /// ISO 4217 `BOV`.
    Bov,
    /// ISO 4217 `BRL`.
    Brl,
    /// ISO 4217 `BSD`.
    Bsd,
    /// ISO 4217 `BTN`.
    Btn,
    /// ISO 4217 `BWP`.
    Bwp,
    /// ISO 4217 `BYN`.
    Byn,
    /// ISO 4217 `BZD`.
    Bzd,
    /// ISO 4217 `CAD`.
    Cad,
    /// ISO 4217 `CDF`.
    Cdf,
    /// ISO 4217 `CHE`.
    Che,
    /// ISO 4217 `CHF`.
    Chf,
    /// ISO 4217 `CHW`.
    Chw,
    /// ISO 4217 `CLF`.
    Clf,
    /// ISO 4217 `CLP`.
    Clp,
    /// ISO 4217 `CNY`.
    Cny,
    /// ISO 4217 `COP`.
    Cop,
    /// ISO 4217 `COU`.
    Cou,
    /// ISO 4217 `CRC`.
    Crc,
    /// ISO 4217 `CUC`.
    Cuc,
    /// ISO 4217 `CUP`.
    Cup,
    /// ISO 4217 `CVE`.
    Cve,
    /// ISO 4217 `CZK`.
    Czk,
    /// ISO 4217 `DJF`.
    Djf,
    /// ISO 4217 `DKK`.
    Dkk,
    /// ISO 4217 `DOP`.
    Dop,
    /// ISO 4217 `DZD`.
    Dzd,
    /// ISO 4217 `EGP`.
    Egp,
    /// ISO 4217 `ERN`.
    Ern,
    /// ISO 4217 `ETB`.
    Etb,
    /// ISO 4217 `EUR`.
    Eur,
    /// ISO 4217 `FJD`.
    Fjd,
    /// ISO 4217 `FKP`.
    Fkp,
    /// ISO 4217 `GBP`.
    Gbp,
    /// ISO 4217 `GEL`.
    Gel,
    /// ISO 4217 `GHS`.
    Ghs,
    /// ISO 4217 `GIP`.
    Gip,
    /// ISO 4217 `GMD`.
    Gmd,
    /// ISO 4217 `GNF`.
    Gnf,
    /// ISO 4217 `GTQ`.
    Gtq,
    /// ISO 4217 `GYD`.
    Gyd,
    /// ISO 4217 `HKD`.
    Hkd,
    /// ISO 4217 `HNL`.
    Hnl,
    /// ISO 4217 `HTG`.
    Htg,
    /// ISO 4217 `HUF`.
    Huf,
    /// ISO 4217 `IDR`.
    Idr,
    /// ISO 4217 `ILS`.
    Ils,
    /// ISO 4217 `INR`.
    Inr,
    /// ISO 4217 `IQD`.
    Iqd,
    /// ISO 4217 `IRR`.
    Irr,
    /// ISO 4217 `ISK`.
    Isk,
    /// ISO 4217 `JMD`.
    Jmd,
    /// ISO 4217 `JOD`.
    Jod,
    /// ISO 4217 `JPY`.
    Jpy,
    /// ISO 4217 `KES`.
    Kes,
    /// ISO 4217 `KGS`.
    Kgs,
    /// ISO 4217 `KHR`.
    Khr,
    /// ISO 4217 `KMF`.
    Kmf,
    /// ISO 4217 `KPW`.
    Kpw,
    /// ISO 4217 `KRW`.
    Krw,
    /// ISO 4217 `KWD`.
    Kwd,
    /// ISO 4217 `KYD`.
    Kyd,
    /// ISO 4217 `KZT`.
    Kzt,
    /// ISO 4217 `LAK`.
    Lak,
    /// ISO 4217 `LBP`.
    Lbp,
    /// ISO 4217 `LKR`.
    Lkr,
    /// ISO 4217 `LRD`.
    Lrd,
    /// ISO 4217 `LSL`.
    Lsl,
    /// ISO 4217 `LYD`.
    Lyd,
    /// ISO 4217 `MAD`.
    Mad,
    /// ISO 4217 `MDL`.
    Mdl,
    /// ISO 4217 `MGA`.
    Mga,
    /// ISO 4217 `MKD`.
    Mkd,
    /// ISO 4217 `MMK`.
    Mmk,
    /// ISO 4217 `MNT`.
    Mnt,
    /// ISO 4217 `MOP`.
    Mop,
    /// ISO 4217 `MRU`.
    Mru,
    /// ISO 4217 `MUR`.
    Mur,
    /// ISO 4217 `MVR`.
    Mvr,
    /// ISO 4217 `MWK`.
    Mwk,
    /// ISO 4217 `MXN`.
    Mxn,
    /// ISO 4217 `MXV`.
    Mxv,
    /// ISO 4217 `MYR`.
    Myr,
    /// ISO 4217 `MZN`.
    Mzn,
    /// ISO 4217 `NAD`.
    Nad,
    /// ISO 4217 `NGN`.
    Ngn,
    /// ISO 4217 `NIO`.
    Nio,
    /// ISO 4217 `NOK`.
    Nok,
    /// ISO 4217 `NPR`.
    Npr,
    /// ISO 4217 `NZD`.
    Nzd,
    /// ISO 4217 `OMR`.
    Omr,
    /// ISO 4217 `PAB`.
    Pab,
    /// ISO 4217 `PEN`.
    Pen,
    /// ISO 4217 `PGK`.
    Pgk,
    /// ISO 4217 `PHP`.
    Php,
    /// ISO 4217 `PKR`.
    Pkr,
    /// ISO 4217 `PLN`.
    Pln,
    /// ISO 4217 `PYG`.
    Pyg,
    /// ISO 4217 `QAR`.
    Qar,
    /// ISO 4217 `RON`.
    Ron,
    /// ISO 4217 `RSD`.
    Rsd,
    /// ISO 4217 `RUB`.
    Rub,
    /// ISO 4217 `RWF`.
    Rwf,
    /// ISO 4217 `SAR`.
    Sar,
    /// ISO 4217 `SBD`.
    Sbd,
    /// ISO 4217 `SCR`.
    Scr,
    /// ISO 4217 `SDG`.
    Sdg,
    /// ISO 4217 `SEK`.
    Sek,
    /// ISO 4217 `SGD`.
    Sgd,
    /// ISO 4217 `SHP`.
    Shp,
    /// ISO 4217 `SLE`.
    Sle,
    /// ISO 4217 `SOS`.
    Sos,
    /// ISO 4217 `SRD`.
    Srd,
    /// ISO 4217 `SSP`.
    Ssp,
    /// ISO 4217 `STN`.
    Stn,
    /// ISO 4217 `SVC`.
    Svc,
    /// ISO 4217 `SYP`.
    Syp,
    /// ISO 4217 `SZL`.
    Szl,
    /// ISO 4217 `THB`.
    Thb,
    /// ISO 4217 `TJS`.
    Tjs,
    /// ISO 4217 `TMT`.
    Tmt,
    /// ISO 4217 `TND`.
    Tnd,
    /// ISO 4217 `TOP`.
    Top,
    /// ISO 4217 `TRY`.
    Try,
    /// ISO 4217 `TTD`.
    Ttd,
    /// ISO 4217 `TWD`.
    Twd,
    /// ISO 4217 `TZS`.
    Tzs,
    /// ISO 4217 `UAH`.
    Uah,
    /// ISO 4217 `UGX`.
    Ugx,
    /// ISO 4217 `USD`.
    Usd,
    /// ISO 4217 `USN`.
    Usn,
    /// ISO 4217 `UYI`.
    Uyi,
    /// ISO 4217 `UYU`.
    Uyu,
    /// ISO 4217 `UYW`.
    Uyw,
    /// ISO 4217 `UZS`.
    Uzs,
    /// ISO 4217 `VED`.
    Ved,
    /// ISO 4217 `VES`.
    Ves,
    /// ISO 4217 `VND`.
    Vnd,
    /// ISO 4217 `VUV`.
    Vuv,
    /// ISO 4217 `WST`.
    Wst,
    /// ISO 4217 `XAF`.
    Xaf,
    /// ISO 4217 `XAG`.
    Xag,
    /// ISO 4217 `XAU`.
    Xau,
    /// ISO 4217 `XBA`.
    Xba,
    /// ISO 4217 `XBB`.
    Xbb,
    /// ISO 4217 `XBC`.
    Xbc,
    /// ISO 4217 `XBD`.
    Xbd,
    /// ISO 4217 `XCD`.
    Xcd,
    /// ISO 4217 `XDR`.
    Xdr,
    /// ISO 4217 `XOF`.
    Xof,
    /// ISO 4217 `XPD`.
    Xpd,
    /// ISO 4217 `XPF`.
    Xpf,
    /// ISO 4217 `XPT`.
    Xpt,
    /// ISO 4217 `XSU`.
    Xsu,
    /// ISO 4217 `XTS`.
    Xts,
    /// ISO 4217 `XUA`.
    Xua,
    /// ISO 4217 `XXX`.
    Xxx,
    /// ISO 4217 `YER`.
    Yer,
    /// ISO 4217 `ZAR`.
    Zar,
    /// ISO 4217 `ZMW`.
    Zmw,
    /// ISO 4217 `ZWG`.
    Zwg,
}

impl CurrencyCode {
    /// All known codes in this crate version.
    pub const fn all() -> &'static [CurrencyCode] {
        &[
            Self::Aed,
            Self::Afn,
            Self::All,
            Self::Amd,
            Self::Ang,
            Self::Aoa,
            Self::Ars,
            Self::Aud,
            Self::Awg,
            Self::Azn,
            Self::Bam,
            Self::Bbd,
            Self::Bdt,
            Self::Bgn,
            Self::Bhd,
            Self::Bif,
            Self::Bmd,
            Self::Bnd,
            Self::Bob,
            Self::Bov,
            Self::Brl,
            Self::Bsd,
            Self::Btn,
            Self::Bwp,
            Self::Byn,
            Self::Bzd,
            Self::Cad,
            Self::Cdf,
            Self::Che,
            Self::Chf,
            Self::Chw,
            Self::Clf,
            Self::Clp,
            Self::Cny,
            Self::Cop,
            Self::Cou,
            Self::Crc,
            Self::Cuc,
            Self::Cup,
            Self::Cve,
            Self::Czk,
            Self::Djf,
            Self::Dkk,
            Self::Dop,
            Self::Dzd,
            Self::Egp,
            Self::Ern,
            Self::Etb,
            Self::Eur,
            Self::Fjd,
            Self::Fkp,
            Self::Gbp,
            Self::Gel,
            Self::Ghs,
            Self::Gip,
            Self::Gmd,
            Self::Gnf,
            Self::Gtq,
            Self::Gyd,
            Self::Hkd,
            Self::Hnl,
            Self::Htg,
            Self::Huf,
            Self::Idr,
            Self::Ils,
            Self::Inr,
            Self::Iqd,
            Self::Irr,
            Self::Isk,
            Self::Jmd,
            Self::Jod,
            Self::Jpy,
            Self::Kes,
            Self::Kgs,
            Self::Khr,
            Self::Kmf,
            Self::Kpw,
            Self::Krw,
            Self::Kwd,
            Self::Kyd,
            Self::Kzt,
            Self::Lak,
            Self::Lbp,
            Self::Lkr,
            Self::Lrd,
            Self::Lsl,
            Self::Lyd,
            Self::Mad,
            Self::Mdl,
            Self::Mga,
            Self::Mkd,
            Self::Mmk,
            Self::Mnt,
            Self::Mop,
            Self::Mru,
            Self::Mur,
            Self::Mvr,
            Self::Mwk,
            Self::Mxn,
            Self::Mxv,
            Self::Myr,
            Self::Mzn,
            Self::Nad,
            Self::Ngn,
            Self::Nio,
            Self::Nok,
            Self::Npr,
            Self::Nzd,
            Self::Omr,
            Self::Pab,
            Self::Pen,
            Self::Pgk,
            Self::Php,
            Self::Pkr,
            Self::Pln,
            Self::Pyg,
            Self::Qar,
            Self::Ron,
            Self::Rsd,
            Self::Rub,
            Self::Rwf,
            Self::Sar,
            Self::Sbd,
            Self::Scr,
            Self::Sdg,
            Self::Sek,
            Self::Sgd,
            Self::Shp,
            Self::Sle,
            Self::Sos,
            Self::Srd,
            Self::Ssp,
            Self::Stn,
            Self::Svc,
            Self::Syp,
            Self::Szl,
            Self::Thb,
            Self::Tjs,
            Self::Tmt,
            Self::Tnd,
            Self::Top,
            Self::Try,
            Self::Ttd,
            Self::Twd,
            Self::Tzs,
            Self::Uah,
            Self::Ugx,
            Self::Usd,
            Self::Usn,
            Self::Uyi,
            Self::Uyu,
            Self::Uyw,
            Self::Uzs,
            Self::Ved,
            Self::Ves,
            Self::Vnd,
            Self::Vuv,
            Self::Wst,
            Self::Xaf,
            Self::Xag,
            Self::Xau,
            Self::Xba,
            Self::Xbb,
            Self::Xbc,
            Self::Xbd,
            Self::Xcd,
            Self::Xdr,
            Self::Xof,
            Self::Xpd,
            Self::Xpf,
            Self::Xpt,
            Self::Xsu,
            Self::Xts,
            Self::Xua,
            Self::Xxx,
            Self::Yer,
            Self::Zar,
            Self::Zmw,
            Self::Zwg,
        ]
    }

    /// Alphabetic code (`"USD"`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Aed => "AED",
            Self::Afn => "AFN",
            Self::All => "ALL",
            Self::Amd => "AMD",
            Self::Ang => "ANG",
            Self::Aoa => "AOA",
            Self::Ars => "ARS",
            Self::Aud => "AUD",
            Self::Awg => "AWG",
            Self::Azn => "AZN",
            Self::Bam => "BAM",
            Self::Bbd => "BBD",
            Self::Bdt => "BDT",
            Self::Bgn => "BGN",
            Self::Bhd => "BHD",
            Self::Bif => "BIF",
            Self::Bmd => "BMD",
            Self::Bnd => "BND",
            Self::Bob => "BOB",
            Self::Bov => "BOV",
            Self::Brl => "BRL",
            Self::Bsd => "BSD",
            Self::Btn => "BTN",
            Self::Bwp => "BWP",
            Self::Byn => "BYN",
            Self::Bzd => "BZD",
            Self::Cad => "CAD",
            Self::Cdf => "CDF",
            Self::Che => "CHE",
            Self::Chf => "CHF",
            Self::Chw => "CHW",
            Self::Clf => "CLF",
            Self::Clp => "CLP",
            Self::Cny => "CNY",
            Self::Cop => "COP",
            Self::Cou => "COU",
            Self::Crc => "CRC",
            Self::Cuc => "CUC",
            Self::Cup => "CUP",
            Self::Cve => "CVE",
            Self::Czk => "CZK",
            Self::Djf => "DJF",
            Self::Dkk => "DKK",
            Self::Dop => "DOP",
            Self::Dzd => "DZD",
            Self::Egp => "EGP",
            Self::Ern => "ERN",
            Self::Etb => "ETB",
            Self::Eur => "EUR",
            Self::Fjd => "FJD",
            Self::Fkp => "FKP",
            Self::Gbp => "GBP",
            Self::Gel => "GEL",
            Self::Ghs => "GHS",
            Self::Gip => "GIP",
            Self::Gmd => "GMD",
            Self::Gnf => "GNF",
            Self::Gtq => "GTQ",
            Self::Gyd => "GYD",
            Self::Hkd => "HKD",
            Self::Hnl => "HNL",
            Self::Htg => "HTG",
            Self::Huf => "HUF",
            Self::Idr => "IDR",
            Self::Ils => "ILS",
            Self::Inr => "INR",
            Self::Iqd => "IQD",
            Self::Irr => "IRR",
            Self::Isk => "ISK",
            Self::Jmd => "JMD",
            Self::Jod => "JOD",
            Self::Jpy => "JPY",
            Self::Kes => "KES",
            Self::Kgs => "KGS",
            Self::Khr => "KHR",
            Self::Kmf => "KMF",
            Self::Kpw => "KPW",
            Self::Krw => "KRW",
            Self::Kwd => "KWD",
            Self::Kyd => "KYD",
            Self::Kzt => "KZT",
            Self::Lak => "LAK",
            Self::Lbp => "LBP",
            Self::Lkr => "LKR",
            Self::Lrd => "LRD",
            Self::Lsl => "LSL",
            Self::Lyd => "LYD",
            Self::Mad => "MAD",
            Self::Mdl => "MDL",
            Self::Mga => "MGA",
            Self::Mkd => "MKD",
            Self::Mmk => "MMK",
            Self::Mnt => "MNT",
            Self::Mop => "MOP",
            Self::Mru => "MRU",
            Self::Mur => "MUR",
            Self::Mvr => "MVR",
            Self::Mwk => "MWK",
            Self::Mxn => "MXN",
            Self::Mxv => "MXV",
            Self::Myr => "MYR",
            Self::Mzn => "MZN",
            Self::Nad => "NAD",
            Self::Ngn => "NGN",
            Self::Nio => "NIO",
            Self::Nok => "NOK",
            Self::Npr => "NPR",
            Self::Nzd => "NZD",
            Self::Omr => "OMR",
            Self::Pab => "PAB",
            Self::Pen => "PEN",
            Self::Pgk => "PGK",
            Self::Php => "PHP",
            Self::Pkr => "PKR",
            Self::Pln => "PLN",
            Self::Pyg => "PYG",
            Self::Qar => "QAR",
            Self::Ron => "RON",
            Self::Rsd => "RSD",
            Self::Rub => "RUB",
            Self::Rwf => "RWF",
            Self::Sar => "SAR",
            Self::Sbd => "SBD",
            Self::Scr => "SCR",
            Self::Sdg => "SDG",
            Self::Sek => "SEK",
            Self::Sgd => "SGD",
            Self::Shp => "SHP",
            Self::Sle => "SLE",
            Self::Sos => "SOS",
            Self::Srd => "SRD",
            Self::Ssp => "SSP",
            Self::Stn => "STN",
            Self::Svc => "SVC",
            Self::Syp => "SYP",
            Self::Szl => "SZL",
            Self::Thb => "THB",
            Self::Tjs => "TJS",
            Self::Tmt => "TMT",
            Self::Tnd => "TND",
            Self::Top => "TOP",
            Self::Try => "TRY",
            Self::Ttd => "TTD",
            Self::Twd => "TWD",
            Self::Tzs => "TZS",
            Self::Uah => "UAH",
            Self::Ugx => "UGX",
            Self::Usd => "USD",
            Self::Usn => "USN",
            Self::Uyi => "UYI",
            Self::Uyu => "UYU",
            Self::Uyw => "UYW",
            Self::Uzs => "UZS",
            Self::Ved => "VED",
            Self::Ves => "VES",
            Self::Vnd => "VND",
            Self::Vuv => "VUV",
            Self::Wst => "WST",
            Self::Xaf => "XAF",
            Self::Xag => "XAG",
            Self::Xau => "XAU",
            Self::Xba => "XBA",
            Self::Xbb => "XBB",
            Self::Xbc => "XBC",
            Self::Xbd => "XBD",
            Self::Xcd => "XCD",
            Self::Xdr => "XDR",
            Self::Xof => "XOF",
            Self::Xpd => "XPD",
            Self::Xpf => "XPF",
            Self::Xpt => "XPT",
            Self::Xsu => "XSU",
            Self::Xts => "XTS",
            Self::Xua => "XUA",
            Self::Xxx => "XXX",
            Self::Yer => "YER",
            Self::Zar => "ZAR",
            Self::Zmw => "ZMW",
            Self::Zwg => "ZWG",
        }
    }

    /// ISO 4217 minor-unit exponent (digits after the decimal in major units).
    pub const fn exponent(self) -> u32 {
        match self {
            Self::Aed => 2,
            Self::Afn => 2,
            Self::All => 2,
            Self::Amd => 2,
            Self::Ang => 2,
            Self::Aoa => 2,
            Self::Ars => 2,
            Self::Aud => 2,
            Self::Awg => 2,
            Self::Azn => 2,
            Self::Bam => 2,
            Self::Bbd => 2,
            Self::Bdt => 2,
            Self::Bgn => 2,
            Self::Bhd => 3,
            Self::Bif => 0,
            Self::Bmd => 2,
            Self::Bnd => 2,
            Self::Bob => 2,
            Self::Bov => 2,
            Self::Brl => 2,
            Self::Bsd => 2,
            Self::Btn => 2,
            Self::Bwp => 2,
            Self::Byn => 2,
            Self::Bzd => 2,
            Self::Cad => 2,
            Self::Cdf => 2,
            Self::Che => 2,
            Self::Chf => 2,
            Self::Chw => 2,
            Self::Clf => 4,
            Self::Clp => 0,
            Self::Cny => 2,
            Self::Cop => 2,
            Self::Cou => 2,
            Self::Crc => 2,
            Self::Cuc => 2,
            Self::Cup => 2,
            Self::Cve => 2,
            Self::Czk => 2,
            Self::Djf => 0,
            Self::Dkk => 2,
            Self::Dop => 2,
            Self::Dzd => 2,
            Self::Egp => 2,
            Self::Ern => 2,
            Self::Etb => 2,
            Self::Eur => 2,
            Self::Fjd => 2,
            Self::Fkp => 2,
            Self::Gbp => 2,
            Self::Gel => 2,
            Self::Ghs => 2,
            Self::Gip => 2,
            Self::Gmd => 2,
            Self::Gnf => 0,
            Self::Gtq => 2,
            Self::Gyd => 2,
            Self::Hkd => 2,
            Self::Hnl => 2,
            Self::Htg => 2,
            Self::Huf => 2,
            Self::Idr => 2,
            Self::Ils => 2,
            Self::Inr => 2,
            Self::Iqd => 3,
            Self::Irr => 2,
            Self::Isk => 0,
            Self::Jmd => 2,
            Self::Jod => 3,
            Self::Jpy => 0,
            Self::Kes => 2,
            Self::Kgs => 2,
            Self::Khr => 2,
            Self::Kmf => 0,
            Self::Kpw => 2,
            Self::Krw => 0,
            Self::Kwd => 3,
            Self::Kyd => 2,
            Self::Kzt => 2,
            Self::Lak => 2,
            Self::Lbp => 2,
            Self::Lkr => 2,
            Self::Lrd => 2,
            Self::Lsl => 2,
            Self::Lyd => 3,
            Self::Mad => 2,
            Self::Mdl => 2,
            Self::Mga => 2,
            Self::Mkd => 2,
            Self::Mmk => 2,
            Self::Mnt => 2,
            Self::Mop => 2,
            Self::Mru => 2,
            Self::Mur => 2,
            Self::Mvr => 2,
            Self::Mwk => 2,
            Self::Mxn => 2,
            Self::Mxv => 2,
            Self::Myr => 2,
            Self::Mzn => 2,
            Self::Nad => 2,
            Self::Ngn => 2,
            Self::Nio => 2,
            Self::Nok => 2,
            Self::Npr => 2,
            Self::Nzd => 2,
            Self::Omr => 3,
            Self::Pab => 2,
            Self::Pen => 2,
            Self::Pgk => 2,
            Self::Php => 2,
            Self::Pkr => 2,
            Self::Pln => 2,
            Self::Pyg => 0,
            Self::Qar => 2,
            Self::Ron => 2,
            Self::Rsd => 2,
            Self::Rub => 2,
            Self::Rwf => 0,
            Self::Sar => 2,
            Self::Sbd => 2,
            Self::Scr => 2,
            Self::Sdg => 2,
            Self::Sek => 2,
            Self::Sgd => 2,
            Self::Shp => 2,
            Self::Sle => 2,
            Self::Sos => 2,
            Self::Srd => 2,
            Self::Ssp => 2,
            Self::Stn => 2,
            Self::Svc => 2,
            Self::Syp => 2,
            Self::Szl => 2,
            Self::Thb => 2,
            Self::Tjs => 2,
            Self::Tmt => 2,
            Self::Tnd => 3,
            Self::Top => 2,
            Self::Try => 2,
            Self::Ttd => 2,
            Self::Twd => 2,
            Self::Tzs => 2,
            Self::Uah => 2,
            Self::Ugx => 0,
            Self::Usd => 2,
            Self::Usn => 2,
            Self::Uyi => 0,
            Self::Uyu => 2,
            Self::Uyw => 4,
            Self::Uzs => 2,
            Self::Ved => 2,
            Self::Ves => 2,
            Self::Vnd => 0,
            Self::Vuv => 0,
            Self::Wst => 2,
            Self::Xaf => 0,
            Self::Xag => 0,
            Self::Xau => 0,
            Self::Xba => 0,
            Self::Xbb => 0,
            Self::Xbc => 0,
            Self::Xbd => 0,
            Self::Xcd => 2,
            Self::Xdr => 0,
            Self::Xof => 0,
            Self::Xpd => 0,
            Self::Xpf => 0,
            Self::Xpt => 0,
            Self::Xsu => 0,
            Self::Xts => 0,
            Self::Xua => 0,
            Self::Xxx => 0,
            Self::Yer => 2,
            Self::Zar => 2,
            Self::Zmw => 2,
            Self::Zwg => 2,
        }
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Unknown or non-canonical currency code string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCurrencyCodeError {
    pub input: String,
}

impl fmt::Display for ParseCurrencyCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown or non-canonical currency code: {}", self.input)
    }
}

impl std::error::Error for ParseCurrencyCodeError {}

impl FromStr for CurrencyCode {
    type Err = ParseCurrencyCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AED" => Ok(Self::Aed),
            "AFN" => Ok(Self::Afn),
            "ALL" => Ok(Self::All),
            "AMD" => Ok(Self::Amd),
            "ANG" => Ok(Self::Ang),
            "AOA" => Ok(Self::Aoa),
            "ARS" => Ok(Self::Ars),
            "AUD" => Ok(Self::Aud),
            "AWG" => Ok(Self::Awg),
            "AZN" => Ok(Self::Azn),
            "BAM" => Ok(Self::Bam),
            "BBD" => Ok(Self::Bbd),
            "BDT" => Ok(Self::Bdt),
            "BGN" => Ok(Self::Bgn),
            "BHD" => Ok(Self::Bhd),
            "BIF" => Ok(Self::Bif),
            "BMD" => Ok(Self::Bmd),
            "BND" => Ok(Self::Bnd),
            "BOB" => Ok(Self::Bob),
            "BOV" => Ok(Self::Bov),
            "BRL" => Ok(Self::Brl),
            "BSD" => Ok(Self::Bsd),
            "BTN" => Ok(Self::Btn),
            "BWP" => Ok(Self::Bwp),
            "BYN" => Ok(Self::Byn),
            "BZD" => Ok(Self::Bzd),
            "CAD" => Ok(Self::Cad),
            "CDF" => Ok(Self::Cdf),
            "CHE" => Ok(Self::Che),
            "CHF" => Ok(Self::Chf),
            "CHW" => Ok(Self::Chw),
            "CLF" => Ok(Self::Clf),
            "CLP" => Ok(Self::Clp),
            "CNY" => Ok(Self::Cny),
            "COP" => Ok(Self::Cop),
            "COU" => Ok(Self::Cou),
            "CRC" => Ok(Self::Crc),
            "CUC" => Ok(Self::Cuc),
            "CUP" => Ok(Self::Cup),
            "CVE" => Ok(Self::Cve),
            "CZK" => Ok(Self::Czk),
            "DJF" => Ok(Self::Djf),
            "DKK" => Ok(Self::Dkk),
            "DOP" => Ok(Self::Dop),
            "DZD" => Ok(Self::Dzd),
            "EGP" => Ok(Self::Egp),
            "ERN" => Ok(Self::Ern),
            "ETB" => Ok(Self::Etb),
            "EUR" => Ok(Self::Eur),
            "FJD" => Ok(Self::Fjd),
            "FKP" => Ok(Self::Fkp),
            "GBP" => Ok(Self::Gbp),
            "GEL" => Ok(Self::Gel),
            "GHS" => Ok(Self::Ghs),
            "GIP" => Ok(Self::Gip),
            "GMD" => Ok(Self::Gmd),
            "GNF" => Ok(Self::Gnf),
            "GTQ" => Ok(Self::Gtq),
            "GYD" => Ok(Self::Gyd),
            "HKD" => Ok(Self::Hkd),
            "HNL" => Ok(Self::Hnl),
            "HTG" => Ok(Self::Htg),
            "HUF" => Ok(Self::Huf),
            "IDR" => Ok(Self::Idr),
            "ILS" => Ok(Self::Ils),
            "INR" => Ok(Self::Inr),
            "IQD" => Ok(Self::Iqd),
            "IRR" => Ok(Self::Irr),
            "ISK" => Ok(Self::Isk),
            "JMD" => Ok(Self::Jmd),
            "JOD" => Ok(Self::Jod),
            "JPY" => Ok(Self::Jpy),
            "KES" => Ok(Self::Kes),
            "KGS" => Ok(Self::Kgs),
            "KHR" => Ok(Self::Khr),
            "KMF" => Ok(Self::Kmf),
            "KPW" => Ok(Self::Kpw),
            "KRW" => Ok(Self::Krw),
            "KWD" => Ok(Self::Kwd),
            "KYD" => Ok(Self::Kyd),
            "KZT" => Ok(Self::Kzt),
            "LAK" => Ok(Self::Lak),
            "LBP" => Ok(Self::Lbp),
            "LKR" => Ok(Self::Lkr),
            "LRD" => Ok(Self::Lrd),
            "LSL" => Ok(Self::Lsl),
            "LYD" => Ok(Self::Lyd),
            "MAD" => Ok(Self::Mad),
            "MDL" => Ok(Self::Mdl),
            "MGA" => Ok(Self::Mga),
            "MKD" => Ok(Self::Mkd),
            "MMK" => Ok(Self::Mmk),
            "MNT" => Ok(Self::Mnt),
            "MOP" => Ok(Self::Mop),
            "MRU" => Ok(Self::Mru),
            "MUR" => Ok(Self::Mur),
            "MVR" => Ok(Self::Mvr),
            "MWK" => Ok(Self::Mwk),
            "MXN" => Ok(Self::Mxn),
            "MXV" => Ok(Self::Mxv),
            "MYR" => Ok(Self::Myr),
            "MZN" => Ok(Self::Mzn),
            "NAD" => Ok(Self::Nad),
            "NGN" => Ok(Self::Ngn),
            "NIO" => Ok(Self::Nio),
            "NOK" => Ok(Self::Nok),
            "NPR" => Ok(Self::Npr),
            "NZD" => Ok(Self::Nzd),
            "OMR" => Ok(Self::Omr),
            "PAB" => Ok(Self::Pab),
            "PEN" => Ok(Self::Pen),
            "PGK" => Ok(Self::Pgk),
            "PHP" => Ok(Self::Php),
            "PKR" => Ok(Self::Pkr),
            "PLN" => Ok(Self::Pln),
            "PYG" => Ok(Self::Pyg),
            "QAR" => Ok(Self::Qar),
            "RON" => Ok(Self::Ron),
            "RSD" => Ok(Self::Rsd),
            "RUB" => Ok(Self::Rub),
            "RWF" => Ok(Self::Rwf),
            "SAR" => Ok(Self::Sar),
            "SBD" => Ok(Self::Sbd),
            "SCR" => Ok(Self::Scr),
            "SDG" => Ok(Self::Sdg),
            "SEK" => Ok(Self::Sek),
            "SGD" => Ok(Self::Sgd),
            "SHP" => Ok(Self::Shp),
            "SLE" => Ok(Self::Sle),
            "SOS" => Ok(Self::Sos),
            "SRD" => Ok(Self::Srd),
            "SSP" => Ok(Self::Ssp),
            "STN" => Ok(Self::Stn),
            "SVC" => Ok(Self::Svc),
            "SYP" => Ok(Self::Syp),
            "SZL" => Ok(Self::Szl),
            "THB" => Ok(Self::Thb),
            "TJS" => Ok(Self::Tjs),
            "TMT" => Ok(Self::Tmt),
            "TND" => Ok(Self::Tnd),
            "TOP" => Ok(Self::Top),
            "TRY" => Ok(Self::Try),
            "TTD" => Ok(Self::Ttd),
            "TWD" => Ok(Self::Twd),
            "TZS" => Ok(Self::Tzs),
            "UAH" => Ok(Self::Uah),
            "UGX" => Ok(Self::Ugx),
            "USD" => Ok(Self::Usd),
            "USN" => Ok(Self::Usn),
            "UYI" => Ok(Self::Uyi),
            "UYU" => Ok(Self::Uyu),
            "UYW" => Ok(Self::Uyw),
            "UZS" => Ok(Self::Uzs),
            "VED" => Ok(Self::Ved),
            "VES" => Ok(Self::Ves),
            "VND" => Ok(Self::Vnd),
            "VUV" => Ok(Self::Vuv),
            "WST" => Ok(Self::Wst),
            "XAF" => Ok(Self::Xaf),
            "XAG" => Ok(Self::Xag),
            "XAU" => Ok(Self::Xau),
            "XBA" => Ok(Self::Xba),
            "XBB" => Ok(Self::Xbb),
            "XBC" => Ok(Self::Xbc),
            "XBD" => Ok(Self::Xbd),
            "XCD" => Ok(Self::Xcd),
            "XDR" => Ok(Self::Xdr),
            "XOF" => Ok(Self::Xof),
            "XPD" => Ok(Self::Xpd),
            "XPF" => Ok(Self::Xpf),
            "XPT" => Ok(Self::Xpt),
            "XSU" => Ok(Self::Xsu),
            "XTS" => Ok(Self::Xts),
            "XUA" => Ok(Self::Xua),
            "XXX" => Ok(Self::Xxx),
            "YER" => Ok(Self::Yer),
            "ZAR" => Ok(Self::Zar),
            "ZMW" => Ok(Self::Zmw),
            "ZWG" => Ok(Self::Zwg),
            _ => Err(ParseCurrencyCodeError {
                input: s.to_string(),
            }),
        }
    }
}

impl TryFrom<&str> for CurrencyCode {
    type Error = ParseCurrencyCodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Serialize for CurrencyCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CurrencyCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_all_variants() {
        for code in CurrencyCode::all() {
            let s = code.as_str();
            assert_eq!(s.parse::<CurrencyCode>().unwrap(), *code);
            assert_eq!(s.len(), 3);
        }
    }

    #[test]
    fn exponents_samples() {
        assert_eq!(CurrencyCode::Usd.exponent(), 2);
        assert_eq!(CurrencyCode::Jpy.exponent(), 0);
        assert_eq!(CurrencyCode::Kwd.exponent(), 3);
    }

    #[test]
    fn rejects_unknown_and_lowercase() {
        assert!("ZZZ".parse::<CurrencyCode>().is_err());
        assert!("usd".parse::<CurrencyCode>().is_err());
    }

    #[test]
    fn serde_string() {
        let v = serde_json::to_value(CurrencyCode::Eur).unwrap();
        assert_eq!(v, serde_json::json!("EUR"));
        let back: CurrencyCode = serde_json::from_value(v).unwrap();
        assert_eq!(back, CurrencyCode::Eur);
    }
}
