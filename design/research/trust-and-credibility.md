<!-- cspell:words WCAG prototypicality colorfulness makinggood Schwarz metacognitive Raluca Behavioral ICSE CSCW SPDX skeptical PNAS glassmorphism Yeh Miratrix Mardiko Mund Fogg Makovsky Soohoo Danielson Marable Tauber Lindgaard Fernandes Dudek Tractinsky Ikar Kurosu Kashimura Tuch Presslaber Stöcklin Stoecklin Opwis Bargas Avila Reinecke Gajos Robins Piepenbrock Mayr Buchner Tufte Bateman Pandey Rall Satterthwaite Bertini Correll Franconeri Crameri Shephard Borland Trockman Vasilescu Kästner Doshi Hauser Altay Gilardi Hayward Fitza Asmus Leth Datawrapper WebAIM Budiu Wathan Ryo Sigstore OpenSSF Scorecard SLSA OpenAlex figshare Lebedev Heimer Oppenheimer -->

# Trust and credibility in visual design

Research notes for fiscalclock's visual identity and design system, compiled 1 November 2026. The question: what does evidence say about how visual design changes perceived trust and credibility, and how can fiscalclock earn that trust honestly rather than borrow it?

## How to read these notes

Each finding gives the claim, the source, the strength of the evidence and the practical rule it implies. Strength uses five labels:

- **Experiment**: a peer-reviewed controlled study.
- **Field or survey**: a large observational dataset, automated scan or survey.
- **Qualitative**: interviews or small expert panels.
- **Guidance**: a standard, a canonical expert reference or a usability research firm's published guidance.
- **Anecdote**: blogs, single cases or practitioner commentary without a stated method.

Many publisher DOI pages (ACM, Taylor & Francis, Elsevier, SAGE) answer automated link checkers with HTTP 403. Where that happens, the DOI is given as code and the link points to an open record that was fetched for these notes: the [OpenAlex](https://api.openalex.org/works/doi:10.1080/01449290500330448) metadata record (which carries the abstract), PubMed, PMC, arXiv or an author or institutional copy. Where only an abstract was read, the finding says so.

## The rules in one table

| # | Rule for fiscalclock | Strongest evidence |
| --- | --- | --- |
| R1 | Make the evidence prominent, not just present: conformance counts, citations and verification commands sit where people look first. | Guidance (prominence-interpretation theory), Qualitative (expert panel) |
| R2 | Keep pages visually simple and conventional in structure; express identity through detail, type and real content, not unusual layouts. | Experiment (Tuch et al. 2012; Reinecke et al. 2013) |
| R3 | One accent colour, a small neutral ramp and a strict component set; never decorate with colour. | Experiment (Reinecke et al. 2013), Guidance (NN/g heuristics) |
| R4 | Polish must never outrun proof: anything illustrative or unverified is labelled as such in the design, not only in prose. | Experiment (Robins and Holmes 2008; fluency research) |
| R5 | Meet WCAG 2.2 AA in both themes, with extra care for small text in dark mode. | Guidance (WCAG 2.2), Experiment (display polarity) |
| R6 | Draw periods to scale: a diagram's lengths are proportional to days, so a 14-week quarter and a 53-week year look exactly as much longer as they are. | Guidance (Tufte's lie factor), Experiment (truncation studies) |
| R7 | Never use colour alone to carry meaning (such as the extra week); label it. No rainbow or red-green scales. | Guidance (WCAG 1.4.1), Experiment and review (Crameri et al. 2020) |
| R8 | Show dates as ISO `YYYY-MM-DD` with half-open bounds and `last_day`; show only the precision the data has, never times or time zones on date results. | Guidance (W3C), mixed Experiment evidence on precision |
| R9 | Use only assessment signals (computed by CI or by a verifier), not decorative badges or self-assertions. | Field or survey (Trockman et al. 2018) |
| R10 | Version, status, licence, maintainer and "last reviewed" date are visible on every docs page and release; no superlatives anywhere. | Guidance (Stanford guidelines, SemVer, Keep a Changelog), Qualitative (expert panel) |
| R11 | Avoid the visual tells now associated with generated sites (stock gradients, glow, generic three-card feature grids); lead with the product's real output. | Anecdote only; indirect Experiment evidence on homogenisation |

## 1. Web credibility research

### 1.1 Prominence-interpretation theory

