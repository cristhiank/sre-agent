# Authoring records that retrieve and read

Write for a responder using ranked lexical search under a clock, not for a reader
browsing a folder tree. The measured consumer sees roughly twenty files and only
a few surrounding lines. A fact that does not rank is functionally absent; a
line that ranks but cannot be read is not a useful hit.

The shape below is therefore a response to retrieval mechanics, not a schema to
fill.

## Let the question choose the shape

Use a **record** when the answer is “here is what is true.” State the needed fact
in that record even when another source is canonical, because a second retrieval
hop may never happen and a vocabulary-free pointer cannot match the query.

Use an **index** when the answer is “go here.” Keep it pointer-shaped because
copying explanations into an index creates competing answers and makes stale
duplication harder to detect.

Keep one independently routable subject per record. A subject deserves its own
file when a responder would search for it by name and act on the result. Keep
rows that only make sense together in one focused catalog; splitting every
endpoint, field, or enum member forces the responder to reconstruct the set.

## Make each part earn its place

Organize the record around the decisions it must support. Headings and keyed
lines are useful only when they answer one of these questions:

| Reader question | Content that earns space | Why it earns space |
|---|---|---|
| Why did this record match? | Literal alert, route, scenario, job, exception, status-code, and feature identifiers; alternate phrasing; stable stems of generated names | Lexical search cannot match a paraphrase, and a stable stem survives generated suffixes. |
| What is happening? | A concise symptom or verdict with the affected scope and the important non-impact | The responder needs the incident shape before spending time on mechanism. |
| Why this cause rather than its nearest look-alike? | A discriminator, falsifier, null-result meaning, and the rival explanation | A label without a way to disprove it anchors the investigation instead of narrowing it. |
| What can I do next? | A bounded probe with source coordinates, exact fields or dimensions, join keys, and empty-result traps | Coordinates alone still leave field discovery and silent query failure to the responder. |
| How much should I trust? | Evidence grade, pinned citations, freshness, unresolved questions, and the probe that would strengthen the claim | Thin evidence should produce a thin claim rather than invented closure. |

Do not create empty sections for questions the evidence cannot answer. State the
gap where it affects a decision, because a short honest record is safer than a
complete-looking fabrication.

## Use search weight deliberately

The measured ranker gives full paths and headings more weight than ordinary body
text. It also rewards rare terms and query coverage, normalizes for file length,
and saturates repeated terms. These mechanics explain the authoring choices:

- Put the rarest stable identifier in the filename and subject heading, because
  path and heading matches carry more ranking signal than another body mention.
- Put operational identifiers at the start of informative keyed lines, because
  a controlled test scored `opt-out` at 76 in key position and 23 in a value;
  `exclusion` scored 78 versus 33.
- Include the distinct literals a real alert carries, because coverage across
  rare terms beats repetition of one term. Repeating a token after saturation
  only consumes context.
- Keep a record focused, because length normalization favors the file devoted to
  the answer over an aggregate containing the same words among many subjects.
- Keep one fact per line and aim below about 200 characters, because the
  consumer receives narrow snippets. In a same-facts conversion, shortening the
  longest line from 2,771 to 187 characters helped move the correct record from
  rank 22 to rank 1 for a real alert title.

Key position is not permission to manufacture keys. The value must teach
something the key alone did not.

```text
BAD   TaskExpiredException: identifier in this record
GOOD  TaskExpiredException: raised when queued work reaches its deadline before dispatch

BAD   429: status code
GOOD  429: the dependency rejected this caller's quota key; local capacity was still healthy
```

## Prefer a compact working shape

This example shows the questions in one readable record. Adapt it to the
evidence; do not reproduce its labels merely to look complete.

```text
# queued-work-expiry — work times out before dispatch

QueuedWorkExpiry: alert family for work that passes its latest-start deadline
TaskExpiredException: emitted by the expiry path; no dependency call has begun
POST-/v1/jobs: stable route stem carried by the alert and request telemetry
aka: queue timeout, scheduled work expired, dispatch never started

## QueuedWorkExpiry vs dependency timeout

discriminator: no outbound dependency attempt exists for the failing work identifier
falsifier: a bound dependency attempt with a transport timeout makes this record wrong
empty-result-trap: searching by display name returns no rows; use the route-form scenario value

## Bounded probe

source: request and scheduler telemetry for the affected deployment scope
join: exact work identifier across creation, dispatch, and expiry events
fields: scenario, deployment scope, status, created time, latest-start time

## Evidence

trust: source-inferred; production confirmation remains open
verify-later: run the bounded join for one known occurrence and confirm no dispatch event
```

## Avoid transformations that destroy meaning

### Hollow lines

Delete a keyed line whose value restates its key, record ID, or location. It
games retrieval without explaining the term and pushes useful lines out of the
returned context.

### Over-splitting

Split at answer boundaries, not by row count, byte count, or a target number of
files. Ask: **would a responder search for this item by name and act on it
alone?** If yes, separation gives it filename weight and a self-sufficient
answer. If no, keep the set together so its relationships survive retrieval.

An independently diagnosable failure, telemetry source, join key, or query trap
often passes this test. One configuration field, endpoint, changelog item, or
verification-queue row usually does not.

### Enumeration collapse

Preserve each source-backed member and its distinct meaning. Replacing
`disabled`, `not-configured`, and `denied` with “not enabled” removes literal
query terms and erases different mechanisms. Group members in one record when
the set is the answer, but do not summarize away the members.

Never turn a discrete list into a continuous range unless the source asserts
continuity. `scope 1, 3, 7` is not `scope 1–7`; the latter invents four members.

### Range truncation

Keep both bounds, the unit, and what the range measures. Turning an observed
`16–81 seconds` into `16 seconds`, or a cited source span `lines 17–43` into
`line 17`, changes the evidence and may hide the branch that qualifies the
claim. If only one endpoint is verified, say so instead of completing the
range.

### Fabricated citations

Cite the pinned identity and span actually read. Do not guess a line number,
extend a partial span, copy a stale citation without resolving it, or make an
inferred location look source-proven. A missing citation is a visible gap; a
plausible false citation blocks verification while appearing trustworthy.

### Inflated certainty

Do not manufacture a discriminator, owner, route, abbreviation expansion, or
look-alike to fill the shape. Preserve the source's evidence grade and name the
missing evidence, because newer or better prose does not strengthen a claim.
