# Example definitions used by the website

The website's guides show these definitions, and its code examples in [`website/examples/`](../../website/examples/) read them. They are illustrative: none is the published calendar of an organisation, and none cites a filing.

| File | What it shows | Page |
| --- | --- | --- |
| [`retail-4-5-4.json`](retail-4-5-4.json) | A 4-5-4 retail calendar whose year ends on the Saturday nearest 31 January. Its definition is identical to that of the conformance vector [`nrf-53-week-years.json`](../../conformance/vectors/cited/nrf-53-week-years.json), which checks its 53-week years against the NRF's published list; the period that takes the extra week and the label templates are this example's own choices. | Quick start |
| [`missing-choices.json`](missing-choices.json) | A definition that leaves out `year_number` and `extra_week`, so validation rejects it. | Write a definition |
| [`two-schemes.json`](two-schemes.json) | The same years with two naming schemes that both write `FY{yyyy}`, so a label such as `FY2024` is ambiguous. | Labels |

`npm run test:examples` in `website/` runs every code example from this folder, so a change here that alters an example's output fails that check.