- **Claim.** Fogg's prominence-interpretation theory says a credibility assessment needs two steps: the user notices an element (prominence) and then judges it (interpretation). An element nobody notices has no effect. The paper's example: a privacy policy rated as important in a telephone survey had virtually no effect when people browsed real sites, because they almost never noticed it. Prominence rises with the user's involvement, topic, task, experience and individual differences. Fogg calls the theory preliminary.
- **Source.** B.J. Fogg, "Prominence-Interpretation Theory: Explaining How People Assess Credibility Online", CHI 2003 short talk ([PDF on the Stanford Web Credibility Project site](https://credibility.stanford.edu/pdf/PITheory.pdf); `doi:10.1145/765891.765951`). Read in full.
- **Strength.** Guidance (a theory paper summarising four years of the lab's studies with over 6,500 participants, by the paper's own account; not itself an experiment).
- **Rule.** Proof that is buried does not count. Put the assessment signals (R9) and the "how to verify" block where a first-time visitor looks: the top of the README, the docs home page and each release page. High-involvement readers (finance teams checking a reporting date) notice more, including typographical errors, so the docs must be proofread as carefully as the code.

### 1.2 Consumers judge the look; finance experts judge the content

- **Claim.** In a 2002 comparison, more than half (54.6%) of consumer comments on finance sites concerned "design look", against 16.4% of comments from finance experts. Across all ten content categories in the parallel Stanford study the consumer average was 46.1%. Finance experts focused on information focus, company motive and bias; when they used design at all it was, in the report's words, "mostly used to identify non-credible sites with flashy graphics". Health experts valued cited sources and named authors with credentials.
- **Source.** Stanford, Tauber, Fogg and Marable, [Experts vs. Online Consumers: A Comparative Credibility Study of Health and Finance Web Sites](https://advocacy.consumerreports.org/wp-content/uploads/2013/05/expert-vs-online-consumers.pdf), Consumer WebWatch, October 2002. Read in full. The underlying consumer study is Fogg et al., "How do users evaluate the credibility of Web sites?", DUX 2003 ([OpenAlex record](https://api.openalex.org/works/doi:10.1145/997078.997097); `doi:10.1145/997078.997097`); its full text was not retrieved.
- **Strength.** Qualitative for the experts (eight health and seven finance experts); Field or survey for the consumer side (over 2,500 participants, figures taken from the 2002 report).
- **Rule.** fiscalclock has both kinds of reader. The visual layer must pass the fast consumer-style judgement, but the people who decide whether to adopt it read like the finance experts: they look for sources, motive and bias, and treat flashy graphics as a warning. So: no hype visuals, no promotional tone, and every claim one click from its source (R1, R10).

### 1.3 Stanford Guidelines for Web Credibility

- **Claim.** Ten guidelines from three years of research involving over 4,500 people. Among them: make accuracy easy to verify; show the real organisation and the honest people behind the site; update often, or "at least show it's been reviewed recently"; "Use restraint with any promotional content"; avoid errors of all types.
- **Source.** B.J. Fogg, [Stanford Guidelines for Web Credibility](https://credibility.stanford.edu/guidelines/index.html), Stanford Persuasive Technology Lab, May 2002 (updated June 2002). The page lists its supporting studies; one, the 2002 Stanford-Makovsky study, is marked as a preliminary report that was not peer reviewed.
- **Strength.** Guidance (a research summary from 2002; the web has changed, but the guidelines are consistent with later work cited below).
- **Rule.** Every docs page carries a "last reviewed" date and a link to its source in the repository. The maintainer and governance are named (the repository already has `GOVERNANCE.md`). There is no promotional content on documentation pages.

### 1.4 Nielsen Norman Group's four credibility factors

- **Claim.** Four factors communicate trustworthiness: design quality; upfront disclosure of everything relevant to the user; comprehensive, correct and current content; and connection to the rest of the web, because users trust external sources more than self-description. A study with Western and Asian users found the same factors applied across both groups.
- **Source.** Aurora Harley, [Trustworthiness in Web Design: 4 Credibility Factors](https://www.nngroup.com/articles/trustworthy-design/), Nielsen Norman Group, 8 May 2016.
- **Strength.** Guidance (usability research firm; the article does not publish the study's data).
- **Rule.** Link outward to the evidence: annual reports and independent generators behind the conformance vectors, and the registries' own provenance pages. External corroboration does more than self-description.

## 2. Aesthetics and first impressions

### 2.1 First impressions form in a fraction of a second

- **Claim.** Ratings of a home page's visual appeal after 50 ms correlated highly with ratings after 500 ms, so visual appeal can be assessed within 50 ms.
- **Source.** Lindgaard, Fernandes, Dudek and Brown, "Attention web designers: You have 50 milliseconds to make a good first impression!", *Behaviour & Information Technology* 25(2), 115–126, 2006 ([OpenAlex record with abstract](https://api.openalex.org/works/doi:10.1080/01449290500330448); `doi:10.1080/01449290500330448`). Abstract only.
- **Strength.** Experiment.
- **Rule.** The first screen of the website and the README is judged before it is read. It must look calm, ordered and finished in both themes at first paint, with no layout shift and no loading spinners in front of the content.

### 2.2 Simple and familiar beats novel

- **Claim.** Visual complexity and prototypicality (how typical a page looks for its category) both shaped aesthetic ratings within 17 to 50 ms. Pages with low complexity and high prototypicality were rated most appealing; complexity acted earlier than prototypicality. A companion blog post by one of the authors summarises the finding as a strong preference for designs that look both simple and familiar.
- **Source.** Tuch, Presslaber, Stoecklin, Opwis and Bargas-Avila, "The role of visual complexity and prototypicality regarding first impression of websites", *International Journal of Human-Computer Studies* 70(11), 794–811, 2012 ([Google Research publication page](https://research.google/pubs/the-role-of-visual-complexity-and-prototypicality-regarding-first-impression-of-websites-working-towards-understanding-aesthetic-judgments/); `doi:10.1016/j.ijhcs.2012.06.003`), and Javier Bargas-Avila, [Users love simple and familiar designs](https://research.google/blog/users-love-simple-and-familiar-designs-why-websites-need-to-make-a-great-first-impression/), Google Research blog, 29 August 2012.
- **Strength.** Experiment.
- **Rule.** Use the conventions of documentation sites that developers already know (left navigation, content column, on-page contents, code blocks with a language tab and a copy button). Distinctiveness comes from typography, the mark, diagrams and real content, not from an unusual page structure.

### 2.3 Complexity matters more than colourfulness, but colour still costs

- **Claim.** In a study of 450 websites rated by 548 volunteers, models of perceived visual complexity and colourfulness, with demographics, explained about half the variance in appeal after 500 ms. Higher complexity sharply lowered appeal; colourfulness played a smaller role. Participants with a PhD were the most negatively affected by high colourfulness, and participants over 45 liked low-complexity sites more than other age groups.
- **Source.** Reinecke, Yeh, Miratrix, Mardiko, Zhao, Liu and Gajos, "Predicting users' first impressions of website aesthetics with a quantification of perceived visual complexity and colorfulness", CHI 2013 ([author PDF](https://iis.seas.harvard.edu/papers/2013/reinecke13aesthetics.pdf); `doi:10.1145/2470654.2481281`). Read in full.
- **Strength.** Experiment (online study with volunteers; demographic interactions are exploratory).
- **Rule.** A small palette: neutrals plus one accent, with semantic colours (error, warning, success) reserved for state. A highly educated, technical and finance audience is the group the study found least tolerant of high colourfulness.

### 2.4 The aesthetic-usability effect, and its limits

- **Claim.** Users show a "tendency to perceive attractive products as more usable". In the first HCI study of the effect (26 ATM layouts, 252 participants), aesthetic ratings correlated more strongly with perceived ease of use than with actual ease of use. A later experiment found that a system's aesthetics affected post-use perceptions of usability, while its actual usability did not. NN/g notes the effect forgives minor problems, not severe ones, and can mask real problems in usability tests.
- **Source.** Kate Moran, [The Aesthetic-Usability Effect](https://www.nngroup.com/articles/aesthetic-usability-effect/), Nielsen Norman Group, 3 February 2024 (summarising Kurosu and Kashimura, "Apparent usability vs. inherent usability", CHI 1995, `doi:10.1145/223355.223680`). Tractinsky, Katz and Ikar, "What is beautiful is usable", *Interacting with Computers*, 2000 ([OpenAlex record with abstract](https://api.openalex.org/works/doi:10.1016/S0953-5438%2800%2900031-X); `doi:10.1016/S0953-5438(00)00031-X`). Abstracts only for the two papers.
- **Strength.** Experiment.
- **Rule.** Good looks buy patience, not correctness. In usability tests of the docs, observe what people do with a real calendar definition, and do not count praise for the visuals as evidence the docs work.

### 2.5 Aesthetics raise credibility for identical content

- **Claim.** When the same content was shown with high and low aesthetic treatment, the higher treatment was judged more credible in 19 of 21 page pairs; credibility judgements took about 3.4 seconds on average. The authors' summary: "Given the same content, a higher aesthetic treatment will increase perceived credibility." They suggest the effect is probably limited to fast, visceral judgements and may fade as people engage with the content.
- **Source.** Robins and Holmes, "Aesthetics and credibility in web site design", *Information Processing & Management* 44(1), 386–399, 2008 ([PDF hosted by makinggood.ac.nz](https://makinggood.ac.nz/media/1276/robinsholmes_2008_aestheticsandcredibilityinwebdesign.pdf); `doi:10.1016/j.ipm.2007.02.003`). Read in full.
- **Strength.** Experiment, but small (20 participants, 840 judgements).
- **Rule.** This is the honesty problem in one finding: polish makes content look more credible whether or not it is. fiscalclock must not let the design lend credibility the evidence has not earned. Example definitions that have not been checked against a filing carry a visible "Illustrative" label in the same component every time, and verified examples carry a "Cited" label with the source (R4).

## 3. Restraint and consistency

### 3.1 Consistency and minimalism as heuristics

- **Claim.** Nielsen's heuristics include consistency and standards ("Follow platform and industry conventions.") and aesthetic and minimalist design: every extra unit of information competes with the relevant ones and lowers their visibility.
- **Source.** Jakob Nielsen, [10 Usability Heuristics for User Interface Design](https://www.nngroup.com/articles/ten-usability-heuristics/), Nielsen Norman Group, 24 April 1994, last reviewed 30 January 2024.
- **Strength.** Guidance (widely used expert heuristics, not a single experiment). The experimental support for "simple and familiar" is in 2.2 and 2.3.
- **Rule.** One component per job: one notice-box style per meaning, one code-block style, one table style, one way to show a period. The same term means the same thing everywhere (`end_exclusive` is never called "end date" in one place and "until" in another). Design tokens, not one-off values, so the system stays consistent across the website, docs and social cards.

## 4. Legibility and readability

### 4.1 Easy-to-read text is judged more true

- **Claim.** Processing fluency, the felt ease of processing, influences judgements across many domains; for example, visually clear statements are judged more true than less clear ones. The review cites Reber and Schwarz (1999), where statements in easier-to-read colours were more often judged true.
- **Source.** Alter and Oppenheimer, "Uniting the tribes of fluency to form a metacognitive nation", *Personality and Social Psychology Review*, 2009 ([PubMed record](https://pubmed.ncbi.nlm.nih.gov/19638628/); `doi:10.1177/1088868309341564`). Abstract only; Reber and Schwarz (1999, `doi:10.1006/ccog.1999.0386`) was not retrieved.
- **Strength.** Experiment (a review of many experiments).
- **Rule.** Legibility is a credibility signal and an ethical lever at the same time: it makes true and untrue statements alike more believable. Use it fully (comfortable size, line length and contrast), and pair it with the evidence rules so that only checked claims get the benefit (R4).

### 4.2 Display polarity and dark mode

- **Claim.** In acuity and proofreading tasks, dark text on a light background outperformed light text on dark for younger and older adults; the authors conclude: "The presentation in positive polarity is recommended for all ages." NN/g's review adds that the light-mode advantage grows as text gets smaller, that some low-vision users read faster in dark mode, and recommends offering dark mode as an option.
- **Source.** Piepenbrock, Mayr, Mund and Buchner, "Positive display polarity is advantageous for both younger and older adults", *Ergonomics*, 2013 ([PubMed record](https://pubmed.ncbi.nlm.nih.gov/23654206/); `doi:10.1080/00140139.2013.790485`). Abstract only. Raluca Budiu, [Dark Mode vs. Light Mode: Which Is Better?](https://www.nngroup.com/articles/dark-mode/), Nielsen Norman Group, 2 February 2020.
- **Strength.** Experiment, with Guidance for the synthesis.
- **Rule.** Dark mode stays first-class (as the project has decided), but it is the harder theme for reading, so: do not shrink body text in dark mode, avoid thin weights for small text on dark backgrounds, check every small-text pairing for contrast in both themes, and follow the reader's system setting rather than forcing one theme.

## 5. Accessibility as a trust signal

- **Claim (standard).** WCAG 2.2 Level AA requires a contrast ratio of at least 4.5:1 for normal text and 3:1 for large text (1.4.3), 3:1 for UI components and meaningful graphics (1.4.11), that colour is not the only means of conveying information (1.4.1), that content survives user text-spacing overrides (1.4.12), and visible, unobscured keyboard focus (2.4.7, 2.4.11).
- **Source.** [Web Content Accessibility Guidelines (WCAG) 2.2](https://www.w3.org/TR/WCAG22/), W3C Recommendation, 12 December 2024.
- **Strength.** Guidance (normative standard).
- **Claim (prevalence).** As of the February 2026 scan of one million home pages, 95.9% had detected WCAG 2 failures and 83.9% had low-contrast text, the most common failure.
- **Source.** WebAIM, [The WebAIM Million](https://webaim.org/projects/million/), 2026 report.
- **Strength.** Field or survey (automated scan; detects only machine-testable failures).
- **Claim (trust).** W3C's business case argues that accessibility enhances brand image and reputation, citing organisations such as Barclays and Microsoft.
- **Source.** W3C WAI, [The Business Case for Digital Accessibility](https://www.w3.org/WAI/business-case/), updated 15 July 2024.
- **Strength.** Guidance built on case studies. No controlled study was found that measures accessibility's direct effect on perceived credibility; treat "accessibility signals trust" as plausible and consistent with the legibility evidence (section 4), not as proven.
- **Rule.** Meet WCAG 2.2 AA in both themes as a release gate for the website and every published asset, and state the conformance target in the docs. Because most sites fail contrast, meeting it is a visible point of care, but do not claim more than has been tested.

## 6. Honest data presentation

### 6.1 The lie factor: draw intervals to scale

- **Claim.** Tufte defines the lie factor as the size of an effect shown in a graphic divided by the size of the effect in the data (*The Visual Display of Quantitative Information*, p. 57). In his fuel-economy example the graphic grows by 783% while the data grows by 53%, a lie factor of 14.8. A reader of Tufte's forum argues that dividing percentages gives inconsistent results; the reply defends Tufte's definition.
- **Source.** [Computing Lie Factor by Dividing Percentages](https://www.edwardtufte.com/notebook/computing-lie-factor-by-dividing-percentages/), Ask E.T. forum thread on Tufte's site (comments by Alexei Lebedev and Jim Heimer). The book itself (1983, second edition 2001, listed on [Tufte's books page](https://www.edwardtufte.com/books/)) was not consulted directly.
- **Strength.** Guidance (canonical expert reference).
- **Rule.** In every fiscalclock diagram, the horizontal length of a period is proportional to its number of days. A 4-4-5 quarter is drawn as 4:4:5; a 53-week year is exactly 7/364 longer than a 52-week year, not exaggerated to make the extra week easy to see. The extra week is highlighted by label and outline, never by stretching it.

### 6.2 Data-ink and its limits

- **Claim.** Tufte's data-ink ratio favours charts where most ink carries data; NN/g's guidance applies this by removing 3D effects, gradients and shadows, dropping redundant grid lines when values are labelled, and labelling series directly instead of using a legend. A controlled study found that embellished charts were described no less accurately than plain ones and were better recalled two to three weeks later; the authors stop short of recommending embellishment everywhere.
- **Source.** Kate Moran, [Cutting Chart Clutter](https://www.nngroup.com/articles/clutter-charts/), Nielsen Norman Group, 6 February 2022. Bateman et al., "Useful junk? The effects of visual embellishment on comprehension and memorability of charts", CHI 2010 ([OpenAlex record with abstract](https://api.openalex.org/works/doi:10.1145/1753326.1753716); `doi:10.1145/1753326.1753716`). Abstract only.
- **Strength.** Guidance for data-ink; Experiment for the counter-evidence.
- **Rule.** No 3D, gradients, shadows or decorative fills in diagrams. Label periods directly (`Q4`, `P12`, `W53`) rather than through a legend. Minimalism is a default, not a dogma: a short annotation that explains, for example, why a year has 53 weeks is welcome.

### 6.3 Truncated and cropped scales

- **Claim.** A crowdsourced study found that participants who saw deceptive charts, including bar charts with truncated axes, perceived the message in exaggerated form. A later series of experiments found that y-axis truncation inflates perceived effect size, and that "the subjective impact of axis truncation is persistent across visualizations designs", even with explicit visual cues marking the truncation.
- **Source.** Pandey et al., "How Deceptive are Deceptive Visualizations?", CHI 2015 ([OpenAlex record with abstract](https://api.openalex.org/works/doi:10.1145/2702123.2702608); `doi:10.1145/2702123.2702608`), with its result as reported by Datawrapper in [Why our column and bar charts start at zero](https://www.datawrapper.de/academy/why-our-column-and-bar-charts-start-at-zero) (updated 23 March 2021). Correll, Bertini and Franconeri, "Truncating the Y-Axis: Threat or Menace?", CHI 2020 ([arXiv](https://arxiv.org/abs/1907.02035); `doi:10.1145/3313831.3376222`). Abstracts only.
- **Strength.** Experiment.
- **Rule.** Timeline diagrams start at a meaningful boundary (the fiscal year's start) and show whole periods. If a diagram must zoom into part of a year, it shows its context (a small overview strip or explicit "…" boundaries with dates), knowing that break marks alone do not undo the exaggeration.

### 6.4 Colour in charts

- **Claim.** Colour maps with uneven gradients, such as rainbow and red-green maps, visually distort data and are unreadable for people with colour-vision deficiency; perceptually uniform scientific colour maps avoid this. An earlier expert article argues the rainbow map is a poor choice even for simple data.
- **Source.** Crameri, Shephard and Heron, [The misuse of colour in science communication](https://doi.org/10.1038/s41467-020-19160-7), *Nature Communications*, 2020. Borland and Taylor, [Rainbow Color Map (Still) Considered Harmful](https://doi.org/10.1109/MCG.2007.323435), *IEEE Computer Graphics and Applications*, 2007. Abstracts only.
- **Strength.** Guidance (peer-reviewed perspective and expert article, drawing on experimental vision research).
- **Rule.** fiscalclock's diagrams mostly show categories (year, quarter, period, week), not continuous values, so they need at most a few distinguishable neutrals plus the accent. Never encode meaning in hue alone (WCAG 1.4.1): the extra week, an exception, or a table year is also labelled in text and marked by outline or pattern. No rainbow or red-green scales in any asset.

### 6.5 Charts as arguments

- **Claim.** Alberto Cairo argues that charts must be read like text rather than glanced at, that readers project their beliefs onto them, and that designers have an ethical duty to see their graphics from the audience's side; his habits for readers include checking the source and accounting for uncertainty. (Paraphrased from an interview; the book was not consulted.)
- **Source.** [Becoming more mindful about visual information: A Q&A with Alberto Cairo](https://brown.columbia.edu/?p=5229), Brown Institute for Media Innovation, 18 November 2019, about *How Charts Lie* (2019, listed on [Cairo's site](https://www.albertocairo.com/)).
- **Strength.** Guidance.
- **Rule.** Every diagram carries a caption naming the calendar definition it was computed from (and, for cited examples, the filing), so a reader can check the source and reproduce the figure.

### 6.6 Exact dates without false precision

- **Claim (formats).** All-numeric dates are ambiguous across locales (US month-first, most of Europe day-first). W3C recommends "explicit month names and 4-digit years" where ambiguity is likely, and notes ISO 8601 `YYYY-MM-DD` is unambiguous and computer-friendly but less natural for people.
- **Source.** W3C Internationalization, [Date formats](https://www.w3.org/International/questions/qa-date-format).
- **Strength.** Guidance.
- **Claim (precision as a signal).** Evidence on whether precise numbers read as more confident is mixed. A study of managerial earnings forecasts found managers used more precise forecasts as an impression-management tactic after setbacks, and that investors generally responded favourably. A set of experiments with citizens (n = 1,505) found no convincing evidence that precision changed how performance numbers were judged.
- **Source.** Hayward and Fitza, "Pseudo-precision? Precise forecasts and impression management in managerial earnings forecasts", *Academy of Management Journal*, 2016 ([University of Exeter repository record](https://uoe.figshare.com/articles/Pseudo-precision_Precise_forecasts_and_impression_management_in_managerial_earnings_forecasts/29767637); `doi:10.5465/amj.2014.0304`). Asmus Leth Olsen, "Precise performance: Do citizens rely on numerical precision as a cue of confidence?", *Journal of Behavioral Public Administration*, 2018 ([OpenAlex record with abstract](https://api.openalex.org/works/doi:10.30636/jbpa.11.19)). Abstracts only.
- **Strength.** Experiment and Field (mixed results).
- **Rule.** fiscalclock's dates are exact, so showing them exactly is honest; the risk is precision the data does not have. In docs and diagrams:
  - show results as ISO `YYYY-MM-DD`, optionally followed by a named-month form (`2024-02-03`, 3 February 2024) for non-programmers;
  - always show the half-open interval with `start`, `end_exclusive` and the derived `last_day`, as the API does ([`spec/api-v1.md`](../../spec/api-v1.md));
  - never add a time of day, time zone or clock face to a date result, which would imply a timestamp the library rejects;
  - show rule-level statements at the precision the rule supports (for example, the 400-year count of 53-week years where it is computed and cited), not rounded averages presented as exact.

## 7. Trust signals for developer tools and open source

### 7.1 Badges: assessment signals versus decoration

- **Claim.** A study of 294,941 npm repositories plus a developer survey found that non-trivial badges (build status, test coverage, dependency freshness) "are mostly reliable signals", correlating with more tests, better pull requests and fresher dependencies. Badges that test an underlying quality (assessment signals) were more reliable than badges that only state information (conventional signals), and the effects did not always persist.
- **Source.** Trockman, Zhou, Kästner and Vasilescu, "Adding Sparkle to Social Coding: An Empirical Study of Repository Badges in the npm Ecosystem", ICSE 2018 ([author PDF](https://cmustrudel.github.io/papers/icse18badges.pdf); `doi:10.1145/3180155.3180209`). Read in full.
- **Strength.** Field or survey (large mining study with time-series analysis), correlational.
- **Rule.** Show only signals that something computes and could fail: CI status, the conformance count from the test run, provenance verification. Drop or demote purely informational badges. A conformance badge is generated by CI from the same run that writes the transcripts, never typed by hand.

### 7.2 What contributors look at

- **Claim.** Interviews with 15 GitHub contributors and data from 9,977 projects showed that people use signals such as the structure of the README and the amount of recent activity, that many important qualities lack easily observable signals, and that some signals are both attractive and unattractive.
- **Source.** Qiu et al., "The Signals that Potential Contributors Look for When Choosing Open-source Projects", *Proceedings of the ACM on Human-Computer Interaction* (CSCW), 2019 ([OpenAlex record with abstract](https://api.openalex.org/works/doi:10.1145/3359224); `doi:10.1145/3359224`). Abstract only.
- **Strength.** Qualitative (interviews) plus Field or survey (project data); peer-reviewed.
- **Rule.** The README is part of the brand: a fixed structure that is easy to scan (what it is, status, install, one example, why you can check the results, licence) that the website mirrors. Make hidden qualities observable: the number of cited sources and independent generators, and the date of the last conformance run.

### 7.3 Provenance and signed releases

- **Claim.** SLSA is a set of incrementally adoptable supply-chain guidelines with build levels; npm provenance records where and how a package was built, signed and logged in a public transparency ledger; PyPI attestations bind each file to a digest and can give publicly verifiable proof that it was published through a specific Trusted Publisher; crates.io Trusted Publishing (from July 2025, GitHub Actions at launch) removes stored publishing secrets. OpenSSF Scorecard checks, among others, signed releases, CI tests, a declared licence and whether a project is maintained.
- **Source.** [SLSA specification v1.2](https://slsa.dev/spec/v1.2/); npm Docs, [Generating provenance statements](https://docs.npmjs.com/generating-provenance-statements); PyPI Docs, [Attestations](https://docs.pypi.org/attestations/); Rust Blog, [crates.io: development update](https://blog.rust-lang.org/2025/07/11/crates-io-development-update-2025-07/), 11 July 2025; [OpenSSF Scorecard](https://scorecard.dev/); [OpenSSF Best Practices Badge](https://www.bestpractices.dev/en).
- **Strength.** Guidance (specifications and registry documentation; they define the mechanisms, not their effect on perceived trust).
- **Rule.** fiscalclock already publishes build provenance attestations for release files (as of 3 Nov 2026, per the README's [Supply chain](../../README.md#supply-chain) section; the `attest` job of the [release workflow](../../.github/workflows/release.yml) creates them, and `gh attestation verify <file> --repo Karthik-beta/fiscalclock` checks a downloaded file). Give verification a designed home: one "Verify this release" component, used on release pages and in the docs, with the exact command and what a pass looks like. Do not show a Scorecard or Best Practices badge until the project has actually run or earned it.

### 7.4 Versioning, changelogs and licence clarity

- **Claim.** Semantic Versioning: "Major version zero (0.y.z) is for initial development", and anything may change. Keep a Changelog: "Changelogs are for humans, not machines", with changes grouped as Added, Changed, Deprecated, Removed, Fixed and Security. The SPDX License List provides short identifiers for reliable identification of licences.
- **Source.** [Semantic Versioning 2.0.0](https://semver.org/); [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/); [SPDX License List](https://spdx.org/licenses/) (version 3.29.0, 16 September 2026, as of 1 November 2026).
- **Strength.** Guidance (community conventions).
- **Rule.** The current version and its status ("0.y: format and API are v1 drafts and may change before 1.0") appear in the site header or docs sidebar, styled as plain information rather than a promotional label. The licence appears as the SPDX expression `MIT OR Apache-2.0` in the footer of every page. The changelog is a first-class docs page with the same typography as the rest.

### 7.5 Maintainer identity, review dates and over-claiming

- **Claim.** The Stanford guidelines recommend showing the honest people behind a site, highlighting expertise, showing that content has been reviewed recently, using restraint with promotion and avoiding errors (section 1.3). Health experts in the 2002 panel rewarded named authors with credentials, and finance experts rewarded unbiased information over nudges towards a provider's own products (section 1.2).
- **Source.** As in sections 1.2 and 1.3.
- **Strength.** Guidance and Qualitative.
- **Rule.** Name the maintainer and link governance and security policy from every page footer. Every docs page shows "Last reviewed" with a date. Copy follows the repository's evidence rules: no superlatives, narrow comparisons only, every number linked to its source. Visual hierarchy never makes a claim larger than its evidence (no hero statistics without a citation underneath).

## 8. What makes a design look "AI-generated"

The evidence here is thin. No peer-reviewed study of the visual tells of generated websites was found.

- **Practitioner commentary (Anecdote).** Blog posts in 2025 describe a recurring look in sites produced with AI coding tools: Inter, Roboto or Arial; indigo or purple accents and gradients on light backgrounds; a centred hero with one call to action followed by three icon boxes; rounded corners and faint shadows; and missing hierarchy and accessibility details. The usual explanation (models copying a widely reused indigo default) is inference without a corpus analysis. Sources: [Why Your AI Keeps Building the Same Purple Gradient Website](https://prg.sh/ramblings/Why-Your-AI-Keeps-Building-the-Same-Purple-Gradient-Website), prg.sh, 26 October 2025; [Design in the AI Era: Beyond the Purple Gradient](https://www.startuphub.ai/ai-news/ai-video/2025/design-in-the-ai-era-beyond-the-purple-gradient/), StartupHub, 20 November 2025, reporting a Y Combinator design review in which Cursor's head of design, Ryo Lu, used the term "AI slop".
- **Homogenisation (Experiment, indirect).** In an online experiment, stories written with ideas from a large language model were rated more creative individually but were more similar to each other than stories written by people alone. This concerns short stories, not visual design; applying it to page design is an extrapolation. Source: Doshi and Hauser, "Generative AI enhances individual creativity but reduces the collective diversity of novel content", *Science Advances*, 2024 ([PMC full text](https://pmc.ncbi.nlm.nih.gov/articles/PMC11244532/); `doi:10.1126/sciadv.adn5290`). Abstract only.
- **Perception of AI involvement (Experiment, indirect).** In two preregistered experiments (N = 4,976), labelling headlines as AI-generated lowered their perceived accuracy whether they were true or false, and whether people or AI wrote them; participants assumed the label meant no human oversight. This concerns labelled text, not unlabelled visual style. Source: Altay and Gilardi, "People are skeptical of headlines labeled as AI-generated, even if true or human-made, because they assume full AI automation", *PNAS Nexus*, 2024 ([PMC full text](https://pmc.ncbi.nlm.nih.gov/articles/PMC11443540/); `doi:10.1093/pnasnexus/pgae403`). Abstract only.
- **Tension with section 2.2.** Familiar layouts score well on first impressions. The answer is not novelty of structure but specificity of content.
- **Rule.** Keep conventional structure (R2) and remove the generic tells: no purple-to-blue or mesh gradients, glows, glassmorphism or 3D blobs; no row of three icon cards with one-line benefit claims; no stock illustration or AI-generated imagery. Lead with real output: an actual definition, the actual JSON it returns and a to-scale diagram of an actual calendar. The project's admired references (design decision DD-004 in [`design/DECISIONS.md`](../DECISIONS.md)) share a restrained developer-docs genre that generated sites also imitate, so fiscalclock's distinctiveness has to come from its own content and diagrams rather than from that genre's surface.

## 9. Gaps and what was not verified

- Full texts not retrieved (abstract or secondary summary only): Fogg et al. 2003 (DUX), Lindgaard et al. 2006, Kurosu and Kashimura 1995, Tractinsky et al. 2000, Alter and Oppenheimer 2009, Piepenbrock et al. 2013, Bateman et al. 2010, Pandey et al. 2015 (its result taken from Datawrapper's summary), Correll et al. 2020, Crameri et al. 2020, Borland and Taylor 2007, Qiu et al. 2019, both precision studies, Doshi and Hauser 2024, and Altay and Gilardi 2024.
- Reber and Schwarz (1999) on fluency and truth was not retrieved; the claim rests on the 2009 review's abstract.
- Tufte's and Cairo's books were not consulted; the lie-factor definition comes from a forum thread on Tufte's site and Cairo's views from an interview.
- No direct experimental evidence was found that accessibility raises perceived credibility.
- No peer-reviewed evidence was found on the visual tells of AI-generated design; section 8 is commentary plus indirect findings.
- Dijkstra's note on half-open ranges (EWD831) was not reachable from this environment (TLS errors), so the half-open rule cites the project's own spec only.
- The Stanford guidelines and the 2002 expert study predate today's web; later experiments (sections 2 and 6) are consistent with them, but the specific percentages are from 2002.
