# Design Document — DBX Enhanced

**Project**: DBX Enhanced  
**Original**: [t8y2/dbx](https://github.com/t8y2/dbx)  
**Author**: George Nakisisa (GICT Africa Technologies)  
**Date**: 2026-10-27  
**Mode**: B — Substantial Enhancement Fork (Apache-2.0)

---

## 1. Original Architecture

### 1.1 Technology Stack
- **Core**: Rust 1.80+, Tauri v2
- **Frontend**: Vue 3 + TypeScript + Vite
- **Desktop**: Tauri (macOS, Windows, Linux)
- **Web**: Axum + WASM (same Rust core)
- **Package Manager**: pnpm (workspace)
- **Database Drivers**: 28 native drivers + JDBC + Agent runtime

### 1.2 Crate Organization (Upstream)
```
dbx-core/           # Application orchestration (connections, queries, schema, AI, admin)
dbx-drivers/        # Driver registry, native drivers, tunnels, JDBC
dbx-types/          # Connection config, identity, DTOs, JSON, MQTT/GridFS
dbx-sql/            # SQL parsing, dialects, risk analysis, DDL/DML, diff
dbx-mcp/            # MCP server (stdio transport)
dbx-ai-provider/    # Model adapters (Claude, OpenAI, Ollama, local)
dbx-web/            # Web backend (Axum)
dbx-cli/            # CLI package
dbx-formats/        # CSV/XLSX/text/time/ZIP
dbx-platform/       # Process mgmt, paths, proxy, downloads
dbx-plugin-runtime/ # Plugin manifest, marketplace, subprocess
... (28+ more crates)
```

### 1.3 Key Features (Upstream)
- 100+ database connections (PostgreSQL, MySQL, SQLite, Redis, MongoDB, ClickHouse, DuckDB, SQL Server, Oracle, Elasticsearch, etc.)
- Schema diff & synchronization with DDL deploy/rollback
- AI-assisted SQL generation and query explanation
- MCP server (stdio) with 10+ tools
- Plugin system with marketplace
- Cross-platform desktop (Tauri v2) + web deployment
- Production safety (risk classification, confirmations, audit)

### 1.4 Gaps Identified

| Gap | Impact |
|-----|--------|
| No African fintech/banking/healthcare domain support | Cannot natively query mobile money, interbank, or health systems |
| No benchmarking infrastructure | Cannot measure or regression-test performance |
| No architecture visualization | Hard to understand crate relationships |
| Basic CI/CD | Missing security scanning, SBOM, dependency auditing |
| MCP server stdio only | No HTTP/SSE for remote agent access |
| No regression detection | Performance degradations go unnoticed |

---

## 2. Problems Identified

### 2.1 Domain Coverage
African developers and enterprises use DBX but must write custom integrations for:
- **Mobile money**: M-Pesa (Daraja), MTN MoMo, Airtel Money, etc. (11 major providers)
- **Interbank systems**: NIBSS (NG), Pesalink (KE), GHIPSS (GH), RTC (ZA), Zimswitch (ZW), TISS (TZ), UIP (UG), IPPS (RW)
- **Healthcare**: DHIS2 (analytics), OpenMRS (medical records)

### 2.2 Observability & Quality
- No automated performance benchmarks
- No regression detection in CI
- No architecture documentation for onboarding
- Limited security scanning (no cargo-audit, cargo-deny, TruffleHog, SBOM)

### 2.3 MCP Limitations
- stdio transport only (local agents)
- No African domain tools exposed via MCP
- No benchmark/architecture tools via MCP

---

## 3. Proposed Architecture

### 3.1 New Crates (Enhancement Layer)

```
dbx-african-domain/         # Shared types: providers, currencies, configs, errors, traits
dbx-driver-mobile-money/    # 11 mobile money drivers (M-Pesa full, others stubbed)
dbx-driver-african-banking/ # 8 interbank settlement drivers (NIBSS full types, others stubbed)
dbx-driver-dhis2/           # DHIS2 analytics connector (full REST implementation)
dbx-driver-openmrs/         # OpenMRS medical records connector (full REST implementation)
dbx-bench/                  # Benchmark suite with regression detection (Criterion-based)
```

### 3.2 Integration Points

```
dbx-drivers (upstream)
       │
       ├── depends on dbx-african-domain
       │
       ├── implements dbx-driver-mobile-money
       ├── implements dbx-driver-african-banking
       ├── implements dbx-driver-dhis2
       └── implements dbx-driver-openmrs

dbx-bench
       ├── uses dbx-core, dbx-drivers, dbx-mcp, dbx-ai-provider
       └── outputs JSON + HTML reports (Criterion)

dbx-mcp (enhanced)
       ├── adds African domain tool handlers
       ├── adds benchmark_mcp_tool
       ├── adds architecture_visualization tool
       └── HTTP/SSE transport (already present via rmcp)

architecture-viz/ (new)
       ├── React Flow visualization
       ├── 42 nodes, 38 edges, 9 categories
       ├── search, filter, minimap, detail panel
       └── deployable as static site
```

### 3.3 Data Flow (African Domain Query)

```
MCP Client → dbx-mcp → dbx-core → dbx-drivers → dbx-driver-mobile-money
                                                      ↓
                                            MobileMoneyProvider enum
                                                      ↓
                                            create_mobile_money_driver()
                                                      ↓
                                            MpesaDarajaDriver → Safaricom Daraja API
```

---

## 4. New Architecture

### 4.1 Crate Dependency Graph (Enhanced)

```
                    ┌─────────────────────┐
                    │    dbx-core         │
                    └─────────┬───────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
       ┌────────────┐  ┌────────────┐  ┌────────────┐
       │ dbx-drivers│  │ dbx-mcp    │  │ dbx-bench  │
       └─────┬──────┘  └─────┬──────┘  └─────┬──────┘
             │               │               │
     ┌───────┼───────┐       │               │
     ▼       ▼       ▼       ▼               ▼
┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐
│mobile  │ │african │ │ dhis2  │ │ openmrs│ │ (uses  │
│money   │ │banking │ │        │ │        │ │ all    │
└────────┘ └────────┘ └────────┘ └────────┘ └────────┘
     │       │       │       │
     └───────┴───────┴───────┘
             │
             ▼
    ┌────────────────┐
    │dbx-african-    │
    │domain (shared) │
    └────────────────┘
```

### 4.2 African Domain Type System

```
dbx-african-domain/
├── MobileMoneyProvider (11 variants)     # Country codes, display names, API URLs
├── AfricanBankingSystem (12 variants)    # Country codes, display names
├── AfricanHealthcareSystem (9 variants)  # Display names
├── AfricanCurrency (55 variants)         # ISO 4217, from_country_code()
├── Environment (Sandbox/Production)
├── MobileMoneyTransactionType (8 variants)
├── MobileMoneyTransactionStatus (7 variants)
├── MobileMoneyConnectionConfig           # API keys, callbacks, timeouts
├── AfricanBankingConnectionConfig        # API keys, certificates, participant codes
├── Dhisd2ConnectionConfig                # Base URL, credentials, API version
├── OpenMrsConnectionConfig               # Base URL, credentials, database
├── AfricanDatabaseIdentity               # Extension of DatabaseIdentity
├── AfricanDomainError / AfricanDomainResult
└── AfricanDatabaseDriver (trait)
```

### 4.3 Benchmark Architecture

```
dbx-bench/
├── BenchmarkConfig          # iterations, warmup, regression threshold, baseline file
├── BenchmarkResult          # mean/median/p50/p95/p99/stddev, throughput, regression%
├── BenchmarkSuiteResult     # suite name, benchmarks[], regressed count
├── BenchmarkRunResult       # all suites, total benchmarks, total regressed, git metadata
├── BenchmarkRunner          # Criterion wrapper, runs all suites
│   ├── driver-connect (19 drivers)
│   ├── driver-query (7 query patterns)
│   ├── mcp-tools (14 tools)
│   ├── ai-sql-generation (10 models)
│   ├── data-grid (5 row counts)
│   ├── schema-diff (5 table counts)
│   └── data-transfer (5 scenarios)
└── CLI (clap)               # --suite, --iterations, --regression, --baseline, --output
```

### 4.4 Architecture Visualization

```
architecture-viz/
├── src/
│   ├── ArchitectureVisualization.tsx   # Main React Flow component
│   ├── data/architecture.ts            # 42 nodes, 38 edges, categories
│   ├── components/                     # CoreNode, DriverNode, PlatformNode,
│   │                                   # AiNode, McpNode, PluginNode,
│   │                                   # FrontendNode, ExternalNode
│   └── main.tsx                        # Entry point
├── package.json, vite.config.ts, tsconfig.json
└── Deploy: pnpm build → dist/ → GitHub Pages / Vercel
```

---

## 5. Why These Changes

### 5.1 African Domain Crates
- **Business need**: African fintech/healthcare/logistics developers need first-class DBX support
- **Technical fit**: Driver architecture already extensible; clean separation via `dbx-african-domain`
- **Portfolio value**: Demonstrates domain expertise, systems integration, Rust crate design
- **License compliance**: Apache-2.0 permits fork, enhancement, redistribution

### 5.2 Benchmark Suite
- **Engineering rigor**: Performance must be measurable, not assumed
- **Regression detection**: CI gate prevents silent degradation
- **Portfolio value**: Shows Criterion expertise, statistical analysis, CLI design
- **Reusability**: Framework extensible to any Rust project

### 5.3 Architecture Visualization
- **Documentation as code**: Living diagram, not stale PNG
- **Interactive exploration**: Filter, search, drill-down for stakeholders
- **Portfolio value**: React Flow, TypeScript, data-driven visualization
- **Maintainability**: Single source of truth (`architecture.ts`)

### 5.4 CI/CD Enhancements
- **Security posture**: cargo-audit (vulns), cargo-deny (licenses/bans), TruffleHog (secrets), SBOM (supply chain)
- **Multi-platform**: Linux/macOS/Windows builds
- **Quality gates**: clippy, fmt, test, audit all required to pass
- **Portfolio value**: Production-grade pipeline design

---

## 6. Technology Decisions

| Decision | Rationale |
|----------|-----------|
| Separate crates per domain | Clear boundaries, independent versioning, optional dependencies |
| `dbx-african-domain` as shared types | Single source of truth for enums, configs, errors, traits |
| Criterion for benchmarking | Industry standard, HTML reports, statistical rigor, async support |
| React Flow for architecture | Interactive, extensible, TypeScript-native, MIT license |
| HTTP/SSE MCP transport | rmcp 0.5+ supports it; enables remote agents |
| Regression threshold 10% | Balances noise vs. signal; configurable via CLI |
| Apache-2.0 throughout | Matches upstream; permissive for adoption |

---

## 7. Security Decisions

| Area | Decision |
|------|----------|
| Credential storage | Environment variables only; `.env.example` with placeholders |
| MCP tool allowlist | Per-connection scope; plugin tools filtered by policy |
| API communication | TLS required; `reqwest` with `rustls-tls` |
| Input validation | `schemars` schemas on all MCP tool arguments |
| Dependency scanning | `cargo-audit` + `cargo-deny` in CI (fail on high/critical) |
| Secret detection | TruffleHog in CI (fail on any secret) |
| SBOM generation | `cargo-cyclonedx` in CI (CycloneDX JSON) |

---

## 8. Performance Decisions

| Area | Decision |
|------|----------|
| Benchmark iterations | Default 100 (configurable); warmup 10 |
| Regression threshold | Default 10% (configurable); statistical significance via Criterion |
| Connection pooling | Reuse `reqwest::Client` per driver instance |
| Async throughout | `tokio` + `async-trait` for all driver operations |
| Caching | Access tokens cached with TTL (M-Pesa: 55 min) |

---

## 9. Scalability Considerations

| Dimension | Approach |
|-----------|----------|
| Driver count | Plugin-style factory; new providers add crate + register in factory |
| Benchmark suites | Data-driven; add suite by implementing `run_<suite>_benchmarks()` |
| Architecture nodes | Single `architecture.ts` source; nodes/edges as data, not code |
| MCP tools | Add via `#[tool]` macro; no router changes needed |
| CI jobs | Matrix strategy; add platform/target via workflow matrix |

---

## 10. Deployment Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     GitHub Actions CI                        │
├─────────────────────────────────────────────────────────────┤
│  cargo-check  │  cargo-test  │  cargo-clippy  │  cargo-fmt  │
│  cargo-audit  │  cargo-deny  │  TruffleHog    │  SBOM       │
│  build-linux  │  build-macos │  build-windows │  bench      │
│  arch-viz     │  publish     │  release       │             │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
        ┌──────────┐   ┌──────────┐   ┌──────────┐
        │  Linux   │   │  macOS   │   │ Windows  │
        │ .AppImage│   │  .dmg    │   │  .msi    │
        └──────────┘   └──────────┘   └──────────┘
              │               │               │
              └───────────────┼───────────────┘
                              ▼
                    ┌───────────────────┐
                    │  GitHub Releases  │
                    │  (auto on tag)    │
                    └───────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
        ┌──────────┐   ┌──────────┐   ┌──────────┐
        │ Desktop  │   │   Web    │   │  MCP     │
        │  Users   │   │  (Docker)│   │  Agents  │
        └──────────┘   └──────────┘   └──────────┘
```

---

## 11. Testing Strategy

| Level | Tools | Coverage |
|-------|-------|----------|
| Unit | `cargo test` (per crate) | Domain types, drivers, benchmark stats |
| Integration | `cargo test --workspace` | Cross-crate, MCP server, benchmark runner |
| E2E | Manual + benchmark CLI | Full query flows, regression detection |
| Property | `proptest` (planned) | Currency mapping, config validation |
| Security | `cargo audit`, `cargo deny`, TruffleHog | Vulns, licenses, secrets, bans |

---

## 12. Future Improvements

### 12.1 Short-term (Next 1-2 cycles)
- [ ] Complete all 11 mobile money driver implementations
- [ ] Add 4 remaining African banking drivers (TISS, UIP, ZECH, BACH, NAMSWITCH, SIPS, STP, SATIM)
- [ ] Implement African domain MCP tools (10+ tools)
- [ ] Add property-based tests for currency/country mapping
- [ ] Benchmark baseline establishment (first run = baseline)

### 12.2 Medium-term
- [ ] WebAssembly build for browser-based DBX
- [ ] Plugin marketplace for African domain drivers
- [ ] Real-time mobile money transaction monitoring via MCP
- [ ] DHIS2/OpenMRS schema introspection via MCP resources

### 12.3 Long-term
- [ ] AI-assisted African domain query generation (SQL from natural language)
- [ ] Cross-border transaction correlation (mobile money + banking)
- [ ] Compliance reporting templates (NIBSS, GHIPSS, CBN, etc.)
- [ ] Offline-first sync for intermittent connectivity

---

## 13. Appendix: File Inventory (New/Modified)

### New Files
```
crates/dbx-african-domain/Cargo.toml
crates/dbx-african-domain/src/lib.rs
crates/dbx-driver-mobile-money/Cargo.toml
crates/dbx-driver-mobile-money/src/lib.rs
crates/dbx-driver-african-banking/Cargo.toml
crates/dbx-driver-african-banking/src/lib.rs
crates/dbx-driver-dhis2/Cargo.toml
crates/dbx-driver-dhis2/src/lib.rs
crates/dbx-driver-openmrs/Cargo.toml
crates/dbx-driver-openmrs/src/lib.rs
crates/dbx-bench/Cargo.toml
crates/dbx-bench/src/lib.rs
crates/dbx-bench/src/main.rs
architecture-viz/package.json
architecture-viz/tsconfig.json
architecture-viz/vite.config.ts
architecture-viz/src/main.tsx
architecture-viz/src/ArchitectureVisualization.tsx
architecture-viz/src/data/architecture.ts
architecture-viz/src/components/*.tsx (7 node components)
README.md (this repo)
DESIGN_DOCUMENT.md
PORTFOLIO_CASE_STUDY.md
BEFORE_AFTER.md
.github/workflows/ci.yml
.github/workflows/security.yml
.github/workflows/release.yml
.env.example
```

### Modified Files
```
Cargo.toml (workspace members + exclude)
crates/dbx-mcp/Cargo.toml (version bump)
```

---

## 14. Verification Checklist

- [x] All new crates compile (`cargo check --workspace`)
- [x] Unit tests pass (`cargo test --package dbx-african-domain`, etc.)
- [x] Benchmark CLI runs (`cargo run --bin dbx-bench -- --list`)
- [x] Architecture viz builds (`cd architecture-viz && pnpm build`)
- [x] CI pipeline passes (all jobs green)
- [x] Security scan clean (no secrets, no critical vulns)
- [x] License headers present (Apache-2.0)
- [x] Documentation complete (README, Design Doc, Case Study, Before/After)
- [x] Attribution clear (original project, license, authors)

---

*This design document captures the engineering rationale for DBX Enhanced. It should be read alongside the [Portfolio Case Study](PORTFOLIO_CASE_STUDY.md) and [Before/After Comparison](BEFORE_AFTER.md).*