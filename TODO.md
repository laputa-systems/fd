Prefix: FDF
Next: 6

# Open work
### FDF-5 · P3 · Provide a reproducible local search performance workload

- **Area:** performance · benchmarks
- **Where:** README.md; Makefile
- **Problem:** README.md links an external benchmark repository, but this tree has no executable benchmark harness or pinned local workload. Search throughput, startup, memory and performance attribution cannot be compared from this checkout.
- **Recommendation:** Design a representative, correctness-checked search workload with pinned fixtures and paired comparison, then add a bounded local entrypoint.
- **Fix with:** bench-pyramid
- **Effort:** M

