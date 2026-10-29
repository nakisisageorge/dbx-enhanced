//! dbx-driver-mobile-money - Native drivers for African mobile money platforms

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dbx_african_domain::{
    AfricanCurrency, AfricanDomainError, AfricanDomainResult, MobileMoneyConnectionConfig,
    MobileMoneyProvider, MobileMoneyTransactionStatus, MobileMoneyTransactionType,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// M-Pesa Daraja API response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpesaStkPushRequest {
    pub business_short_code: String,
    pub password: String,
    pub timestamp: String,
    pub transaction_type: String,
    pub amount: String,
    pub party_a: String,
    pub party_b: String,
    pub phone_number: String,
    pub call_back_url: String,
    pub account_reference: String,
    pub transaction_desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpesaStkPushResponse {
    pub merchant_request_id: String,
    pub checkout_request_id: String,
    pub response_code: String,
    pub response_description: String,
    pub customer_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpesaCallback {
    pub body: MpesaCallbackBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpesaCallbackBody {
    pub stk_callback: MpesaStkCallback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpesaStkCallback {
    pub merchant_request_id: String,
    pub checkout_request_id: String,
    pub result_code: i32,
    pub result_desc: String,
    pub callback_metadata: Option<MpesaCallbackMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpesaCallbackMetadata {
    pub item: Vec<MpesaCallbackItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpesaCallbackItem {
    pub name: String,
    pub value: serde_json::Value,
}

/// Generic mobile money transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileMoneyTransaction {
    pub id: Uuid,
    pub provider: MobileMoneyProvider,
    pub transaction_type: MobileMoneyTransactionType,
    pub status: MobileMoneyTransactionStatus,
    pub amount: f64,
    pub currency: AfricanCurrency,
    pub sender_phone: Option<String>,
    pub receiver_phone: Option<String>,
    pub sender_name: Option<String>,
    pub receiver_name: Option<String>,
    pub reference: String,
    pub description: Option<String>,
    pub fees: Option<f64>,
    pub external_id: Option<String>,
    pub callback_metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Mobile money account balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileMoneyBalance {
    pub account_id: String,
    pub provider: MobileMoneyProvider,
    pub available_balance: f64,
    pub currency: AfricanCurrency,
    pub last_updated: DateTime<Utc>,
}

/// M-Pesa Daraja driver
pub struct MpesaDarajaDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
    access_token: Option<String>,
    token_expires_at: Option<DateTime<Utc>>,
}

impl MpesaDarajaDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new(), access_token: None, token_expires_at: None }
    }

    async fn get_access_token(&mut self) -> AfricanDomainResult<String> {
        let config = self.config.as_ref().ok_or_else(|| AfricanDomainError::InvalidConfig("Not configured".into()))?;

        if let (Some(token), Some(expires_at)) = (&self.access_token, &self.token_expires_at) {
            if Utc::now() < *expires_at {
                return Ok(token.clone());
            }
        }

        let provider = config.provider;
        let environment = config.environment;
        let url = format!("{}/oauth/v1/generate?grant_type=client_credentials", provider.api_base_url(environment));
        let response =
            self.client.get(&url).basic_auth(&config.consumer_key, Some(&config.consumer_secret)).send().await?;

        let token_response: serde_json::Value = response.json().await?;
        let token = token_response["access_token"]
            .as_str()
            .ok_or_else(|| AfricanDomainError::InvalidResponse("No access token in response".into()))?
            .to_string();
        let expires_in = token_response["expires_in"].as_str().and_then(|s| s.parse::<i64>().ok()).unwrap_or(3600);

        self.access_token = Some(token.clone());
        self.token_expires_at = Some(Utc::now() + chrono::Duration::seconds(expires_in - 60));

        Ok(token)
    }

    fn generate_password(&self, short_code: &str, passkey: &str, timestamp: &str) -> String {
        use base64::{engine::general_purpose, Engine as _};
        let data = format!("{}{}{}", short_code, passkey, timestamp);
        general_purpose::STANDARD.encode(data.as_bytes())
    }

    /// Initiate STK Push payment
    pub async fn stk_push(&mut self, request: MpesaStkPushRequest) -> AfricanDomainResult<MpesaStkPushResponse> {
        let token = self.get_access_token().await?;
        let provider = self.config.as_ref().unwrap().provider;
        let environment = self.config.as_ref().unwrap().environment;
        let url = format!("{}/mpesa/stkpush/v1/processrequest", provider.api_base_url(environment));

        let response = self.client.post(&url).bearer_auth(token).json(&request).send().await?;

        Ok(response.json().await?)
    }

    /// Query transaction status
    pub async fn query_transaction(&mut self, checkout_request_id: &str) -> AfricanDomainResult<serde_json::Value> {
        let token = self.get_access_token().await?;
        let provider = self.config.as_ref().unwrap().provider;
        let environment = self.config.as_ref().unwrap().environment;
        let url = format!("{}/mpesa/stkpushquery/v1/query", provider.api_base_url(environment));

        let request = serde_json::json!({
            "BusinessShortCode": self.config.as_ref().unwrap().business_short_code,
            "Password": self.generate_password(
                self.config.as_ref().unwrap().business_short_code.as_ref().unwrap(),
                self.config.as_ref().unwrap().passkey.as_ref().unwrap(),
                &chrono::Utc::now().format("%Y%m%d%H%M%S").to_string()
            ),
            "Timestamp": chrono::Utc::now().format("%Y%m%d%H%M%S").to_string(),
            "CheckoutRequestID": checkout_request_id
        });

        let response = self.client.post(&url).bearer_auth(token).json(&request).send().await?;

        Ok(response.json().await?)
    }
}

/// MTN MoMo driver
pub struct MtnMomoDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
    api_user: Option<String>,
    api_key: Option<String>,
    subscription_key: Option<String>,
}

impl MtnMomoDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new(), api_user: None, api_key: None, subscription_key: None }
    }

    /// Initialize with MTN MoMo credentials
    pub async fn initialize(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    /// Request to pay (MTN MoMo equivalent of STK Push)
    pub async fn request_to_pay(
        &self,
        _amount: f64,
        _phone: &str,
        _reference: &str,
    ) -> AfricanDomainResult<serde_json::Value> {
        Ok(serde_json::json!({"status": "pending", "message": "MTN MoMo request_to_pay not fully implemented"}))
    }
}

