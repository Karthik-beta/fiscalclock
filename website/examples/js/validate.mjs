import { readFileSync } from "node:fs";
import { validate } from "fiscalclock";

const result = validate(readFileSync("retail-4-5-4.json", "utf8"));
console.log(result.ok, result.calendar_id);
