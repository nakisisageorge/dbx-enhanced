//! dbx-driver-openmrs - OpenMRS medical records database connector for African healthcare systems

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dbx_african_domain::{AfricanDomainError, AfricanDomainResult, OpenMrsConnectionConfig};
use dbx_types::types::QueryResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OpenMRS patient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsPatient {
    pub uuid: String,
    pub display: String,
    pub identifiers: Vec<OpenMrsPatientIdentifier>,
    pub person: OpenMrsPerson,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsPatientIdentifier {
    pub uuid: String,
    pub identifier: String,
    pub identifier_type: OpenMrsIdentifierType,
    pub location: Option<OpenMrsLocation>,
    pub preferred: bool,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsIdentifierType {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub format_description: Option<String>,
    pub required: bool,
    pub check_digit: bool,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsPerson {
    pub uuid: String,
    pub names: Vec<OpenMrsPersonName>,
    pub addresses: Vec<OpenMrsPersonAddress>,
    pub attributes: Vec<OpenMrsPersonAttribute>,
    pub birthdate: Option<String>,
    pub birthdate_estimated: bool,
    pub gender: Option<String>,
    pub dead: bool,
    pub death_date: Option<String>,
    pub cause_of_death: Option<OpenMrsConcept>,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsPersonName {
    pub uuid: String,
    pub given_name: Option<String>,
    pub middle_name: Option<String>,
    pub family_name: Option<String>,
    pub family_name2: Option<String>,
    pub family_name_suffix: Option<String>,
    pub degree: Option<String>,
    pub preferred: bool,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsPersonAddress {
    pub uuid: String,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city_village: Option<String>,
    pub state_province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub preferred: bool,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsPersonAttribute {
    pub uuid: String,
    pub attribute_type: OpenMrsPersonAttributeType,
    pub value: String,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsPersonAttributeType {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub searchable: bool,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsLocation {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub address: Option<String>,
    pub county_district: Option<String>,
    pub state_province: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub parent_location: Option<Box<OpenMrsLocation>>,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsConcept {
    pub uuid: String,
    pub display: String,
    pub names: Vec<OpenMrsConceptName>,
    pub datatype: OpenMrsConceptDatatype,
    pub concept_class: OpenMrsConceptClass,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsConceptName {
    pub uuid: String,
    pub name: String,
    pub locale: String,
    pub locale_preferred: bool,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsConceptDatatype {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub hl7_abbreviation: Option<String>,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsConceptClass {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub voided: bool,
}

/// OpenMRS encounter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsEncounter {
    pub uuid: String,
    pub display: String,
    pub patient: OpenMrsPatient,
    pub encounter_type: OpenMrsEncounterType,
    pub encounter_datetime: String,
    pub location: OpenMrsLocation,
    pub form: Option<OpenMrsForm>,
    pub provider: Option<OpenMrsProvider>,
    pub observations: Vec<OpenMrsObs>,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsEncounterType {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsForm {
    pub uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsProvider {
    pub uuid: String,
    pub display: String,
    pub identifier: Option<String>,
    pub person: OpenMrsPerson,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsObs {
    pub uuid: String,
    pub display: String,
    pub concept: OpenMrsConcept,
    pub value: Option<String>,
    pub value_numeric: Option<f64>,
    pub value_text: Option<String>,
    pub value_datetime: Option<String>,
    pub value_complex: Option<OpenMrsComplexData>,
    pub value_coded: Option<OpenMrsConcept>,
    pub value_coded_name_id: Option<String>,
    pub value_drug: Option<OpenMrsDrug>,
    pub value_group_id: Option<String>,
    pub obs_datetime: String,
    pub location: Option<OpenMrsLocation>,
    pub voided: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsComplexData {
    pub uuid: String,
    pub title: String,
    pub size: u64,
    pub mime_type: String,
    pub extension: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenMrsDrug {
    pub uuid: String,
    pub display: String,
    pub name: String,
    pub dose: u32,
    pub units: String,
    pub frequency: String,
    pub duration: u32,
    pub duration_units: String,
    pub quantity: u32,
    pub route: OpenMrsConcept,
    pub voided: bool,
}

/// OpenMRS driver
pub struct OpenMrsDriver {
    config: Option<OpenMrsConnectionConfig>,
    client: Client,
}

impl OpenMrsDriver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }

    fn build_url(&self, path: &str) -> String {
        let config = self.config.as_ref().expect("OpenMRS driver not configured");
        let base = config.base_url.trim_end_matches('/');
        format!("{}/ws/rest/v1/{}", base, path.trim_start_matches('/'))
    }
}

/// Common trait for OpenMRS drivers
#[async_trait]
pub trait OpenMrsDriverTrait: Send + Sync {
    async fn connect(&mut self, config: &OpenMrsConnectionConfig) -> AfricanDomainResult<()>;
    async fn disconnect(&mut self) -> AfricanDomainResult<()>;
    async fn test_connection(&self) -> AfricanDomainResult<bool>;
}

#[async_trait]
impl OpenMrsDriverTrait for OpenMrsDriver {
    async fn connect(&mut self, config: &OpenMrsConnectionConfig) -> AfricanDomainResult<()> {
        self.config = Some(config.clone());
        let _ = self.get_system_info().await?;
        Ok(())
    }

    async fn disconnect(&mut self) -> AfricanDomainResult<()> {
        self.config = None;
        Ok(())
    }

    async fn test_connection(&self) -> AfricanDomainResult<bool> {
        if self.config.is_none() {
            return Ok(false);
        }
        self.get_system_info().await.map(|_| true).or(Ok(false))
    }
}

impl OpenMrsDriver {
    async fn get_system_info(&self) -> AfricanDomainResult<serde_json::Value> {
        let url = self.build_url("systeminfo");
        let config = self.config.as_ref().ok_or_else(|| AfricanDomainError::InvalidConfig("Not configured".into()))?;
        let response = self.client.get(&url).basic_auth(&config.username, Some(&config.password)).send().await?;
        Ok(response.json().await?)
    }

    pub async fn get_patients(&self, query: Option<&str>) -> AfricanDomainResult<Vec<OpenMrsPatient>> {
        let url = self.build_url("patient");
        let config = self.config.as_ref().ok_or_else(|| AfricanDomainError::InvalidConfig("Not configured".into()))?;
        let mut params = vec![("v", "full")];
        if let Some(q) = query {
            params.push(("q", q));
        }
        let response =
            self.client.get(&url).basic_auth(&config.username, Some(&config.password)).query(&params).send().await?;
        let data: OpenMrsListResponse<OpenMrsPatient> = response.json().await?;
        Ok(data.results)
    }

    pub async fn get_encounters(&self, patient_uuid: &str) -> AfricanDomainResult<Vec<OpenMrsEncounter>> {
        let url = self.build_url("encounter");
        let config = self.config.as_ref().ok_or_else(|| AfricanDomainError::InvalidConfig("Not configured".into()))?;
        let response = self
            .client
            .get(&url)
            .basic_auth(&config.username, Some(&config.password))
            .query(&[("patient", patient_uuid), ("v", "full")])
            .send()
            .await?;
        let data: OpenMrsListResponse<OpenMrsEncounter> = response.json().await?;
        Ok(data.results)
    }

    pub async fn get_observations(
        &self,
        patient_uuid: &str,
        concept_uuid: Option<&str>,
    ) -> AfricanDomainResult<Vec<OpenMrsObs>> {
        let url = self.build_url("obs");
        let config = self.config.as_ref().ok_or_else(|| AfricanDomainError::InvalidConfig("Not configured".into()))?;
        let mut params = vec![("patient", patient_uuid), ("v", "full")];
        if let Some(concept) = concept_uuid {
            params.push(("concept", concept));
        }
        let response =
            self.client.get(&url).basic_auth(&config.username, Some(&config.password)).query(&params).send().await?;
        let data: OpenMrsListResponse<OpenMrsObs> = response.json().await?;
        Ok(data.results)
    }

    pub async fn execute_sql(&self, _sql: &str) -> AfricanDomainResult<QueryResult> {
        Err(AfricanDomainError::UnsupportedProvider("Direct SQL execution not implemented for OpenMRS".into()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenMrsListResponse<T> {
    results: Vec<T>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openmrs_driver_creation() {
        let driver = OpenMrsDriver::new();
        // Just test it compiles and creates
    }
}
