# Contributing to DBX Enhanced

Thank you for your interest in contributing! This project welcomes contributions from the community.

## Code of Conduct

By participating, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md) (adapted from Contributor Covenant).

## How to Contribute

### Reporting Bugs
1. Check existing issues first
2. Create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment (OS, Rust version, DBX version)
   - Logs/screenshots if applicable

### Suggesting Features
1. Check existing issues/discussions
2. Create a feature request issue with:
   - Problem statement
   - Proposed solution
   - Alternatives considered
   - Implementation hints (if any)

### Pull Requests
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Run quality checks locally:
   ```bash
   cargo fmt --all --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   cargo audit
   cargo deny check
   ```
5. Commit with conventional messages: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
6. Push and open a PR

## Development Setup

### Prerequisites
- Rust 1.80+ (`rustup install 1.80`)
- Node.js 20+ and pnpm 9+
- System dependencies (see CI workflow)

### Build Commands
```bash
# Install frontend deps
pnpm install

# Build frontend
pnpm build

# Check Rust workspace
cargo check --workspace

# Run tests
cargo test --workspace

# Run benchmarks
cargo run --release --bin dbx-bench -- --list

# Build architecture viz
cd architecture-viz && pnpm install && pnpm build
```

## African Domain Contributions

We especially welcome contributions for:
- **Mobile money providers**: Complete implementations for the 10 stubbed providers
- **Banking systems**: Complete the 6 remaining interbank systems
- **Healthcare connectors**: OpenLMIS, iHRIS, mHero, CommCare, ODK
- **MCP tools**: African domain query tools for mobile money, banking, healthcare

### Adding a New Mobile Money Provider
1. Add provider to `MobileMoneyProvider` enum in `dbx-african-domain`
2. Add country codes, display name, API URLs
3. Create driver in `dbx-driver-mobile-money`
4. Implement `Driver` trait with actual API calls
5. Add to `create_mobile_money_driver()` factory
6. Add tests (unit + mock integration)
7. Update documentation

### Adding a New Banking System
1. Add system to `AfricanBankingSystem` enum in `dbx-african-domain`
2. Create driver in `dbx-driver-african-banking`
3. Implement `Driver` trait
4. Add to `create_african_banking_driver()` factory
5. Add tests and documentation

## Benchmark Contributions

### Adding a Benchmark Suite
1. Add suite name to `run_all()` match in `dbx-bench/src/lib.rs`
2. Implement `run_<suite>_benchmarks()` method
3. Follow existing pattern: iterations, statistical analysis, baseline comparison
4. Add CLI integration if needed

### Improving Regression Detection
- Statistical significance testing (t-test, Mann-Whitney)
- Multiple baseline comparison (median of last N runs)
- Trend analysis over time

## Architecture Visualization

### Adding Nodes/Edges
1. Edit `architecture-viz/src/data/architecture.ts`
2. Add node to `architectureNodes` array
3. Add edges to `architectureEdges` array
4. Create component if new category: `architecture-viz/src/components/`
5. Register in `ArchitectureVisualization.tsx` nodeTypes

### Styling Guidelines
- Use category colors from `categoryColors`
- Keep node positions organized (grid-like)
- Edge labels: concise relationship type
- Minimap should remain readable

## Code Style

### Rust
- Follow `rustfmt` default style
- `clippy` pedantic lints enabled
- Public APIs must have doc comments
- Error types use `thiserror`, `anyhow` for applications

### TypeScript/Vue
- Follow project ESLint/Prettier config
- TypeScript strict mode
- Composition API preferred
- Component props typed with interfaces

### Commits
```
feat: add M-Pesa STK push callback handling
fix: resolve DHIS2 analytics pagination bug
docs: update benchmark CLI usage examples
refactor: extract AfricanCurrency mapping to separate module
test: add property tests for currency/country mapping
chore: update cargo-deny config for new license
```

## Testing Guidelines

### Unit Tests
- Place in `#[cfg(test)] mod tests` in same file
- Test public API, error paths, edge cases
- Use `wiremock` for HTTP mocking

### Integration Tests
- Place in `tests/` directory per crate
- Test cross-crate interactions
- Use `tempfile` for filesystem isolation

### Benchmark Tests
- Run with `cargo test --package dbx-bench`
- Verify statistical calculations
- Test regression detection logic

## Documentation

### Required for New Features
- Update `README.md` if user-facing
- Add doc comments to public Rust APIs
- Update `architecture-viz` if architecture changes
- Add MCP tool documentation if adding tools

### Documentation Files
- `README.md` — Project overview, quick start
- `DESIGN_DOCUMENT.md` — Architecture decisions
- `PORTFOLIO_CASE_STUDY.md` — Engineering narrative
- `BEFORE_AFTER.md` — Detailed comparison
- `SECURITY.md` — Security policy
- `CHANGELOG.md` — Version history (auto-generated from commits)

## Release Process

Maintainers only:
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create tag: `git tag vX.Y.Z`
4. Push tag: `git push origin vX.Y.Z`
5. GitHub Actions creates release with artifacts

## License

By contributing, you agree that your contributions will be licensed under the Apache-2.0 license (same as the project).

## Questions?

- Open a GitHub Discussion
- Email: nakisisageorge@gmail.com
- X: @gictafrica

---

*DBX Enhanced — Building technology for Africa and beyond.*