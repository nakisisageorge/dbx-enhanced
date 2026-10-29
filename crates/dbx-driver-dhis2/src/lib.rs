//! dbx-driver-dhis2 - DHIS2 analytics database connector for African health data systems

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dbx_african_domain::{AfricanDomainError, AfricanDomainResult, Dhisd2ConnectionConfig};
use dbx_types::types::QueryResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DHIS2 data element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dhisd2DataElement {
    pub id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub value_type: String,
    pub aggregation_type: String,
    pub domain_type: String,
    pub zero_is_significant: bool,
    pub category_combo: Dhisd2CategoryCombo,
    pub created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dhisd2CategoryCombo {
    pub id: String,
    pub name: String,
}

/// DHIS2 organisation unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dhisd2OrganisationUnit {
    pub id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub level: u32,
    pub parent_id: Option<String>,
    pub opening_date: Option<DateTime<Utc>>,
    pub closed_date: Option<DateTime<Utc>>,
    pub coordinates: Option<String>,
    pub created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// DHIS2 analytics query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dhisd2AnalyticsQuery {
    pub dimension: String,
    pub filter: Option<String>,
    pub display_property: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub skip_meta: Option<bool>,
    pub skip_data: Option<bool>,
    pub hierarchy_meta: Option<bool>,
    pub show_hierarchy: Option<bool>,
    pub include_metadata_details: Option<bool>,
    pub include_num_den: Option<bool>,
    pub output_type: Option<String>,
    pub relative_period_date: Option<String>,
}

