//! dbx-bench - Comprehensive benchmark suite with regression detection

use clap::Parser;
use dbx_bench::{BenchmarkConfig, BenchmarkRunner, BenchmarkSuite};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "dbx-bench", version, about = "DBX Benchmark Suite with Regression Detection")]
struct Args {
    /// Benchmark configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Output directory for results
    #[arg(short, long, default_value = "bench-results")]
    output: PathBuf,

    /// Run specific benchmark suite
    #[arg(short, long)]
    suite: Option<String>,

    /// Number of iterations per benchmark
    #[arg(short, long, default_value = "100")]
    iterations: usize,

    /// Warmup iterations
    #[arg(long, default_value = "10")]
    warmup: usize,

    /// Enable regression detection
    #[arg(long, default_value = "true")]
    regression: bool,

    /// Regression threshold (percentage)
    #[arg(long, default_value = "10.0")]
    threshold: f64,

    /// Baseline results file for comparison
    #[arg(long)]
    baseline: Option<PathBuf>,

    /// Output format
    #[arg(short, long, default_value = "json")]
    format: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// List available benchmarks
    #[arg(long)]
    list: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = BenchmarkConfig {
        iterations: args.iterations,
        warmup: args.warmup,
        enable_regression: args.regression,
        regression_threshold: args.threshold,
        baseline_file: args.baseline,
        output_dir: args.output,
        output_format: args.format,
        verbose: args.verbose,
        target_suite: args.suite,
    };

    let mut runner = BenchmarkRunner::new(config);

    if args.list {
        runner.list_benchmarks().await?;
        return Ok(());
    }

    let results = runner.run_all().await?;

    if args.verbose {
        println!("Benchmarks completed: {} suites, {} benchmarks", results.suites.len(), results.total_benchmarks);
        for suite in &results.suites {
            println!("  {}: {} benchmarks", suite.name, suite.benchmarks.len());
            for bench in &suite.benchmarks {
                let status = if bench.regressed { "⚠️ REGRESSED" } else { "✅ OK" };
                println!("    {}: {:.2}ms {} ({})", bench.name, bench.mean_ms, status, bench.unit);
            }
        }
    }

    if results.has_regressions {
        eprintln!("⚠️ Performance regressions detected!");
        std::process::exit(1);
    }

    Ok(())
}
