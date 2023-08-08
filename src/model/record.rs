use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use sqlx::types::JsonValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
  pub release: Option<String>,
  pub execution_type: Option<String>,
  pub project: Option<String>,
  pub service: Option<String>,
  pub flow: Option<String>,
  pub tag: Option<String>,
  pub vu: Option<i16>,
  pub duration: Option<String>,
  pub tps: Option<BigDecimal>,
  pub error_rate: Option<BigDecimal>,
  pub rt_avg: Option<BigDecimal>,
  pub rt_min: Option<BigDecimal>,
  pub rt_max: Option<BigDecimal>,
  pub rt_p90: Option<BigDecimal>,
  pub rt_p95: Option<BigDecimal>,
  pub rt_p99: Option<BigDecimal>,
  pub is_cpu_below_request: Option<bool>,
  pub resource_map: Option<JsonValue>,
  pub replica_map: Option<JsonValue>,
  pub cpu_utilization: Option<Vec<BigDecimal>>,
  pub cpu_request: Option<Vec<BigDecimal>>,
  pub cpu_limit: Option<Vec<BigDecimal>>,
  pub memory_utilization: Option<Vec<BigDecimal>>,
  pub memory_request: Option<Vec<BigDecimal>>,
  pub memory_limit: Option<Vec<BigDecimal>>,
  pub timestamp: Option<String>,
}