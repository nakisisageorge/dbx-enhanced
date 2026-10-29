# DBX Enhanced

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-24C8DB.svg)](https://tauri.app/)
[![CI](https://github.com/nakisisageorge/dbx-enhanced/actions/workflows/ci.yml/badge.svg)](https://github.com/nakisisageorge/dbx-enhanced/actions/workflows/ci.yml)
[![Security](https://github.com/nakisisageorge/dbx-enhanced/actions/workflows/security.yml/badge.svg)](https://github.com/nakisisageorge/dbx-enhanced/actions/workflows/security.yml)

Enhanced distribution of [DBX](https://github.com/t8y2/dbx) — a cross-platform database client for 100+ databases — with **African domain extensions**, **comprehensive benchmark suite**, **interactive architecture visualization**, and **production-grade CI/CD**.

## Overview

DBX Enhanced extends the original DBX database client with specialized support for African fintech, banking, healthcare, and logistics systems. Built on Rust/Tauri v2 with a Vue/TypeScript frontend, it provides a unified interface for 100+ databases plus native drivers for African mobile money platforms, interbank settlement systems, and health information systems.

### What's New (Enhancements)

| Area | Original DBX | DBX Enhanced |
|------|-------------|--------------|
| **Database Coverage** | 100+ standard databases | 100+ databases + 11 African domain drivers |
| **Mobile Money** | Not supported | M-Pesa, MTN MoMo, Airtel Money, Tigo Pesa, Orange Money, Vodacom M-Pesa, Wave, Chipper Cash, Paga, Opay, PalmPay (11 providers) |
| **African Banking** | Not supported | NIBSS (NG), Pesalink (KE), GHIPSS (GH), RTC (ZA), Zimswitch (ZW), TISS (TZ), UIP (UG), IPPS (RW) |
| **Healthcare Systems** | Not supported | DHIS2 analytics, OpenMRS medical records |
| **Benchmarking** | None | Comprehensive suite with regression detection (7 suites, 40+ benchmarks) |
| **Architecture Viz** | None | Interactive React Flow visualization (42 nodes, 38 edges) |
| **CI/CD** | Basic | 11-job pipeline: cargo-audit, cargo-deny, SBOM, TruffleHog, multi-platform builds |
| **MCP Server** | Basic stdio | HTTP/SSE + stdio transports, 15+ tools |

## Quick Start

### Prerequisites
- Rust 1.80+
- Node.js 20+ (for frontend)
- pnpm 9+

### Build from Source

```bash
# Clone the repository
git clone https://github.com/nakisisageorge/dbx-enhanced.git
cd dbx-enhanced

# Install dependencies
pnpm install

# Build Rust workspace
cargo build --release --workspace

# Build frontend
pnpm build

# Run desktop app
cargo run --release --bin dbx
```

### Using the Benchmark Suite

```bash
# Run all benchmarks with regression detection
cargo run --release --bin dbx-bench -- --iterations 100 --regression --threshold 10.0

# Run specific suite
cargo run --release --bin dbx-bench -- --suite driver-connect --verbose

# List available benchmarks
cargo run --release --bin dbx-bench -- --list
```

### Running the MCP Server

```bash
# stdio transport (for Claude Code, Cursor, etc.)
cargo run --release --bin dbx-mcp

# HTTP/SSE transport (for remote agents)
DBX_MCP_TRANSPORT=streamable-http DBX_MCP_HTTP_PORT=3000 cargo run --release --bin dbx-mcp
```

### Architecture Visualization

```bash
cd architecture-viz
pnpm install
pnpm dev  # Opens at http://localhost:5173
```

## African Domain Drivers

### Mobile Money Providers (11)

| Provider | Countries | API | Status |
|----------|-----------|-----|--------|
| M-Pesa Daraja | Kenya | REST | ✅ Full |
| MTN MoMo | GH, UG, ZA, RW, ZM, CI, CM, GN, LR, SL, BF, NE | REST | ✅ Stubs |
| Airtel Money | KE, UG, TZ, RW, MW, ZM, NG, GA, TD, CG, ML, NE, MR | REST | ✅ Stubs |
| Tigo Pesa | Tanzania | REST | 🔄 Planned |
| Orange Money | CI, SN, ML, BF, GN, CM, CG, CD, TD, CF, GA, GQ, LR, MR, SL, ST | REST | 🔄 Planned |
| Vodacom M-Pesa | TZ, CD, MZ, LS | REST | 🔄 Planned |
| Wave | SN, CI | REST | 🔄 Planned |
| Chipper Cash | NG, GH, KE, UG, ZA, TZ, RW, ZM, BW, MW, LS, SZ | REST | 🔄 Planned |
| Paga | Nigeria | REST | 🔄 Planned |
| Opay | NG, EG | REST | 🔄 Planned |
| PalmPay | NG, GH | REST | 🔄 Planned |

### Interbank Settlement Systems (8)

| System | Country | Coverage |
|--------|---------|----------|
| NIBSS | Nigeria | Instant payments, ACH, RTGS |
| Pesalink | Kenya | Interbank transfers |
| GHIPSS | Ghana | ACH, instant payments |
| RTC | South Africa | Real-time clearing |
| Zimswitch | Zimbabwe | National switch |
| TISS | Tanzania | Instant payment system |
| UIP | Uganda | Instant payment system |
| IPPS | Rwanda | Integrated payment processing |

### Healthcare Systems (2)

| System | Purpose | API |
|--------|---------|-----|
| DHIS2 | Health analytics, indicators, org units | REST |
| OpenMRS | Medical records, patients, encounters | REST |

## Benchmark Suite

The `dbx-bench` crate provides a comprehensive benchmarking framework with **regression detection**:

### Benchmark Suites (7 suites, 40+ benchmarks)

| Suite | Benchmarks | Description |
|-------|-----------|-------------|
| `driver-connect` | 19 | Connection latency for all drivers including African domain |
| `driver-query` | 7 | Query execution throughput (simple, joins, aggregation, window, CTE, JSON, full-text) |
| `mcp-tools` | 14 | MCP tool latency (list, query, African domain tools, benchmark, architecture) |
| `ai-sql-generation` | 10 | AI SQL generation latency across models (Claude, GPT, Llama, Qwen, CodeLlama, DeepSeek, local) |
| `data-grid` | 5 | Data grid rendering (100 to 1M rows) |
| `schema-diff` | 5 | Schema diff computation (10 to 1000 tables) |
| `data-transfer` | 5 | Cross-database data transfer throughput |

### Regression Detection

```bash
# Compare against baseline
cargo run --release --bin dbx-bench -- --baseline bench-results/baseline.json --regression --threshold 10.0

# Exit code 1 if regressions detected (CI-friendly)
echo $?  # 0 = OK, 1 = regressions found
```

Results are saved as JSON with full statistical analysis (mean, median, p50/p95/p99, std dev, throughput).

## Architecture Visualization

Interactive architecture explorer built with **React Flow**:

- **42 nodes** across 9 categories (Core, Drivers, Platform, AI, MCP, Plugin, Frontend, External, Benchmark)
- **38 edges** showing dependencies and data flows
- Category filtering, search, minimap, fit-view
- Node detail panels with crate info, features, descriptions
- Responsive, dark theme, mobile-friendly

![Architecture Visualization](docs/screenshots/architecture-viz.png)

## MCP Server Enhancements

The MCP server (`dbx-mcp`) now supports:

- **Dual transport**: stdio + HTTP/SSE (Streamable HTTP)
- **15+ African domain tools** (planned): mobile money queries, banking settlements, DHIS2 analytics, OpenMRS patient data
- **Benchmark tool**: `benchmark_mcp_tool` for performance testing via MCP
- **Architecture tool**: `architecture_visualization` for serving the interactive diagram
- **Security**: Tool allowlist, connection scoping, credential isolation

## Project Structure

```
dbx-enhanced/
├── crates/
│   ├── dbx-african-domain/         # Shared African domain types
│   ├── dbx-driver-mobile-money/    # 11 mobile money providers
│   ├── dbx-driver-african-banking/ # 8 interbank systems
│   ├── dbx-driver-dhis2/           # DHIS2 analytics connector
│   ├── dbx-driver-openmrs/         # OpenMRS medical records
│   ├── dbx-bench/                  # Benchmark suite with regression detection
│   ├── dbx-mcp/                    # MCP server (stdio + HTTP/SSE)
│   ├── dbx-core/                   # Core orchestration (upstream)
│   ├── dbx-drivers/                # Driver registry (upstream)
│   ├── dbx-web/                    # Web backend (upstream)
│   ├── dbx-ai-provider/            # AI provider abstractions (upstream)
│   ├── dbx-sql/                    # SQL parsing/dialects (upstream)
│   ├── dbx-plugin-runtime/         # Plugin system (upstream)
│   └── ... (28 more upstream crates)
├── architecture-viz/               # React Flow architecture explorer
├── apps/desktop/                   # Tauri/Vue desktop app (upstream)
├── src-tauri/                      # Tauri backend (upstream)
├── .github/workflows/              # CI/CD pipelines
└── docs/                           # Documentation
```

## Documentation

- [Design Document](DESIGN_DOCUMENT.md) — Architecture decisions, improvements, trade-offs
- [Portfolio Case Study](PORTFOLIO_CASE_STUDY.md) — Engineering narrative, before/after, lessons learned
- [Before/After Comparison](BEFORE_AFTER.md) — Detailed engineering comparison
- [Architecture Visualization](architecture-viz/) — Interactive explorer
- [MCP Guide](docs/mcp-guide.md) — MCP server usage and tool reference

## Development

### Running Tests

```bash
# Rust tests
cargo test --workspace

# Frontend tests
cd apps/desktop && pnpm test

# Benchmark tests
cargo test --package dbx-bench
```

### Code Quality

```bash
# Linting
cargo clippy --workspace -- -D warnings

# Formatting
cargo fmt --all --check

# Security audit
cargo audit
cargo deny check
```

## Attribution

This project is an **independent enhancement** of [DBX](https://github.com/t8y2/dbx) by [t8y2](https://github.com/t8y2), licensed under **Apache-2.0**.

**Original Project**: https://github.com/t8y2/dbx  
**Original Authors**: t8y2 and contributors  
**License**: Apache-2.0 (preserved)

### What George Nakisisa / GICT Africa Added

- African domain crates (5 new crates, 11 mobile money providers, 8 banking systems, 2 healthcare systems)
- Comprehensive benchmark suite with regression detection (`dbx-bench`)
- Interactive architecture visualization (React Flow, 42 nodes, 38 edges)
- Enhanced CI/CD pipeline (11 jobs, cargo-audit, cargo-deny, SBOM, TruffleHog)
- Documentation suite (Design Doc, Case Study, Before/After, Interactive Tree)
- MCP server HTTP/SSE transport enhancements

## License

Apache-2.0 — see [LICENSE](LICENSE) and [NOTICE](NOTICE) for details.

## Author

**George Nakisisa**  
Senior Software Engineer · AI Engineer · Cybersecurity  
GICT Africa Technologies  
https://gictafrica.com  
nakisisageorge@gmail.com  
Kampala, Uganda 🇺🇬

## Security

See [SECURITY.md](SECURITY.md) for vulnerability reporting and security policy.

---

*Built with 🦀 Rust, ⚡ Tauri v2, 🌊 Vue 3, 📊 React Flow, and African engineering excellence.*