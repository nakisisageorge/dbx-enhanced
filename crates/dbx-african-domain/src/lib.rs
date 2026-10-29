//! dbx-african-domain - Shared types, patterns, and utilities for African database domains
//!
//! This crate provides common types, enums, and utilities for African fintech,
//! banking, healthcare, and logistics database integrations.

use dbx_types::models::connection::ConnectionConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// African mobile money service providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MobileMoneyProvider {
    /// Safaricom M-Pesa (Kenya) - Daraja API
    MpesaDaraja,
    /// MTN MoMo (Ghana, Uganda, South Africa, etc.)
    MtnMomo,
    /// Airtel Money (multiple African countries)
    AirtelMoney,
    /// Tigo Pesa (Tanzania)
    TigoPesa,
    /// Orange Money (West Africa)
    OrangeMoney,
    /// Vodacom M-Pesa (Tanzania, DRC, Mozambique, Lesotho)
    VodacomMpesa,
    /// Moov Money (West Africa)
    MoovMoney,
    /// Wave (Senegal, Ivory Coast)
    Wave,
    /// Chipper Cash (cross-border)
    ChipperCash,
    /// Paga (Nigeria)
    Paga,
    /// Opay (Nigeria, Egypt)
    Opay,
    /// PalmPay (Nigeria, Ghana)
    PalmPay,
}

impl MobileMoneyProvider {
    pub fn country_codes(&self) -> &'static [&'static str] {
        match self {
            MobileMoneyProvider::MpesaDaraja => &["KE"],
            MobileMoneyProvider::MtnMomo => &["GH", "UG", "ZA", "RW", "ZM", "CI", "CM", "GN", "LR", "SL", "BF", "NE"],
            MobileMoneyProvider::AirtelMoney => &["KE", "UG", "TZ", "RW", "MW", "ZM", "NG", "GA", "TD", "CG", "ML", "NE", "MR"],
            MobileMoneyProvider::TigoPesa => &["TZ"],
            MobileMoneyProvider::OrangeMoney => &["CI", "SN", "ML", "BF", "GN", "CI", "CM", "CG", "CD", "TD", "CF", "GA", "GQ", "LR", "MR", "SL", "ST"],
            MobileMoneyProvider::VodacomMpesa => &["TZ", "CD", "MZ", "LS"],
            MobileMoneyProvider::MoovMoney => &["CI", "BF", "NE", "TG", "BJ"],
            MobileMoneyProvider::Wave => &["SN", "CI"],
            MobileMoneyProvider::ChipperCash => &["NG", "GH", "KE", "UG", "ZA", "TZ", "RW", "ZM", "BW", "MW", "LS", "SZ"],
            MobileMoneyProvider::Paga => &["NG"],
            MobileMoneyProvider::Opay => &["NG", "EG"],
            MobileMoneyProvider::PalmPay => &["NG", "GH"],
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            MobileMoneyProvider::MpesaDaraja => "M-Pesa (Daraja API)",
            MobileMoneyProvider::MtnMomo => "MTN MoMo",
            MobileMoneyProvider::AirtelMoney => "Airtel Money",
            MobileMoneyProvider::TigoPesa => "Tigo Pesa",
            MobileMoneyProvider::OrangeMoney => "Orange Money",
            MobileMoneyProvider::VodacomMpesa => "Vodacom M-Pesa",
            MobileMoneyProvider::MoovMoney => "Moov Money",
            MobileMoneyProvider::Wave => "Wave",
            MobileMoneyProvider::ChipperCash => "Chipper Cash",
            MobileMoneyProvider::Paga => "Paga",
            MobileMoneyProvider::Opay => "Opay",
            MobileMoneyProvider::PalmPay => "PalmPay",
        }
    }

    pub fn api_base_url(&self, environment: Environment) -> &'static str {
        match (self, environment) {
            (MobileMoneyProvider::MpesaDaraja, Environment::Sandbox) => "https://sandbox.safaricom.co.ke",
            (MobileMoneyProvider::MpesaDaraja, Environment::Production) => "https://api.safaricom.co.ke",
            (MobileMoneyProvider::MtnMomo, Environment::Sandbox) => "https://sandbox.momodeveloper.mtn.com",
            (MobileMoneyProvider::MtnMomo, Environment::Production) => "https://proxy.momoapi.mtn.com",
            _ => "https://api.example.com", // Placeholder for others
        }
    }
}

