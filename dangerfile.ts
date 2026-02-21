import { danger, fail, warn, message } from "danger";

const pr = danger.github.pr;
const body = pr.body || "";
const branch = danger.github.pr.head.ref;

const modified = danger.git.modified_files;
const created = danger.git.created_files;
const allFiles = [...modified, ...created];

//
// -------------------------------
// 1) PR MUST REFERENCE AN ISSUE
// -------------------------------
//

const issueRegex =
    /(close[sd]?|fix(e[sd])?|resolve[sd]?|refs?|related to)\s+#\d+/i;

if (!issueRegex.test(body)) {
    fail(`
❌ This PR does not reference an issue.

Add one of the following to the PR description:

Closes #123
Fixes #123
Resolves #123
Refs #123

Every change must be tied to tracked work.
`);
}

//
// -------------------------------
// 2) BRANCH MUST CONTAIN ISSUE NUMBER
// -------------------------------
//

// -------------------------------
// 2) BRANCH MUST CONTAIN ISSUE NUMBER
// -------------------------------

const branch =
    danger.github?.pr?.head?.ref ??
    (danger as any).github?.thisPR?.head?.ref ??
    "";

const actor = pr.user?.login ?? "";
const isBot = pr.user?.type === "Bot" || /bot/i.test(actor);

// allow merge PRs / automation PRs to bypass if you want
const isMergePR =
    /^Merge\b/i.test(pr.title) || /dev\s*->\s*stable/i.test(pr.title);

const branchIssueRegex =
    /^(feat|fix|docs|refactor|test|chore|build|ci|perf)\/\d+([-/].+)?$/i;

if (!isBot && !isMergePR) {
    if (!branch) {
        warn("Could not determine PR branch name in Danger context. Skipping branch naming rule.");
    } else if (!branchIssueRegex.test(branch)) {
        fail(`
❌ Branch name must include the issue number.

Required format:
  type/123-description

Examples:
  feat/123-paging
  fix/88-memory-leak
  refactor/52-allocator
`);
    }
}

//
// -------------------------------
// 3) CONVENTIONAL COMMITS TITLE
// -------------------------------
//

const conventional =
    /^(feat|fix|docs|style|refactor|test|chore|build|ci|perf)(\(.+\))?: .+/;

if (!conventional.test(pr.title)) {
    fail(`
❌ PR title must follow Conventional Commits:

feat(kernel): add paging
fix(memory): prevent double free
refactor(allocator): simplify free list
`);
}

//
// -------------------------------
// 4) DESCRIPTION QUALITY
// -------------------------------
//

if (body.trim().length < 30) {
    warn("PR description is very short. Explain WHY this change exists.");
}

//
// -------------------------------
// 5) HUGE PR WARNING
// -------------------------------
//

const bigPR = pr.additions + pr.deletions > 800;
if (bigPR) {
    warn("Large PR detected (>800 lines). Consider splitting it.");
}

//
// -------------------------------
// 6) PREVENT BUILD ARTIFACTS
// -------------------------------
//

const badFiles = allFiles.filter(f =>
    f.startsWith("target/") ||
    f.endsWith(".o") ||
    f.endsWith(".a") ||
    f.endsWith(".iso") ||
    f.endsWith(".img")
);

if (badFiles.length) {
    fail(`Build artifacts committed: ${badFiles.join(", ")}`);
}

//
// -------------------------------
// 7) CHANGELOG SUGGESTION
// -------------------------------
//

const touchesRust = allFiles.some(f => f.endsWith(".rs"));
const touchedChangelog = allFiles.some(f => /changelog/i.test(f));

if (touchesRust && !touchedChangelog) {
    message("Rust code changed — consider updating CHANGELOG.md if user-facing.");
}
