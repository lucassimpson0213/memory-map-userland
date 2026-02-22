---
name: Parent / Initiative
about: Track a cross-repository capability or system outcome
title: "[Initiative] "
labels: initiative
assignees: ""
---

# Objective

Describe the real-world capability this initiative establishes.

**When this issue is complete, I will be able to:**

> _______________________________________________

(Example: Boot my kernel inside a VM using libvirt-cli and reliably observe early boot output.)

---

# Why This Matters

Explain what confusion, failure, or limitation currently exists.

- What cannot be determined today?
- What kind of debugging or development is blocked?

> _______________________________________________

---

# Definition of Done (System-Level Evidence)

This initiative is complete when the following observable truths are all satisfied:

- [ ] ___________________________________________
- [ ] ___________________________________________
- [ ] ___________________________________________
- [ ] ___________________________________________
- [ ] ___________________________________________

⚠️ These should be **facts about the system**, not actions you perform.
Good: “VM boots and output is visible”  
Bad: “Implement logging function”

---

# Child Issues (By Repository)

Link issues in the repositories that actually implement the work.

## kernel
- [ ] <link to issue>
- [ ] <link to issue>

## libvirt-cli
- [ ] <link to issue>
- [ ] <link to issue>

## lab-infra / environment
- [ ] <link to issue>
- [ ] <link to issue>

Only link — **do not duplicate the details here.**

---

# Dependencies / Blockers

List anything external or prerequisite:

- permissions / virtualization setup
- hardware limitations
- missing tooling
- environment configuration

> _______________________________________________

---

# Observations

Short discoveries while working (not implementation plans).

Example:
> `qemu:///session` behaves differently from `qemu:///system` regarding domain visibility.

- _______________________________________________
- _______________________________________________

---

# Current Focus

One sentence describing the immediate area of work.

> _______________________________________________

---

# Status

Keep this extremely short.

> Active / Blocked / Paused

If blocked, state by what:
> Blocked by: ____________________________________
