# Pull Request

> **Rules enforced by CI/Danger:**
>
> * PR must reference an issue (e.g., `Closes #123`)
> * Branch name must include an issue number (e.g., `feat/123-short-desc`)
> * PR title must follow Conventional Commits (e.g., `feat(api): add endpoint`)
> * **How can a reviewer verify?** must include numbered steps
> * **System Impact** must have at least one checkbox checked
> * No build artifacts committed (e.g., `target/`, `*.o`, `*.a`, `*.iso`, `*.img`)
> * Large PRs (>800 LOC) will be warned
> * If Rust code changes, consider updating `CHANGELOG.md`

---

## Linked Issue

**Required.** Use a closing keyword so GitHub links + closes the issue.

* Closes #

Examples:

* `Closes #123`
* `Fixes #88`
* `Resolves #52`
* `Refs #21` (use only when you don’t want auto-close)

---

## What does this change do?

Describe the change in plain English.

* What new behavior exists after this PR that did not exist before?

---

## Why is this change needed?

Explain the reason for the change.

* Bug? Feature? Refactor? Infrastructure?
* What problem does this solve?

---

## How was this tested?

Describe what you ran and what you observed.

Examples:

* ran LocalStack and invoked endpoint
* uploaded file(s) to S3 and verified downstream processing
* sent SQS message and verified handler behavior
* executed Step Function and verified success/failure paths
* booted kernel in QEMU and verified output
* started/stopped VM through libvirt and verified lifecycle

Include commands if helpful.

---

## System Impact

**Required.** Check at least one area that this PR affects.

* [ ] API behavior
* [ ] Event flow
* [ ] Data model / database
* [ ] Retry behavior
* [ ] Idempotency
* [ ] Failure handling
* [ ] Infrastructure (CDK / deployment)
* [ ] Performance
* [ ] Logging / observability

---

## How can a reviewer verify?

**Required.** Provide reproducible steps. Must include a numbered list.

1.
2.
3.

## Expected result:

---

## Breaking Changes

* [ ] No
* [ ] Yes (explain below)

If yes, describe what breaks and how to migrate.

---

## Risks

What could go wrong?

Examples:

* duplicate processing
* race conditions
* ordering issues
* retries creating side effects
* data corruption

---

## Additional Notes

Anything else a reviewer should know (tradeoffs, follow-ups, context).

---

## Checklist

* [ ] Linked to an issue (`Closes #123`)
* [ ] Branch name includes issue number (e.g., `feat/123-short-desc`)
* [ ] PR title follows Conventional Commits (e.g., `feat(api): add endpoint`)
* [ ] Includes reviewer verification steps
* [ ] CI passes
* [ ] No build artifacts committed (`target/`, `*.o`, `*.a`, `*.iso`, `*.img`)
* [ ] Added/updated tests if applicable
* [ ] Updated docs/changelog if behavior is user-facing
