import { readFileSync } from "node:fs";
import { validate } from "fiscalclock";

const result = validate(readFileSync("missing-choices.json", "utf8"));
for (const error of result.errors) {
  console.log(error.path, error.code);
}
