# Releasing fiscalclock

Only the maintainer releases. One tag publishes the same version to crates.io, PyPI and npm from one commit on `main`, then creates the GitHub release. [`.github/workflows/release.yml`](.github/workflows/release.yml) does the work. This page lists the steps, what each gate checks, and what to do when a step fails.

## 1. Prepare the release pull request

1. Set the new version in the package files: the three `Cargo.toml` manifests (`crates/core`, `bindings/python`, `bindings/js`), `bindings/python/pyproject.toml`, `bindings/python/python/fiscalclock/__init__.py`, `bindings/js/package.json`, `bindings/js/package-lock.json`, and the `fiscalclock` dependency pins in the two bindings' manifests. `python -I scripts/check_versions.py` prints them first and fails until they all agree, and when `release.yml` passes it the tag, until they also agree with the tag. Leave the tool pins (the minimum supported Rust version, maturin, wasm-bindgen, pandas, yamllint, check-jsonschema, zizmor) and the website's `fiscalclock` pins alone. The script checks only that each of those groups agrees within itself, and the website pins move after the release (see "After the release").
2. In `CHANGELOG.md`, move the entries under "Unreleased" to a new `## [x.y.z] - YYYY-MM-DD` section below it, leave the "Unreleased" heading empty, and update the `[Unreleased]` and `[x.y.z]` links at the bottom. The release notes are this section, read by `scripts/release_notes.py`.
3. Run `python -I scripts/check.py`, and merge the pull request when CI is green.

A pull request that changes how the packages are built (the manifests, `Cargo.lock`, `scripts/smoke.*` or `release.yml` itself) also runs the release workflow as a dry run. It builds every wheel, the sdist, the crate and the npm tarball, and runs the smoke tests on them.

## 2. Tag the merge commit

Wait until the `ci` and `docs` workflows have passed on the merge commit on `main`, then push a tag for it:

```sh
VERSION=0.2.0 # example: use the version from step 1
git switch main && git pull
git tag "v$VERSION"
git push origin "v$VERSION"
```

Before anything is published, the `check` job confirms these:

- every manifest states the tagged version;
- the tagged commit is on `main`;
- `CHANGELOG.md` has a section for the version;
- `ci` and `docs` have a successful run for the tagged commit.

If the last check fails because CI is still running, wait for it to finish, then re-run the failed job. If CI was cancelled because a newer commit reached `main` first, re-run that cancelled run (`gh run rerun <run-id>`), or run the workflow on the tag (`gh workflow run ci.yml --ref "v$VERSION"`). Both use the tagged commit; a new run on `main` would test `main`'s latest commit instead.

## 3. What runs before publishing

- The crate is packaged and verified, and the Python wheels (seven platforms, each with a stable-ABI and a free-threaded wheel), the sdist and the npm tarball are built.
- npm checks the tarball with `npm publish --dry-run`, using the same path that the publish step uses.
- The smoke tests install the built files and run `scripts/smoke.py` and `scripts/smoke.mjs` on Linux x86_64 and arm64, macOS arm64 and x86_64, and Windows. The tests cover the abi3 wheels on CPython 3.10, the free-threaded wheels on CPython 3.14t, the abi3 musllinux wheels on Alpine Linux, the sdist built from source on Linux x86_64, and the npm tarball on Node.js 22.
- Every file gets a build provenance attestation.

## 4. Approve the three publish jobs

Each registry has its own GitHub environment (`crates-io`, `pypi`, `npm`) that accepts only `v*` tags and waits for the maintainer's approval. All three use trusted publishing, so the repository stores no registry tokens. Before it publishes, the crates.io job installs the Rust version that packaged the crate, packages it again, and checks that the file is identical to the attested one.

When all three have published, the `github-release` job creates the GitHub release with the changelog section and every built file.

## If a publish job fails

As of 4 Nov 2026, none of the three registries accepts a version again once it has been published or used, so do not delete the tag and push it again:

- crates.io: "a publish is generally permanent. The version can never be overwritten, and the code cannot be deleted." ([Cargo book, Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)).
- PyPI: "PyPI does not allow for a filename to be reused, even once a project has been deleted and recreated." ([PyPI troubleshooting](https://docs.pypi.org/publishing/troubleshooting/)).
- npm: "Once package@version has been used, you can never use it again." ([npm unpublish policy](https://docs.npmjs.com/policies/unpublish/)).

To recover:

- Find the cause in the failed job's log, then re-run only the failed jobs from the run page or with `gh run rerun <run-id> --failed` ([GitHub: re-running workflows and jobs](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/re-run-workflows-and-jobs)). GitHub re-runs "all of the failed jobs and their dependent jobs" ([REST API: re-run failed jobs](https://docs.github.com/en/rest/actions/workflow-runs#re-run-failed-jobs-from-a-workflow-run)), so a publish job that already succeeded is not in that set and does not publish again.
- If the cause needs a code change, the version cannot be completed from this tag. Fix the cause on `main` and release the next patch version. Say in its changelog section which registries missed the earlier version.

## After the release

- The website runs a published release, not the code on `main`. Move its `fiscalclock` pins to the new version in one pull request. `scripts/check_versions.py` lists the five files under "fiscalclock release the website runs".
