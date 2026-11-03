# Security policy

fiscalclock computes dates from data. A security problem here could be a crash or a hang on crafted input, unbounded memory use, a supply-chain compromise of a published package, or a wrong result that an attacker can trigger deliberately.

## Supported versions

Security fixes go into the latest minor release only. As of 3 Nov 2026 the only release is 0.1.0, released on 31 Oct 2026 (see the [changelog](CHANGELOG.md)).

| Version | Supported |
| --- | --- |
| Latest 0.x minor release | Yes |
| Older releases | No |

## Reporting a vulnerability

Please report vulnerabilities privately through GitHub's [private vulnerability reporting](https://github.com/Karthik-beta/fiscalclock/security/advisories/new). Do not open a public issue, discussion or pull request.

Include:

- the affected package (Rust crate, Python package or npm package) and version;
- a calendar definition and inputs that reproduce the problem;
- what happens and what you expected.

You should receive an acknowledgement within seven days. Once a fix is ready, it is released and a security advisory is published that credits you, unless you ask not to be named.

## Scope

Wrong fiscal dates that are not caused deliberately are bugs, not vulnerabilities. Report them with the [bug report](https://github.com/Karthik-beta/fiscalclock/issues/new?template=bug_report.yml) or [wrong result](https://github.com/Karthik-beta/fiscalclock/issues/new?template=wrong_result.yml) forms.
