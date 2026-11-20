Prefix: DSLP
Next: 9

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier E). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After DSLP-5, work the open P1 and P2 findings, then fill in `## Report` below.

## Report

Base: `59e6e0233b95fc9fff271c538fe37f7a98df59bf`; upstream merge base: `c44735ec0f130c2486afe9c976937c03b57a8c62`. All measurements are macOS arm64, default features, busy shared host; CI minutes: none (local-only pass).

- Scorecard: Rust `src/` lines 6,596 → 6,616 (includes inline tests); `tests/` lines 3,456 → 3,456. Escape-hatch matching lines 129 → 129. Dead code and dependencies removed: 0. Findings filed: one P1 fixed (DSLP-6), one P3 open (DSLP-8), zero P2/P4. Escaped regressions: none observed, not a guarantee. Performance verdict: no runtime hot path changed; no local benchmark or defensible absolute timing gate.
- Ground: added the shared test entrypoint and documented boundaries. Initial cold test with in-repository TMPDIR failed in 97.73s because an upstream fixture detected the enclosing Git repository; owner selected an outside-repository TMPDIR. Subsequent suite passed 183 unit and 117 integration tests, 38.48s including rebuild; corrected warm run 10.78s. Initial lint 91.67s. The gha-test-workflow checker is unavailable; workflow inspection shows pull-request test and lint calls.
- Diagnose: focused fork review reproduced the stale linked-worktree SHA and filed DSLP-6. Test 10.94s, lint 35.28s. Review was limited to fork changes; upstream code was not audited.
- Types: `cargo clippy --locked --bin fd` with the nine candidate warnings produced 26 diagnostics: unsafe 1, wildcard enum 5, unwrap 16, needless pass by value 4; match wildcard single variant, truncation, sign loss, expect and redundant clone each 0. Adopted none: no demonstrated fork defect or trust deletion at the required false-positive rate. Test 41.70s, lint 47.35s.
- Tests: Edit is the relevant `cli::fd::tests` selector; Change is `make test`. Suite runner times varied on the busy host; integration execution was 1.42s at baseline and 1.78s after, within the 30s budget. Critical sampled parser contracts: version and unknown-option errors. Replacing each with help caused its named test to fail (two caught, zero missed, zero timeout or unviable); restored before the passing suite. The linked-worktree ref regression failed before its fix and passed after. Automated mutation and line/branch coverage: Needs environment, cargo-mutants and cargo-llvm-cov unavailable; no automated survivor count or CI mutation lane is claimed. `make coverage` has no target. Test 12.41s, lint 32.99s.
- Dead code: `cargo rustc --locked --bin fd -- -W dead_code -W unreachable_pub` found no dead_code diagnostics and 164 unreachable_pub visibility suggestions. One category-level disposition: binary crate public visibility is not an external export and does not establish dead code; no cut was proven. cargo-machete and udeps: Needs environment. Test 12.26s, lint 35.93s.
- Bench: no benchmark harness is present in this tree; README links an external repository. Outcomes needing coverage: startup, representative search throughput, CPU, memory, and attribution; filed DSLP-8 (P3). No benchmark was run or speedup claimed. Test 39.18s, lint 33.10s.
- Finding: DSLP-6 watches the common Git directory's branch and packed refs. Regression failed before the fix; after commit `555f314`, a fresh `cargo build --locked --bin fd` reported `555f314a6518`, matching HEAD. Test 12.17s, lint 45.22s. Build-time path only; search runtime unaffected.
- Report review: checked the cumulative diff and removed test-only unwraps added during the fix. Shared-file gap: the deslop scratch rule needs an exception for tests that assume their temporary directory has no Git ancestor; this pass used the owner's external TMPDIR ruling. Method feedback: an actual worktree branch-advance check catches what the original path-only tests missed. Remaining: DSLP-8; no open P1/P2.

## Tickets

### DSLP-8 · P3 · Provide a reproducible local search performance workload

- **Area:** performance · benchmarks
- **Where:** README.md; Makefile
- **Problem:** README.md links an external benchmark repository, but this tree has no executable benchmark harness or pinned local workload. Search throughput, startup, memory and performance attribution cannot be compared from this checkout.
- **Recommendation:** Design a representative, correctness-checked search workload with pinned fixtures and paired comparison, then add a bounded local entrypoint.
- **Fix with:** bench-pyramid
- **Effort:** M

