

# Rust Userland Systems Sandbox

**Purpose:**
Learn ownership, memory, and invariants in userland *before* writing a kernel.
-------------------------------------------------------------------------------
-------------------------------------------------------------------------------

# Wiki
Where most of the rust thoughts go
[Wiki](https://github.com/lucassimpson0213/sys-userland-kernel-rust/wiki)
----------------------------------------------------------------------------
----------------------------------------------------------------------------


## What I Do

Follow Brown Rust Book:
[https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html)

Rust for rustasceans:
[rust for rustasceans](https://github.com/rustaccato/road-to-being-master-rustacean/blob/main/Rust%20for%20Rustaceans.pdf)

# Systems Learning Loop (Operational Edition)



## How To Use This Document

Every week you pick **one failure phenomenon** and run it through the
full loop. -- https://github.com/Simpson-Systems/sys-userland-kernel-rust/issues/22

# Weekly Phenomenon Checklist

Pick ONE per week. Do not combine.

- [ ] Duplicate execution
- [ ] Lost acknowledgement
- [ ] Crash during write
- [ ] Partial commit
- [ ] Stale read
- [ ] Message reordering
- [ ] Split brain
- [ ] Replay after recovery

---

This week:

> _________DUPLICATES__________________

Invariant:

> ___________________________

Enforced by:

> ___________________________

------------------------------------------------------------------------

# The Weekly Loop

## Stage 1 --- Observe (MIT 6.824) \| Mon--Tue

Goal: Identify the broken guarantee.

Work on a single lab slice.

Before fixing anything, answer:

1.  What failed?
2.  What state existed before the failure?
3.  What state existed after the failure?
4.  What assumption was violated?

Deliverable: Write a short note describing the violated invariant.

Rule: Do not patch the code yet.

You are diagnosing a system, not debugging a program.

------------------------------------------------------------------------

## Stage 2 --- Explain (Notes) \| Wed Morning

No coding.

Explain the failure in plain terms.

Restrictions: - no Go - no RPC - no Raft - no APIs

Only: state + time + ownership

Example: "Two workers both believed they owned the same state transition
because no durable record established ownership."

Deliverable: One page explanation.

If you cannot explain it simply, you do not yet understand it.

------------------------------------------------------------------------

## Stage 3 --- Model (Rust Userland Simulation) \| Wed Afternoon

use Rust brown book
Implement a small Rust simulation (\~100--300 lines).
https://github.com/Simpson-Systems/sys-userland-kernel-rust/issues/11

Do NOT build a service. Do NOT use networking.

Build a model that reproduces the failure.

Examples: - commit log - scheduler - lock - message queue - page table

Goal: Make the failure happen locally and predictably.

Deliverable: A program where you can intentionally trigger the failure.

If you cannot reproduce it, you do not yet understand it.

------------------------------------------------------------------------

## Stage 4 --- Implement a Rule (Maelstrom) \| Thu

Now implement a prevention rule using unreliable messaging.

You are not solving Maelstrom exercises. You are enforcing a guarantee.

Possible rules: - idempotency - sequence numbers - deduplication -
ownership - monotonic state machines

Deliverable: A working node that prevents the failure you studied.

------------------------------------------------------------------------

## Stage 5 --- Reinforce (Return to MIT) \| Fri

Return to the MIT lab.

Now fix the bug.

You should notice: You are no longer guessing solutions. You know which
property is missing.

This is the moment systems intuition forms.

------------------------------------------------------------------------

## Stage 6 --- Reality (Reliability / Storage Day) \| Sat

Apply the same failure to persistent state.

Perform real-world experiments:

-   kill a process during a write
-   restart a container mid-operation
-   interrupt a backup
-   restore from snapshot
-   intentionally corrupt a file
- https://github.com/Simpson-Systems/sys-userland-kernel-rust/issues/12

Then answer:

What truth survived?

Deliverable: A restore test that proves recovery works.

This is where distributed systems meets real systems.

------------------------------------------------------------------------

## Stage 7 --- Compression (OS Mechanism) \| Sun

Only now touch the kernel or low-level code.

Implement ONE mechanism: - interrupt handler - scheduler tick - memory
flag - syscall boundary

The OS is now enforcing the invariant you learned.

Rule: Never open the OS project out of guilt.

The OS is a **compression artifact of understanding**.

------------------------------------------------------------------------

# Why This Loop Works

You are seeing the same idea at multiple levels:

  Layer        What You Learn
  ------------ -------------------------
  MIT          failure observation
  Notes        causal reasoning
  Rust model   representation
  Maelstrom    protocol guarantees
  Storage      real-world consequences
  OS           mechanism enforcement

This maps:

machine → process → data → disk → hardware

------------------------------------------------------------------------

# Saturday Reliability Checklist -- https://github.com/Simpson-Systems/sys-userland-kernel-rust/issues/12

Every week perform at least one:

-   Verify backups run successfully
-   Perform a file restore
-   Perform a directory restore
-   Restore onto a clean machine
-   Kill service mid-backup
-   Disconnect storage during write
-   Run `restic check`
-   Validate snapshot history

If you have never restored your backups, you do not have backups.

------------------------------------------------------------------------

# Completion Criteria (After Several Months)

You should naturally be able to:

-   reason about retries safely
-   design idempotent APIs
-   understand write-ahead logs
-   understand Raft logs
-   understand crash recovery
-   build a durable queue
-   recover systems after failure

You are no longer learning programming.

You are learning **guarantees**.