/// African interbank settlement systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AfricanBankingSystem {
    /// Nigeria Inter-Bank Settlement System
    Nibss,
    /// Kenya Pesalink
    Pesalink,
    /// Ghana Interbank Payment and Settlement Systems
    Ghipss,
    /// South Africa Real Time Clearing
    Rtc,
    /// Zimbabwe Zimswitch
    Zimswitch,
    /// Tanzania Instant Payment System
    Tiss,
    /// Uganda Instant Payment System
    Uip,
    /// Rwanda Integrated Payment Processing System
    Ipps,
    /// Zambia Electronic Clearing House
    Zech,
    /// Botswana Automated Clearing House
    Bach,
    /// Namibia Payment System
    Namswitch,
    /// Egypt Instant Payment Network
    IpN,
    /// Morocco Interbank Payment System
    Sips,
    /// Tunisia Payment System
    Stp,
    /// Algeria Interbank System
    Satim,
}

impl AfricanBankingSystem {
    pub fn country_code(&self) -> &'static str {
        match self {
            AfricanBankingSystem::Nibss => "NG",
            AfricanBankingSystem::Pesalink => "KE",
            AfricanBankingSystem::Ghipss => "GH",
            AfricanBankingSystem::Rtc => "ZA",
            AfricanBankingSystem::Zimswitch => "ZW",
            AfricanBankingSystem::Tiss => "TZ",
            AfricanBankingSystem::Uip => "UG",
            AfricanBankingSystem::Ipps => "RW",
            AfricanBankingSystem::Zech => "ZM",
            AfricanBankingSystem::Bach => "BW",
            AfricanBankingSystem::Namswitch => "NA",
            AfricanBankingSystem::IpN => "EG",
            AfricanBankingSystem::Sips => "MA",
            AfricanBankingSystem::Stp => "TN",
            AfricanBankingSystem::Satim => "DZ",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AfricanBankingSystem::Nibss => "NIBSS (Nigeria)",
            AfricanBankingSystem::Pesalink => "Pesalink (Kenya)",
            AfricanBankingSystem::Ghipss => "GHIPSS (Ghana)",
            AfricanBankingSystem::Rtc => "RTC (South Africa)",
            AfricanBankingSystem::Zimswitch => "Zimswitch (Zimbabwe)",
            AfricanBankingSystem::Tiss => "TISS (Tanzania)",
            AfricanBankingSystem::Uip => "UIP (Uganda)",
            AfricanBankingSystem::Ipps => "IPPS (Rwanda)",
            AfricanBankingSystem::Zech => "ZECH (Zambia)",
            AfricanBankingSystem::Bach => "BACH (Botswana)",
            AfricanBankingSystem::Namswitch => "NamSwitch (Namibia)",
            AfricanBankingSystem::IpN => "IPN (Egypt)",
            AfricanBankingSystem::Sips => "SIPS (Morocco)",
            AfricanBankingSystem::Stp => "STP (Tunisia)",
            AfricanBankingSystem::Satim => "SATIM (Algeria)",
        }
    }
}

/// African healthcare data systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AfricanHealthcareSystem {
    /// District Health Information Software 2
    Dhisd2,
    /// Open Medical Record System
    OpenMrs,
    /// HL7 FHIR Africa Implementation Guide
    FhirAfrica,
    /// OpenLMIS (Logistics Management Information System)
    OpenLmis,
    /// iHRIS (Human Resources Information System)
    Ihris,
    /// mHero (Mobile Health Worker Communication)
    Mhero,
    /// CommCare (Dimagi)
    CommCare,
    /// ODK (Open Data Kit)
    ODK,
}

impl AfricanHealthcareSystem {
    pub fn display_name(&self) -> &'static str {
        match self {
            AfricanHealthcareSystem::Dhisd2 => "DHIS2",
            AfricanHealthcareSystem::OpenMrs => "OpenMRS",
            AfricanHealthcareSystem::FhirAfrica => "HL7 FHIR Africa",
            AfricanHealthcareSystem::OpenLmis => "OpenLMIS",
            AfricanHealthcareSystem::Ihris => "iHRIS",
            AfricanHealthcareSystem::Mhero => "mHero",
            AfricanHealthcareSystem::CommCare => "CommCare",
            AfricanHealthcareSystem::ODK => "ODK",
        }
    }
}

/// Environment for API endpoints
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Sandbox,
    Production,
}

