Prefix: DSLP
Next: 9

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier E). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After DSLP-5, work the open P1 and P2 findings, then fill in `## Report` below.

## Report

Not started.

## Tickets

### DSLP-6 · P1 · Rebuild version when a linked worktree branch advances

- **Area:** build · version
- **Where:** build.rs; src/git_rerun.rs
- **Problem:** In a linked worktree git rev-parse --git-dir points to .git/worktrees/<name>, while the branch ref resides in the common Git directory. After commit 95840a5, cargo build --locked --bin fd completed but target/debug/fd --version still reported 59e6e0233b95 instead of 95840a5aab0d. The watched ref path is wrong and the binary embeds stale revision information.
- **Recommendation:** Watch the actual Git reference location and packed refs as needed; add a regression exercising a linked worktree branch advance and the built version.
- **Fix with:** root-cause
- **Effort:** M

### DSLP-8 · P3 · Provide a reproducible local search performance workload

- **Area:** performance · benchmarks
- **Where:** README.md; Makefile
- **Problem:** README.md links an external benchmark repository, but this tree has no executable benchmark harness or pinned local workload. Search throughput, startup, memory and performance attribution cannot be compared from this checkout.
- **Recommendation:** Design a representative, correctness-checked search workload with pinned fixtures and paired comparison, then add a bounded local entrypoint.
- **Fix with:** bench-pyramid
- **Effort:** M