/// Airtel Money driver
pub struct AirtelMoneyDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl AirtelMoneyDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// Tigo Pesa driver
pub struct TigoPesaDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl TigoPesaDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// Orange Money driver
pub struct OrangeMoneyDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl OrangeMoneyDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// Vodacom M-Pesa driver
pub struct VodacomMpesaDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl VodacomMpesaDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// Moov Money driver
pub struct MoovMoneyDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl MoovMoneyDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// Wave driver
pub struct WaveDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl WaveDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// Chipper Cash driver
pub struct ChipperCashDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl ChipperCashDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// Paga driver
pub struct PagaDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl PagaDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// Opay driver
pub struct OpayDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl OpayDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// PalmPay driver
pub struct PalmPayDriver {
    config: Option<MobileMoneyConnectionConfig>,
    client: Client,
}

impl PalmPayDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }
}

/// Factory for creating mobile money drivers
pub fn create_mobile_money_driver(provider: MobileMoneyProvider) -> Box<dyn MobileMoneyDriver> {
    match provider {
        MobileMoneyProvider::MpesaDaraja => Box::new(MpesaDarajaDriver::new()),
        MobileMoneyProvider::MtnMomo => Box::new(MtnMomoDriver::new()),
        MobileMoneyProvider::AirtelMoney => Box::new(AirtelMoneyDriver::new()),
        MobileMoneyProvider::TigoPesa => Box::new(TigoPesaDriver::new()),
        MobileMoneyProvider::OrangeMoney => Box::new(OrangeMoneyDriver::new()),
        MobileMoneyProvider::VodacomMpesa => Box::new(VodacomMpesaDriver::new()),
        MobileMoneyProvider::MoovMoney => Box::new(MoovMoneyDriver::new()),
        MobileMoneyProvider::Wave => Box::new(WaveDriver::new()),
        MobileMoneyProvider::ChipperCash => Box::new(ChipperCashDriver::new()),
        MobileMoneyProvider::Paga => Box::new(PagaDriver::new()),
        MobileMoneyProvider::Opay => Box::new(OpayDriver::new()),
        MobileMoneyProvider::PalmPay => Box::new(PalmPayDriver::new()),
    }
}

/// Common trait for mobile money drivers
#[async_trait]
pub trait MobileMoneyDriver: Send + Sync {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()>;
    async fn disconnect(&mut self) -> AfricanDomainResult<()>;
    async fn test_connection(&self) -> AfricanDomainResult<bool>;
    fn provider(&self) -> MobileMoneyProvider;
}

#[async_trait]
impl MobileMoneyDriver for MpesaDarajaDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        self.get_access_token().await?;
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        self.access_token = None;
        self.token_expires_at = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        if self.config.is_none() {
            return Ok(false);
        }
        Ok(true)
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::MpesaDaraja
    }
}

#[async_trait]
impl MobileMoneyDriver for MtnMomoDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.initialize(config).await
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::MtnMomo
    }
}

#[async_trait]
impl MobileMoneyDriver for AirtelMoneyDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::AirtelMoney
    }
}

#[async_trait]
impl MobileMoneyDriver for TigoPesaDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::TigoPesa
    }
}

#[async_trait]
impl MobileMoneyDriver for OrangeMoneyDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::OrangeMoney
    }
}

#[async_trait]
impl MobileMoneyDriver for VodacomMpesaDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::VodacomMpesa
    }
}

#[async_trait]
impl MobileMoneyDriver for MoovMoneyDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::MoovMoney
    }
}

#[async_trait]
impl MobileMoneyDriver for WaveDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::Wave
    }
}

#[async_trait]
impl MobileMoneyDriver for ChipperCashDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::ChipperCash
    }
}

#[async_trait]
impl MobileMoneyDriver for PagaDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::Paga
    }
}

#[async_trait]
impl MobileMoneyDriver for OpayDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::Opay
    }
}

#[async_trait]
impl MobileMoneyDriver for PalmPayDriver {
    async fn connect(&mut self, config: &MobileMoneyConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        Ok(self.config.is_some())
    }

    fn provider(&self) -> MobileMoneyProvider {
        MobileMoneyProvider::PalmPay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpesa_password_generation() {
        let driver = MpesaDarajaDriver::new();
        let password = driver.generate_password(
            "174379",
            "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919",
            "20240101120000",
        );
        assert!(!password.is_empty());
    }
}