/// Mobile money transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MobileMoneyTransactionType {
    CustomerToBusinessPayBill,
    CustomerToBusinessBuyGoods,
    BusinessToCustomer,
    BusinessToBusiness,
    CustomerToCustomer,
    Reversal,
    Refund,
    BalanceInquiry,
    MiniStatement,
}

/// Mobile money transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MobileMoneyTransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
    Expired,
    Cancelled,
}

/// African currency codes (ISO 4217)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AfricanCurrency {
    Kes, // Kenyan Shilling
    Ugx, // Ugandan Shilling
    Tzs, // Tanzanian Shilling
    Rwf, // Rwandan Franc
    Bif, // Burundian Franc
    Etb, // Ethiopian Birr
    Sos, // Somali Shilling
    Dji, // Djiboutian Franc
    Eru, // Eritrean Nakfa
    Sdg, // Sudanese Pound
    Ssp, // South Sudanese Pound
    Kmf, // Comorian Franc
    Mga, // Malagasy Ariary
    Mur, // Mauritian Rupee
    Scr, // Seychellois Rupee
    Znl, // Zambian Kwacha (old)
    Zmw, // Zambian Kwacha
    Mwk, // Malawian Kwacha
    Bwp, // Botswana Pula
    Nad, // Namibian Dollar
    Lsl, // Lesotho Loti
    Szl, // Swazi Lilangeni
    Zar, // South African Rand
    Aoa, // Angolan Kwanza
    Cdf, // Congolese Franc
    Xaf, // Central African CFA Franc
    Xof, // West African CFA Franc
    Cve, // Cape Verdean Escudo
    Gnf, // Guinean Franc
    Lrd, // Liberian Dollar
    Slr, // Sierra Leonean Leone (old)
    Sle, // Sierra Leonean Leone
    Gmd, // Gambian Dalasi
    Mrn, // Mauritanian Ouguiya (old)
    Mru, // Mauritanian Ouguiya
    Std, // São Tomé and Príncipe Dobra (old)
    Stn, // São Tomé and Príncipe Dobra
    Eoi, // Egyptian Pound
    Lyb, // Libyan Dinar (old)
    Lyd, // Libyan Dinar
    Tnd, // Tunisian Dinar
    Dzd, // Algerian Dinar
    Mad, // Moroccan Dirham
}

impl AfricanCurrency {
    pub fn from_country_code(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            "KE" => Some(AfricanCurrency::Kes),
            "UG" => Some(AfricanCurrency::Ugx),
            "TZ" => Some(AfricanCurrency::Tzs),
            "RW" => Some(AfricanCurrency::Rwf),
            "BI" => Some(AfricanCurrency::Bif),
            "ET" => Some(AfricanCurrency::Etb),
            "SO" => Some(AfricanCurrency::Sos),
            "DJ" => Some(AfricanCurrency::Dji),
            "ER" => Some(AfricanCurrency::Eru),
            "SD" => Some(AfricanCurrency::Sdg),
            "SS" => Some(AfricanCurrency::Ssp),
            "KM" => Some(AfricanCurrency::Kmf),
            "MG" => Some(AfricanCurrency::Mga),
            "MU" => Some(AfricanCurrency::Mur),
            "SC" => Some(AfricanCurrency::Scr),
            "ZM" => Some(AfricanCurrency::Zmw),
            "MW" => Some(AfricanCurrency::Mwk),
            "BW" => Some(AfricanCurrency::Bwp),
            "NA" => Some(AfricanCurrency::Nad),
            "LS" => Some(AfricanCurrency::Lsl),
            "SZ" => Some(AfricanCurrency::Szl),
            "ZA" => Some(AfricanCurrency::Zar),
            "AO" => Some(AfricanCurrency::Aoa),
            "CD" => Some(AfricanCurrency::Cdf),
            "CF" | "TD" | "CG" | "CM" | "GA" | "GQ" => Some(AfricanCurrency::Xaf),
            "BJ" | "BF" | "CI" | "GW" | "ML" | "NE" | "SN" | "TG" => Some(AfricanCurrency::Xof),
            "CV" => Some(AfricanCurrency::Cve),
            "GN" => Some(AfricanCurrency::Gnf),
            "LR" => Some(AfricanCurrency::Lrd),
            "SL" => Some(AfricanCurrency::Sle),
            "GM" => Some(AfricanCurrency::Gmd),
            "MR" => Some(AfricanCurrency::Mru),
            "ST" => Some(AfricanCurrency::Stn),
            "EG" => Some(AfricanCurrency::Eoi),
            "LY" => Some(AfricanCurrency::Lyd),
            "TN" => Some(AfricanCurrency::Tnd),
            "DZ" => Some(AfricanCurrency::Dzd),
            "MA" => Some(AfricanCurrency::Mad),
            _ => None,
        }
    }
}

