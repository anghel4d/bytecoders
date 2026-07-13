# CLAUDE.md

## What this is
bytecoders is the frozen record and reusable harness of a multi-model evolutionary code contest. A head orchestrator (Claude Opus) supervised six blind Opus agents; each agent commissioned fifteen implementations per component from a separate, cheap generation backend (the `claudex` CLI routing to gpt-5.6 `sol`/`luna` through a local `cli-proxy-api`). Across three generations the winning component of each stage propagated as fixed substrate to the next. The product is a working stack-machine bytecode VM at `artifact/final_vm.rs`: assembler, executor, peephole optimizer, each selected from ninety candidates by an objective fitness battery.

## The thesis it proves
Spend the metered-expensive model only on judgment, orchestration, and verification; push high-volume generation to a cheaper backend and keep its bulk output out of billed context. Here the 270 generation calls billed to the OpenAI-side proxy (~1% of a plan), every task under 250k context; the Claude plan paid only for the thin supervisory skeleton — six agents, ~30-70k tokens each.

## The hierarchy is the directory tree
- `orchestrator/` — the head level. `ORCHESTRATION.md` is the decision ledger; `substrate/{A,B,C}.rs` are the frozen winners propagated between generations (the surviving genome).
- `orchestrator/supervisors/agent-{1..6}/` — the six blind Opus supervisors.
- `.../agent-N/stage-{A-assembler,B-executor,C-optimizer}/candidates/` — the fifteen claudex candidates per stage, named `NN-<model>.rs` (01-05 sol/high, 06-15 luna/xhigh). `results.tsv` holds their fitness.
- `.../agent-N/champions/{A,B,C}.rs` — that agent's per-stage local winner.
- `harness/` — the shared tooling: `prelude.rs` (the fixed `Op` ISA), `golden/` (reference stages + a reference optimizer used to score stages in isolation), `testmain.rs` (the 16-program battery), `prompts/` (one per stage), and the two scripts.
- `artifact/final_vm.rs` — the assembled three-winner VM. Self-contained, runnable.
- `methodology/` — the design, the results, the economics, and recommendations for the next run (fleet sizing, token caps).
- `.claude/skills/coding-campaign/` — the GAN coder system packaged as a reusable skill: the user → commander → squadron-leader → sol/luna-cluster hierarchy and a domain-agnostic fleet runner (`reference/run_fleet.sh`, plus a PowerShell 7+ port `run_fleet.ps1`), with this contest's harness demoted to the worked example at `examples/bytecode-vm/`.
- `.claude/tools/` — the fleet runner as standalone project tools (`run_fleet.sh`, `run_fleet.ps1`), kept identical to the skill's `reference/` copies.

## Reproduce or re-run
Fitness of any A/B/C trio: `CONTEST_ROOT=harness bash harness/compose_and_test.sh <A.rs> <B.rs> <C.rs>` — emits `FITNESS pass=X/16 ... reduction=N`, or `BUILD_FAIL`/`TIMEOUT`/`CRASH`. The golden trio scores 16/16 reduction=0; the shipped winners score 16/16 reduction=23. Run the artifact: `rustc --edition 2021 -O artifact/final_vm.rs -o /tmp/vm && /tmp/vm`.

A full agent generation: `CONTEST_ROOT=harness WORK=/tmp/agent-1 bash harness/run_stage.sh A` commissions the fleet, tests each candidate against the substrate, ranks, and copies a provisional champion. Requires `claudex` on PATH and `rustc`. Seed `harness/substrate/` first — it doesn't exist at rest: copy `orchestrator/substrate/{A,B,C}.rs` to rebuild against the frozen winners, or the goldens (`goldenC.rs` as `C.rs`) to start a fresh contest at stage A. `run_stage.sh` is a thin stage adapter over the shared fleet runner — it auto-finds `run_fleet.sh` in `.claude/tools/` or the skill's `reference/` (override with `RUN_FLEET`). All scripts print usage with `-h`. The skill's bundled example (`.claude/skills/coding-campaign/examples/bytecode-vm/`) ships golden substrate and runs as-is.

## Conventions
- The fixed contract is load-bearing: candidates implement exactly one `pub fn` (`assemble`/`run`/`optimize`) against the `Op` enum in `prelude.rs`; they never redefine `Op`. Composition wraps each stage in its own module so imports cannot collide.
- Selection is fitness first (must be 16/16 — semantics preserved), then reduction (Stage C only), then clarity and brevity. A correct non-shrinking optimizer beats an incorrect shrinking one.
- The head re-tests every finalist in its own harness before crowning a global winner; agent self-reports are not trusted blind.
- Candidate sources are frozen records — do not edit them. Regenerate a stage rather than hand-patch a candidate.
