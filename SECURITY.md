# Security Policy

RustUse takes security issues seriously. Please report suspected vulnerabilities privately so they can be investigated and addressed before public disclosure.

## Supported Versions

Security fixes are provided for the latest published version of `rustuse-cli`.

Pre-release versions are supported on a best-effort basis while RustUse remains under active development. Older versions may be unsupported after a newer release becomes available.

| Version                     | Supported   |
| --------------------------- | ----------- |
| Latest release              | Yes         |
| Older releases              | Best effort |
| Unreleased development code | Best effort |

Users should upgrade to the latest available version before reporting an issue that may already have been resolved.

## Reporting a Vulnerability

Do not disclose suspected vulnerabilities in a public GitHub issue, discussion, pull request, commit, or other public channel.

Use GitHub's private vulnerability reporting for this repository:

1. Open the repository's **Security** tab.
2. Select **Report a vulnerability**.
3. Provide the information requested below.

If private vulnerability reporting is unavailable, contact the repository maintainers through GitHub without publicly including sensitive technical details.

Include as much of the following information as possible:

- The affected RustUse version or commit.
- The affected operating system and Rust toolchain.
- A description of the vulnerability and its potential impact.
- The conditions required to reproduce the issue.
- Minimal reproduction steps or proof-of-concept code.
- Relevant logs, stack traces, commands, or configuration.
- Whether the issue is known to have been publicly disclosed.
- Any suggested mitigation or remediation.

Remove credentials, access tokens, private keys, personal information, and unrelated sensitive data from reports.

## Response Process

After receiving a report, maintainers will make a reasonable effort to:

1. Acknowledge the report within three business days.
2. Perform an initial assessment within seven business days.
3. Confirm whether the issue is accepted, rejected, duplicated, or requires more information.
4. Develop and test an appropriate remediation.
5. Coordinate disclosure and release timing with the reporter when practical.
6. Publish a security advisory when the issue materially affects users.

Resolution time depends on the issue's severity, complexity, reproducibility, and the availability of a safe fix.

The maintainers may request additional information during investigation. Reports that cannot be reproduced may be closed unless further evidence becomes available.

## Coordinated Disclosure

Please allow maintainers a reasonable opportunity to investigate and remediate a vulnerability before publishing technical details.

Do not:

- Exploit a vulnerability beyond what is necessary to demonstrate it.
- Access, modify, retain, or disclose data belonging to other users.
- Disrupt RustUse infrastructure, repositories, registries, or third-party services.
- Use social engineering, phishing, denial-of-service attacks, or physical attacks.
- Publish exploit details before a coordinated disclosure date has been agreed upon.

Once a fix is available, maintainers may publish:

- A GitHub security advisory.
- A patched crate release.
- Updated release binaries.
- Upgrade or mitigation instructions.
- Appropriate credit for the reporter.

Reporter attribution is optional and will be omitted upon request.

## Security Scope

Security reports may include issues involving:

- Execution of unintended commands or processes.
- Argument, path, shell, or configuration injection.
- Unsafe filesystem modification or deletion.
- Directory traversal or operations outside the requested project.
- Mishandling of credentials, tokens, or sensitive configuration.
- Dependency or package-resolution behavior that creates a security risk.
- Installation or update mechanisms.
- Release artifacts or provenance.
- Git, GitHub, GitLab, Cargo, or crates.io integrations.
- CI automation maintained by this repository.
- Vulnerabilities in RustUse-owned code or official release artifacts.

Reports about third-party dependencies are welcome when they materially affect RustUse and include a practical impact or reachable vulnerable path.

## Out of Scope

The following are generally not considered security vulnerabilities:

- Ordinary defects without a security impact.
- Feature requests or usability concerns.
- Unsupported operating systems, toolchains, or configurations.
- Findings that require a user to intentionally execute already-trusted malicious code.
- Vulnerabilities that exist only in an unrelated third-party service.
- Automated scanner output without evidence of applicability or impact.
- Missing optional hardening that does not create an exploitable condition.
- Denial-of-service reports requiring unrealistic local resource consumption.
- Social engineering, phishing, or physical attacks.
- Publicly known dependency advisories that do not affect reachable RustUse behavior.

Non-security defects should be reported through the repository's normal issue tracker.

## Dependency Vulnerabilities

RustUse uses automated dependency and security checks, but automated tooling cannot establish whether every advisory is reachable or exploitable.

When reporting a dependency vulnerability, include:

- The advisory identifier.
- The affected crate and version.
- The dependency path from `rustuse-cli`.
- The RustUse feature or command that reaches the affected code.
- Evidence of practical impact when available.
- A compatible fixed version or mitigation.

## Security Updates

Security fixes may be delivered through a patch, minor, or pre-release version depending on compatibility and project maturity.

Users are responsible for monitoring published advisories and updating their installations. After a security release, maintainers may limit support for vulnerable versions.

## Good-Faith Research

Security research performed in good faith and within this policy is welcomed.

Good-faith activity means:

- Avoiding harm to users, maintainers, infrastructure, and third parties.
- Testing only what is reasonably necessary to validate a finding.
- Reporting the issue privately and promptly.
- Protecting any sensitive information encountered.
- Allowing reasonable time for remediation before disclosure.
- Complying with applicable laws and third-party terms.

This policy does not authorize testing against infrastructure, accounts, repositories, or services that you do not own or have permission to test.
