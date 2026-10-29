# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| Latest release | ✅ |
| Previous minor | ✅ Security fixes only |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities via public GitHub issues.**

Instead, please report them via email to: **nakisisageorge@gmail.com**

Include the following information:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will acknowledge receipt within 48 hours and provide a timeline for remediation.

## Security Measures

### Dependency Scanning
- **cargo-audit**: Runs on every PR and weekly schedule. Fails on critical/high severity vulnerabilities.
- **cargo-deny**: Enforces license compliance, bans problematic crates, checks version requirements.
- **SBOM Generation**: CycloneDX JSON + SPDX generated on every release and weekly.

### Secret Detection
- **TruffleHog**: Scans entire git history on every PR and weekly schedule. Fails on any detected secret.
- **Pre-commit hooks**: Local secret scanning via `.husky/pre-commit` (install with `pnpm prepare`).

### Secure Coding Practices
- All API communications use TLS (`reqwest` with `rustls-tls`)
- Credentials never logged; only masked in debug output
- MCP tool arguments validated via `schemars` JSON Schema
- Connection scoping and tool allowlist in MCP server
- Input sanitization on all user-facing APIs

### Supply Chain Security
- All dependencies from crates.io (no git dependencies in production)
- Workspace-internal path dependencies only
- `Cargo.lock` committed and verified in CI
- Minimal MSRV (1.80) with regular updates

### Build Security
- Multi-platform builds in isolated GitHub Actions runners
- Release artifacts signed via GitHub Attestations (planned)
- SBOM attached to every release

## Threat Model

| Asset | Threat | Mitigation |
|-------|--------|------------|
| Database credentials | Leakage via logs, config files | Env vars only, `.env.example` placeholders, secret scanning |
| API keys (mobile money, banking) | Theft, misuse | Short-lived tokens (M-Pesa: 55min), scope-limited, rotation docs |
| MCP server exposure | Unauthorized access | Transport auth (planned), tool allowlist, connection scoping |
| Dependency vulnerabilities | Exploit via transitive deps | cargo-audit, cargo-deny, weekly scans, SBOM |
| Supply chain attack | Malicious crate injection | crates.io only, lockfile verification, deny bans |

## Security Checklist for Contributors

- [ ] No secrets in code, comments, or commit messages
- [ ] All new dependencies reviewed for license compatibility (Apache-2.0/MIT/BSD)
- [ ] New MCP tools include `schemars` validation schemas
- [ ] New API integrations use TLS and short-lived credentials
- [ ] Error messages don't leak sensitive information
- [ ] Run `cargo audit` and `cargo deny check` locally before PR
- [ ] Run `trufflehog` locally if adding new config patterns

## Incident Response

1. **Detect**: Automated scans (cargo-audit, TruffleHog) + user reports
2. **Assess**: Severity classification (CVSS), affected versions
3. **Contain**: Revoke compromised credentials, disable affected features
4. **Remediate**: Patch, rebuild, release new version
5. **Notify**: Security advisory via GitHub, email to affected users (if any)
6. **Review**: Post-incident analysis, update threat model

## Security Contacts

- **Primary**: George Nakisisa (nakisisageorge@gmail.com)
- **Company**: GICT Africa Technologies (https://gictafrica.com)
- **PGP Key**: Available on request

## Acknowledgments

We thank the security research community and the following projects for their security tooling:
- [cargo-audit](https://github.com/RustSec/cargo-audit) by RustSec
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) by Embark Studios
- [TruffleHog](https://github.com/trufflesecurity/trufflehog) by Truffle Security
- [cargo-cyclonedx](https://github.com/CycloneDX/cargo-cyclonedx) by CycloneDX