/// Connection configuration for African mobile money APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileMoneyConnectionConfig {
    pub provider: MobileMoneyProvider,
    pub environment: Environment,
    pub consumer_key: String,
    pub consumer_secret: String,
    pub business_short_code: Option<String>,
    pub passkey: Option<String>,
    pub callback_url: Option<String>,
    pub timeout_seconds: Option<u64>,
    pub custom_headers: Option<HashMap<String, String>>,
}

/// Connection configuration for African banking systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AfricanBankingConnectionConfig {
    pub system: AfricanBankingSystem,
    pub environment: Environment,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub participant_code: Option<String>,
    pub certificate_path: Option<String>,
    pub certificate_password: Option<String>,
    pub timeout_seconds: Option<u64>,
    pub custom_headers: Option<HashMap<String, String>>,
}

/// Connection configuration for DHIS2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dhisd2ConnectionConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    pub api_version: Option<String>,
    pub timeout_seconds: Option<u64>,
}

/// Connection configuration for OpenMRS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsConnectionConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    pub database_name: Option<String>,
    pub timeout_seconds: Option<u64>,
}

/// African database identity extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AfricanDatabaseIdentity {
    pub base: ConnectionConfig,
    pub mobile_money_provider: Option<MobileMoneyProvider>,
    pub banking_system: Option<AfricanBankingSystem>,
    pub healthcare_system: Option<AfricanHealthcareSystem>,
    pub currency: Option<AfricanCurrency>,
    pub country_code: Option<String>,
    pub region: Option<String>,
}

/// Error types for African domain operations
#[derive(Debug, Error)]
pub enum AfricanDomainError {
    #[error("Invalid provider configuration: {0}")]
    InvalidConfig(String),
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    #[error("API request failed: {0}")]
    ApiRequestFailed(String),
    #[error("Unsupported provider: {0}")]
    UnsupportedProvider(String),
    #[error("Rate limited: retry after {0} seconds")]
    RateLimited(u64),
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Result type for African domain operations
pub type AfricanDomainResult<T> = Result<T, AfricanDomainError>;

/// Trait for African database drivers
#[async_trait::async_trait]
pub trait AfricanDatabaseDriver: Send + Sync {
    async fn connect(&self, config: &AfricanDatabaseIdentity) -> AfricanDomainResult<()>;
    async fn disconnect(&self) -> AfricanDomainResult<()>;
    async fn test_connection(&self) -> AfricanDomainResult<bool>;
    fn supported_providers(&self) -> Vec<MobileMoneyProvider>;
    fn supported_banking_systems(&self) -> Vec<AfricanBankingSystem>;
    fn supported_healthcare_systems(&self) -> Vec<AfricanHealthcareSystem>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_money_provider_country_codes() {
        assert_eq!(MobileMoneyProvider::MpesaDaraja.country_codes(), &["KE"]);
        assert!(MobileMoneyProvider::MtnMomo.country_codes().contains(&"GH"));
        assert!(MobileMoneyProvider::AirtelMoney.country_codes().contains(&"NG"));
    }

    #[test]
        fn test_african_currency_from_country() {
            assert_eq!(AfricanCurrency::from_country_code("KE"), Some(AfricanCurrency::Kes));
            assert_eq!(AfricanCurrency::from_country_code("NG"), None); // Nigeria uses NGN, not in our enum yet
            assert_eq!(AfricanCurrency::from_country_code("ZA"), Some(AfricanCurrency::Zar));
            assert_eq!(AfricanCurrency::from_country_code("EG"), Some(AfricanCurrency::Eoi));
        }

    #[test]
    fn test_african_banking_system_country() {
        assert_eq!(AfricanBankingSystem::Nibss.country_code(), "NG");
        assert_eq!(AfricanBankingSystem::Pesalink.country_code(), "KE");
        assert_eq!(AfricanBankingSystem::Rtc.country_code(), "ZA");
    }
}