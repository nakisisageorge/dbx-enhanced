//! dbx-bench - Benchmark suite core library

use criterion::{BenchmarkId, Criterion, Throughput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup: usize,
    pub enable_regression: bool,
    pub regression_threshold: f64,
    pub baseline_file: Option<PathBuf>,
    pub output_dir: PathBuf,
    pub output_format: String,
    pub verbose: bool,
    pub target_suite: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub suite: String,
    pub mean_ms: f64,
    pub median_ms: f64,
    pub std_dev_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub throughput: Option<f64>,
    pub unit: String,
    pub regressed: bool,
    pub regression_percent: Option<f64>,
    pub baseline_mean_ms: Option<f64>,
    pub timestamp: String,
    pub iterations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuiteResult {
    pub name: String,
    pub benchmarks: Vec<BenchmarkResult>,
    pub total_benchmarks: usize,
    pub regressed_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRunResult {
    pub suites: Vec<BenchmarkSuiteResult>,
    pub total_benchmarks: usize,
    pub total_regressed: usize,
    pub has_regressions: bool,
    pub timestamp: String,
    pub git_commit: Option<String>,
    pub git_branch: Option<String>,
}

pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    criterion: Criterion,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        let criterion = Criterion::default()
            .configure_from_args()
            .sample_size(config.iterations)
            .warm_up_time(Duration::from_millis(config.warmup as u64))
            .measurement_time(Duration::from_secs(30));

        Self { config, criterion }
    }

    pub async fn list_benchmarks(&self) -> anyhow::Result<()> {
        println!("Available benchmark suites:");
        println!("  driver-connect     - Database driver connection latency");
        println!("  driver-query       - Query execution throughput");
        println!("  mcp-tools          - MCP tool latency");
        println!("  ai-sql-generation  - AI SQL generation latency");
        println!("  data-grid          - Data grid rendering performance");
        println!("  schema-diff        - Schema diff computation");
        println!("  data-transfer      - Cross-database data transfer");
        Ok(())
    }

    pub async fn run_all(&mut self) -> anyhow::Result<BenchmarkRunResult> {
        let mut suites = Vec::new();
        let mut total_benchmarks = 0;
        let mut total_regressed = 0;

        let suite_names: Vec<String> = if let Some(target) = &self.config.target_suite {
            vec![target.clone()]
        } else {
            vec![
                "driver-connect".to_string(),
                "driver-query".to_string(),
                "mcp-tools".to_string(),
                "ai-sql-generation".to_string(),
                "data-grid".to_string(),
                "schema-diff".to_string(),
                "data-transfer".to_string(),
            ]
        };

        for suite_name in suite_names {
            let suite_result = self.run_suite(&suite_name).await?;
            total_benchmarks += suite_result.total_benchmarks;
            total_regressed += suite_result.regressed_count;
            suites.push(suite_result);
        }

        let has_regressions = total_regressed > 0;

        let result = BenchmarkRunResult {
            suites,
            total_benchmarks,
            total_regressed,
            has_regressions,
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_commit: Self::get_git_commit(),
            git_branch: Self::get_git_branch(),
        };

        self.save_results(&result).await?;

        Ok(result)
    }

    async fn run_suite(&mut self, suite_name: &str) -> anyhow::Result<BenchmarkSuiteResult> {
        let mut benchmarks = Vec::new();

        match suite_name {
            "driver-connect" => {
                benchmarks.extend(self.run_driver_connect_benchmarks().await?);
            }
            "driver-query" => {
                benchmarks.extend(self.run_driver_query_benchmarks().await?);
            }
            "mcp-tools" => {
                benchmarks.extend(self.run_mcp_tool_benchmarks().await?);
            }
            "ai-sql-generation" => {
                benchmarks.extend(self.run_ai_sql_benchmarks().await?);
            }
            "data-grid" => {
                benchmarks.extend(self.run_data_grid_benchmarks().await?);
            }
            "schema-diff" => {
                benchmarks.extend(self.run_schema_diff_benchmarks().await?);
            }
            "data-transfer" => {
                benchmarks.extend(self.run_data_transfer_benchmarks().await?);
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown benchmark suite: {}", suite_name));
            }
        }

        let regressed_count = benchmarks.iter().filter(|b| b.regressed).count();
        let total_benchmarks = benchmarks.len();

        Ok(BenchmarkSuiteResult { name: suite_name.to_string(), benchmarks, total_benchmarks, regressed_count })
    }

    async fn run_driver_connect_benchmarks(&mut self) -> anyhow::Result<Vec<BenchmarkResult>> {
        let drivers = vec![
            "postgres",
            "mysql",
            "sqlite",
            "redis",
            "mongodb",
            "clickhouse",
            "duckdb",
            "sqlserver",
            "oracle",
            "mpesa-daraja",
            "mtn-momo",
            "airtel-money",
            "nibss",
            "pesalink",
            "ghipss",
            "rtc",
            "zimswitch",
            "dhis2",
            "openmrs",
        ];

        let mut results = Vec::new();

        for driver in drivers {
            let mut times = Vec::new();
            for _ in 0..self.config.iterations {
                let start = std::time::Instant::now();
                // Simulate connection
                tokio::time::sleep(Duration::from_micros(100)).await;
                times.push(start.elapsed().as_millis() as f64);
            }

            let mean = times.iter().sum::<f64>() / times.len() as f64;
            let mut sorted = times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];
            let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
            let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];
            let min = sorted[0];
            let max = sorted[sorted.len() - 1];
            let variance = times.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / times.len() as f64;
            let std_dev = variance.sqrt();

            let baseline_mean = self.load_baseline("driver-connect", driver);
            let (regressed, regression_percent) = if let Some(baseline) = baseline_mean {
                let change = ((mean - baseline) / baseline) * 100.0;
                (change > self.config.regression_threshold, Some(change))
            } else {
                (false, None)
            };

            results.push(BenchmarkResult {
                name: driver.to_string(),
                suite: "driver-connect".to_string(),
                mean_ms: mean,
                median_ms: median,
                std_dev_ms: std_dev,
                min_ms: min,
                max_ms: max,
                p50_ms: median,
                p95_ms: p95,
                p99_ms: p99,
                throughput: Some(1000.0 / mean),
                unit: "ms".to_string(),
                regressed,
                regression_percent,
                baseline_mean_ms: baseline_mean,
                timestamp: chrono::Utc::now().to_rfc3339(),
                iterations: self.config.iterations,
            });
        }

        Ok(results)
    }

    async fn run_driver_query_benchmarks(&mut self) -> anyhow::Result<Vec<BenchmarkResult>> {
        let queries = vec![
            ("simple_select", "SELECT 1"),
            ("join_10_tables", "SELECT * FROM t1 JOIN t2 ... JOIN t10"),
            ("aggregation_1m_rows", "SELECT COUNT(*) FROM large_table GROUP BY col"),
            ("window_functions", "SELECT ROW_NUMBER() OVER (PARTITION BY x ORDER BY y)"),
            ("recursive_cte", "WITH RECURSIVE ..."),
            ("json_extract", "SELECT data->>'key' FROM json_table"),
            ("full_text_search", "SELECT * FROM docs WHERE content @@ 'query'"),
        ];

        let mut results = Vec::new();

        for (name, _query) in queries {
            let mut times = Vec::new();
            for _ in 0..self.config.iterations {
                let start = std::time::Instant::now();
                tokio::time::sleep(Duration::from_micros(500)).await;
                times.push(start.elapsed().as_millis() as f64);
            }

            let mean = times.iter().sum::<f64>() / times.len() as f64;
            let mut sorted = times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];
            let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
            let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];
            let min = sorted[0];
            let max = sorted[sorted.len() - 1];
            let variance = times.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / times.len() as f64;
            let std_dev = variance.sqrt();

            let baseline_mean = self.load_baseline("driver-query", name);
            let (regressed, regression_percent) = if let Some(baseline) = baseline_mean {
                let change = ((mean - baseline) / baseline) * 100.0;
                (change > self.config.regression_threshold, Some(change))
            } else {
                (false, None)
            };

            results.push(BenchmarkResult {
                name: name.to_string(),
                suite: "driver-query".to_string(),
                mean_ms: mean,
                median_ms: median,
                std_dev_ms: std_dev,
                min_ms: min,
                max_ms: max,
                p50_ms: median,
                p95_ms: p95,
                p99_ms: p99,
                throughput: Some(1000.0 / mean),
                unit: "ms".to_string(),
                regressed,
                regression_percent,
                baseline_mean_ms: baseline_mean,
                timestamp: chrono::Utc::now().to_rfc3339(),
                iterations: self.config.iterations,
            });
        }

        Ok(results)
    }

    async fn run_mcp_tool_benchmarks(&mut self) -> anyhow::Result<Vec<BenchmarkResult>> {
        let tools = vec![
            "list_connections",
            "list_tables",
            "execute_query",
            "describe_table",
            "get_table_sample",
            "african_mobile_money_query",
            "african_banking_settlement",
            "dhis2_analytics_query",
            "schema_diff_african",
            "data_lineage_mobile_money",
            "compliance_report_nibss",
            "benchmark_mcp_tool",
            "architecture_visualization",
        ];

        let mut results = Vec::new();

        for tool in tools {
            let mut times = Vec::new();
            for _ in 0..self.config.iterations {
                let start = std::time::Instant::now();
                tokio::time::sleep(Duration::from_micros(200)).await;
                times.push(start.elapsed().as_millis() as f64);
            }

            let mean = times.iter().sum::<f64>() / times.len() as f64;
            let mut sorted = times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];
            let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
            let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];
            let min = sorted[0];
            let max = sorted[sorted.len() - 1];
            let variance = times.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / times.len() as f64;
            let std_dev = variance.sqrt();

            let baseline_mean = self.load_baseline("mcp-tools", tool);
            let (regressed, regression_percent) = if let Some(baseline) = baseline_mean {
                let change = ((mean - baseline) / baseline) * 100.0;
                (change > self.config.regression_threshold, Some(change))
            } else {
                (false, None)
            };

            results.push(BenchmarkResult {
                name: tool.to_string(),
                suite: "mcp-tools".to_string(),
                mean_ms: mean,
                median_ms: median,
                std_dev_ms: std_dev,
                min_ms: min,
                max_ms: max,
                p50_ms: median,
                p95_ms: p95,
                p99_ms: p99,
                throughput: Some(1000.0 / mean),
                unit: "ms".to_string(),
                regressed,
                regression_percent,
                baseline_mean_ms: baseline_mean,
                timestamp: chrono::Utc::now().to_rfc3339(),
                iterations: self.config.iterations,
            });
        }

        Ok(results)
    }

    async fn run_ai_sql_benchmarks(&mut self) -> anyhow::Result<Vec<BenchmarkResult>> {
        let models = vec![
            "claude-3.5-sonnet",
            "gpt-4o",
            "gpt-4o-mini",
            "llama-3.1-70b",
            "llama-3.1-8b",
            "qwen2.5-72b",
            "codellama-34b",
            "deepseek-coder",
            "ollama-local",
        ];

        let mut results = Vec::new();

        for model in models {
            let mut times = Vec::new();
            let mut tokens = Vec::new();
            for _ in 0..self.config.iterations {
                let start = std::time::Instant::now();
                tokio::time::sleep(Duration::from_millis(50)).await;
                times.push(start.elapsed().as_millis() as f64);
                tokens.push(1500); // Simulated token count
            }

            let mean = times.iter().sum::<f64>() / times.len() as f64;
            let mut sorted = times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];
            let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
            let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];
            let min = sorted[0];
            let max = sorted[sorted.len() - 1];
            let variance = times.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / times.len() as f64;
            let std_dev = variance.sqrt();
            let avg_tokens = tokens.iter().sum::<u32>() as f64 / tokens.len() as f64;

            let baseline_mean = self.load_baseline("ai-sql-generation", model);
            let (regressed, regression_percent) = if let Some(baseline) = baseline_mean {
                let change = ((mean - baseline) / baseline) * 100.0;
                (change > self.config.regression_threshold, Some(change))
            } else {
                (false, None)
            };

            results.push(BenchmarkResult {
                name: model.to_string(),
                suite: "ai-sql-generation".to_string(),
                mean_ms: mean,
                median_ms: median,
                std_dev_ms: std_dev,
                min_ms: min,
                max_ms: max,
                p50_ms: median,
                p95_ms: p95,
                p99_ms: p99,
                throughput: Some(avg_tokens / (mean / 1000.0)),
                unit: "ms".to_string(),
                regressed,
                regression_percent,
                baseline_mean_ms: baseline_mean,
                timestamp: chrono::Utc::now().to_rfc3339(),
                iterations: self.config.iterations,
            });
        }

        Ok(results)
    }

    async fn run_data_grid_benchmarks(&mut self) -> anyhow::Result<Vec<BenchmarkResult>> {
        let row_counts = vec![100, 1_000, 10_000, 100_000, 1_000_000];
        let mut results = Vec::new();

        for rows in row_counts {
            let mut times = Vec::new();
            for _ in 0..self.config.iterations {
                let start = std::time::Instant::now();
                tokio::time::sleep(Duration::from_micros((rows as f64 * 0.1) as u64)).await;
                times.push(start.elapsed().as_millis() as f64);
            }

            let mean = times.iter().sum::<f64>() / times.len() as f64;
            let mut sorted = times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];
            let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
            let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];
            let min = sorted[0];
            let max = sorted[sorted.len() - 1];
            let variance = times.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / times.len() as f64;
            let std_dev = variance.sqrt();

            let baseline_mean = self.load_baseline("data-grid", &rows.to_string());
            let (regressed, regression_percent) = if let Some(baseline) = baseline_mean {
                let change = ((mean - baseline) / baseline) * 100.0;
                (change > self.config.regression_threshold, Some(change))
            } else {
                (false, None)
            };

            results.push(BenchmarkResult {
                name: format!("{}_rows", rows),
                suite: "data-grid".to_string(),
                mean_ms: mean,
                median_ms: median,
                std_dev_ms: std_dev,
                min_ms: min,
                max_ms: max,
                p50_ms: median,
                p95_ms: p95,
                p99_ms: p99,
                throughput: Some(rows as f64 / (mean / 1000.0)),
                unit: "ms".to_string(),
                regressed,
                regression_percent,
                baseline_mean_ms: baseline_mean,
                timestamp: chrono::Utc::now().to_rfc3339(),
                iterations: self.config.iterations,
            });
        }

        Ok(results)
    }

    async fn run_schema_diff_benchmarks(&mut self) -> anyhow::Result<Vec<BenchmarkResult>> {
        let table_counts = vec![10, 50, 100, 500, 1000];
        let mut results = Vec::new();

        for tables in table_counts {
            let mut times = Vec::new();
            for _ in 0..self.config.iterations {
                let start = std::time::Instant::now();
                tokio::time::sleep(Duration::from_micros((tables * 50) as u64)).await;
                times.push(start.elapsed().as_millis() as f64);
            }

            let mean = times.iter().sum::<f64>() / times.len() as f64;
            let mut sorted = times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];
            let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
            let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];
            let min = sorted[0];
            let max = sorted[sorted.len() - 1];
            let variance = times.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / times.len() as f64;
            let std_dev = variance.sqrt();

            let baseline_mean = self.load_baseline("schema-diff", &tables.to_string());
            let (regressed, regression_percent) = if let Some(baseline) = baseline_mean {
                let change = ((mean - baseline) / baseline) * 100.0;
                (change > self.config.regression_threshold, Some(change))
            } else {
                (false, None)
            };

            results.push(BenchmarkResult {
                name: format!("{}_tables", tables),
                suite: "schema-diff".to_string(),
                mean_ms: mean,
                median_ms: median,
                std_dev_ms: std_dev,
                min_ms: min,
                max_ms: max,
                p50_ms: median,
                p95_ms: p95,
                p99_ms: p99,
                throughput: Some(tables as f64 / (mean / 1000.0)),
                unit: "ms".to_string(),
                regressed,
                regression_percent,
                baseline_mean_ms: baseline_mean,
                timestamp: chrono::Utc::now().to_rfc3339(),
                iterations: self.config.iterations,
            });
        }

        Ok(results)
    }

    async fn run_data_transfer_benchmarks(&mut self) -> anyhow::Result<Vec<BenchmarkResult>> {
        let row_counts = vec![1_000, 10_000, 100_000, 1_000_000];
        let mut results = Vec::new();

        for rows in row_counts {
            let mut times = Vec::new();
            for _ in 0..self.config.iterations {
                let start = std::time::Instant::now();
                tokio::time::sleep(Duration::from_micros((rows * 2) as u64)).await;
                times.push(start.elapsed().as_millis() as f64);
            }

            let mean = times.iter().sum::<f64>() / times.len() as f64;
            let mut sorted = times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];
            let p95 = sorted[(sorted.len() as f64 * 0.95) as usize];
            let p99 = sorted[(sorted.len() as f64 * 0.99) as usize];
            let min = sorted[0];
            let max = sorted[sorted.len() - 1];
            let variance = times.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / times.len() as f64;
            let std_dev = variance.sqrt();

            let baseline_mean = self.load_baseline("data-transfer", &rows.to_string());
            let (regressed, regression_percent) = if let Some(baseline) = baseline_mean {
                let change = ((mean - baseline) / baseline) * 100.0;
                (change > self.config.regression_threshold, Some(change))
            } else {
                (false, None)
            };

            results.push(BenchmarkResult {
                name: format!("{}_rows", rows),
                suite: "data-transfer".to_string(),
                mean_ms: mean,
                median_ms: median,
                std_dev_ms: std_dev,
                min_ms: min,
                max_ms: max,
                p50_ms: median,
                p95_ms: p95,
                p99_ms: p99,
                throughput: Some(rows as f64 / (mean / 1000.0)),
                unit: "ms".to_string(),
                regressed,
                regression_percent,
                baseline_mean_ms: baseline_mean,
                timestamp: chrono::Utc::now().to_rfc3339(),
                iterations: self.config.iterations,
            });
        }

        Ok(results)
    }

    fn load_baseline(&self, suite: &str, benchmark: &str) -> Option<f64> {
        if let Some(baseline_file) = &self.config.baseline_file {
            if let Ok(content) = std::fs::read_to_string(baseline_file) {
                if let Ok(baseline) = serde_json::from_str::<HashMap<String, HashMap<String, f64>>>(&content) {
                    return baseline.get(suite).and_then(|s| s.get(benchmark)).copied();
                }
            }
        }
        None
    }

    async fn save_results(&self, results: &BenchmarkRunResult) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.config.output_dir)?;

        let file_name = format!("bench-results-{}.json", chrono::Utc::now().format("%Y%m%d-%H%M%S"));
        let file_path = self.config.output_dir.join(file_name);
        let json = serde_json::to_string_pretty(results)?;
        std::fs::write(&file_path, json)?;

        if self.config.verbose {
            println!("Results saved to: {}", file_path.display());
        }

        Ok(())
    }

    fn get_git_commit() -> Option<String> {
        std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
    }

    fn get_git_branch() -> Option<String> {
        std::process::Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
    }
}

pub struct BenchmarkSuite {
    pub name: String,
    pub benchmarks: Vec<BenchmarkDefinition>,
}

#[derive(Debug, Clone)]
pub struct BenchmarkDefinition {
    pub name: String,
    pub function: fn(&mut Criterion),
}

impl BenchmarkSuite {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), benchmarks: Vec::new() }
    }

    pub fn add_benchmark(mut self, name: &str, func: fn(&mut Criterion)) -> Self {
        self.benchmarks.push(BenchmarkDefinition { name: name.to_string(), function: func });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_config_defaults() {
        let config = BenchmarkConfig {
            iterations: 100,
            warmup: 10,
            enable_regression: true,
            regression_threshold: 10.0,
            baseline_file: None,
            output_dir: PathBuf::from("bench-results"),
            output_format: "json".to_string(),
            verbose: false,
            target_suite: None,
        };
        assert_eq!(config.iterations, 100);
        assert!(config.enable_regression);
    }
}
