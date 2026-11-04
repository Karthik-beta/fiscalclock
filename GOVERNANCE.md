# Governance

This document explains who maintains fiscalclock, how decisions are made, and how the definition format and the API are versioned and kept stable.

## Maintainers

fiscalclock currently has one maintainer, [@Karthik-beta](https://github.com/Karthik-beta), who has final say on all decisions and on releases. The project would benefit from more maintainers. A regular contributor whose reviews have been consistently careful can be invited to become a maintainer; the invitation and the acceptance are recorded in a Discussions thread, and this file is updated in the same pull request that adds them to `.github/CODEOWNERS`.

## How decisions are made

Design decisions are recorded as Architecture Decision Records (ADRs) on the [wiki](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-Index).

1. Anyone may propose an ADR. It starts with the status **Proposed**.
2. Each Proposed ADR has a thread in the [Design decisions](https://github.com/Karthik-beta/fiscalclock/discussions) category, where the options and trade-offs are discussed in the open.
3. The thread stays open for at least seven days, so that people in other time zones and with other calendars can respond. A maintainer may shorten this for a decision that blocks a security fix.
4. A maintainer then records the outcome in the ADR (**Accepted** or **Rejected**), with the reasoning and a link to the thread, and closes the thread with a summary.
5. An Accepted ADR is changed only by a new ADR that supersedes it. The old ADR keeps its text and gains a "Superseded by" link.

Decisions that need no ADR (bug fixes, documentation, tests, tooling) are made in issues and pull requests.

## Versioning and stability

fiscalclock publishes three things that carry compatibility promises.

| Contract | Version | Promise |
| --- | --- | --- |
| Calendar definition format | `spec_version` (an integer, currently 1) | A definition that is valid for a given `spec_version` keeps the same meaning in every later library release. A change that would alter the meaning of a valid definition requires a new `spec_version`. New optional fields with no effect on existing definitions may be added within a version. |
| JSON outputs and error codes | API version (currently 1) | Keys, value types and error codes do not change within an API version. New keys may be added. Error `message` and `hint` text may change at any time. |
| Packages on crates.io, PyPI and npm | [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) | All three packages share one version number and are released together from the same commit. |

Before 1.0.0, a minor release (0.x.0) may make breaking changes to the package APIs. Every breaking change is listed in `CHANGELOG.md` with migration notes. A change to the result of any operation for a valid input is treated as breaking even when it fixes a bug, and is announced in the changelog under its own heading.

From 1.0.0, breaking changes to package APIs require a major release, and the definition format and API version guarantees above become permanent.

## Releases

Releases are made from a tagged commit on `main` by the release workflow, which publishes to all three registries with build provenance. A release is made only when every check in the release's roadmap gate passes. The gates are listed on the [Roadmap](https://github.com/Karthik-beta/fiscalclock/wiki/Roadmap).

## Code of Conduct

Everyone taking part follows the [Code of Conduct](CODE_OF_CONDUCT.md).
