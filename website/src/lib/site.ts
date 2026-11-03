/**
 * Facts about the site that more than one page states. The version is the
 * `fiscalclock` dependency in package.json, the package the landing page and
 * the playground run, so a version bump there updates every page that names it.
 */
import pkg from "../../package.json";

export const FISCALCLOCK_VERSION = pkg.dependencies.fiscalclock;

/** The Rust crate's API documentation for that version. */
export const DOCS_RS_URL = `https://docs.rs/fiscalclock/${FISCALCLOCK_VERSION}/fiscalclock/`;