/// DHIS2 analytics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dhisd2AnalyticsResponse {
    pub headers: Vec<Dhisd2AnalyticsHeader>,
    pub meta_data: Dhisd2AnalyticsMetaData,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dhisd2AnalyticsHeader {
    pub name: String,
    pub column: String,
    pub meta: bool,
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dhisd2AnalyticsMetaData {
    pub names: HashMap<String, String>,
    pub items: HashMap<String, Dhisd2MetaDataItem>,
    pub dimensions: HashMap<String, Vec<String>>,
    pub ou_hierarchy: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dhisd2MetaDataItem {
    pub name: String,
    pub short_name: Option<String>,
    pub code: Option<String>,
}

/// DHIS2 driver
pub struct Dhisd2Driver {
    config: Option<Dhisd2ConnectionConfig>,
    client: Client,
}

impl Dhisd2Driver {
    pub fn new() -> Self {
        Self { config: None, client: Client::new() }
    }

    fn build_url(&self, path: &str) -> String {
        let config = self.config.as_ref().expect("DHIS2 driver not configured");
        let base = config.base_url.trim_end_matches('/');
        let api_version = config.api_version.as_deref().unwrap_or("api");
        format!("{}/{}/{}", base, api_version, path.trim_start_matches('/'))
    }
}

/// Common trait for DHIS2 drivers
#[async_trait]
pub trait Dhisd2DriverTrait: Send + Sync {
    async fn connect(&mut self, config: &Dhisd2ConnectionConfig) -> AfricanDomainResult<()>;
    async fn disconnect(&mut self) -> AfricanDomainResult<()>;
    async fn test_connection(&self) -> AfricanDomainResult<bool>;
}

#[async_trait]
impl Dhisd2DriverTrait for Dhisd2Driver {
    async fn connect(&mut self, config: &Dhisd2ConnectionConfig) -> AfricanDomainResult<()> {
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

impl Dhisd2Driver {
    async fn get_system_info(&self) -> AfricanDomainResult<serde_json::Value> {
        let url = self.build_url("system/info");
        let config = self.config.as_ref().ok_or_else(|| AfricanDomainError::InvalidConfig("Not configured".into()))?;
        let response = self.client.get(&url).basic_auth(&config.username, Some(&config.password)).send().await?;
        Ok(response.json().await?)
    }

    pub async fn get_data_elements(&self) -> AfricanDomainResult<Vec<Dhisd2DataElement>> {
        let url = self.build_url("dataElements");
        let config = self.config.as_ref().ok_or_else(|| AfricanDomainError::InvalidConfig("Not configured".into()))?;
        let response = self
            .client
            .get(&url)
            .basic_auth(&config.username, Some(&config.password))
            .query(&[("paging", "false"), ("fields", "id,name,shortName,code,description,valueType,aggregationType,domainType,zeroIsSignificant,categoryCombo[id,name],created,lastUpdated")])
            .send()
            .await?;
        let data: Dhisd2ListResponse<Dhisd2DataElement> = response.json().await?;
        Ok(data.data_elements)
    }

    pub async fn get_organisation_units(&self) -> AfricanDomainResult<Vec<Dhisd2OrganisationUnit>> {
        let url = self.build_url("organisationUnits");
        let config = self.config.as_ref().ok_or_else(|| AfricanDomainError::InvalidConfig("Not configured".into()))?;
        let response = self
            .client
            .get(&url)
            .basic_auth(&config.username, Some(&config.password))
            .query(&[("paging", "false"), ("fields", "id,name,shortName,code,description,level,parent[id],openingDate,closedDate,coordinates,created,lastUpdated")])
            .send()
            .await?;
        let data: Dhisd2ListResponse<Dhisd2OrganisationUnit> = response.json().await?;
        Ok(data.organisation_units)
    }

    pub async fn execute_analytics(
        &self,
        query: &Dhisd2AnalyticsQuery,
    ) -> AfricanDomainResult<Dhisd2AnalyticsResponse> {
        let url = self.build_url("analytics");
        let config = self.config.as_ref().ok_or_else(|| AfricanDomainError::InvalidConfig("Not configured".into()))?;
        let mut params = vec![("dimension", query.dimension.as_str())];
        if let Some(filter) = &query.filter {
            params.push(("filter", filter.as_str()));
        }
        if let Some(display_property) = &query.display_property {
            params.push(("displayProperty", display_property.as_str()));
        }
        if let Some(start_date) = &query.start_date {
            params.push(("startDate", start_date.as_str()));
        }
        if let Some(end_date) = &query.end_date {
            params.push(("endDate", end_date.as_str()));
        }
        if let Some(skip_meta) = query.skip_meta {
            params.push(("skipMeta", if skip_meta { "true" } else { "false" }));
        }
        if let Some(skip_data) = query.skip_data {
            params.push(("skipData", if skip_data { "true" } else { "false" }));
        }
        if let Some(hierarchy_meta) = query.hierarchy_meta {
            params.push(("hierarchyMeta", if hierarchy_meta { "true" } else { "false" }));
        }
        if let Some(show_hierarchy) = query.show_hierarchy {
            params.push(("showHierarchy", if show_hierarchy { "true" } else { "false" }));
        }
        if let Some(include_metadata_details) = query.include_metadata_details {
            params.push(("includeMetadataDetails", if include_metadata_details { "true" } else { "false" }));
        }
        if let Some(include_num_den) = query.include_num_den {
            params.push(("includeNumDen", if include_num_den { "true" } else { "false" }));
        }
        if let Some(output_type) = &query.output_type {
            params.push(("outputType", output_type.as_str()));
        }
        if let Some(relative_period_date) = &query.relative_period_date {
            params.push(("relativePeriodDate", relative_period_date.as_str()));
        }

        let response =
            self.client.get(&url).basic_auth(&config.username, Some(&config.password)).query(&params).send().await?;
        Ok(response.json().await?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Dhisd2ListResponse<T> {
    #[serde(rename = "dataElements")]
    data_elements: Vec<T>,
    #[serde(rename = "organisationUnits")]
    organisation_units: Vec<T>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dhis2_driver_creation() {
        let driver = Dhisd2Driver::new();
        // Just test it compiles and creates
    }
}
