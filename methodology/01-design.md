# Design

## The target
A stack-machine bytecode VM decomposes cleanly into three components with fixed interfaces, which is what makes the components independently testable and freely composable:

- Assembler: `pub fn assemble(src: &str) -> Result<Vec<Op>, String>` — text assembly (labels, jumps, comments) to a `Vec<Op>`.
- Executor: `pub fn run(program: &[Op]) -> Result<Vec<i64>, String>` — a stack VM returning the sequence of printed values.
- Optimizer: `pub fn optimize(program: &[Op]) -> Vec<Op>` — a semantics-preserving peephole pass that must shrink the program and rewrite absolute jump targets.

The `Op` enum (`prelude.rs`) is the fixed contract. Every candidate implements exactly one of the three functions against it and never redefines it. Fixing the ISA is what lets a stage-B winner run programs from a stage-A winner, and lets any three winners compose into one file.

## Fitness
`testmain.rs` is a battery of sixteen programs: arithmetic (each of which is a constant-fold target), stack ops, multi-output, two label/jump loops (the trap for optimizer jump-fixup), and two error cases. Each case asserts either an exact output vector or that an error is raised. The harness catches per-case panics (a bad candidate fails that case, not the run) and a five-second outer timeout kills infinite loops. It prints `FITNESS pass=X/16 in_ops=.. out_ops=.. reduction=..`.

A stage is scored in isolation by composing the candidate with golden versions of the other two stages (`golden/`). The golden optimizer is the identity, so it never interferes when scoring the assembler or executor. To score an optimizer, the two jump-carrying battery programs make target-fixup mandatory: a folder that shrinks straight-line code but leaves stale `Jmp`/`Jz` indices scores below 16/16. This was validated before any agent ran — the golden trio scores 16/16 reduction=0, a reference folder scores 16/16 reduction=21, and that same folder with its fixup deleted drops to 15/16.

## Composition
`compose_and_test.sh` concatenates `prelude.rs`, then each stage source wrapped in its own module (`mod a`/`mod b`/`mod c`, each with `use super::Op::{self, *}`), then `testmain.rs`, and compiles the whole as one binary. The module wrapping is load-bearing: two independently generated winners will both `use std::collections::HashMap`, and at crate root that is a duplicate-import error; inside separate modules it is fine. A candidate that redefines `Op` or forgets `pub` simply fails to compile and is culled — evolution handles non-conformance rather than the harness policing it.

## The generation loop
One agent, one stage: `run_stage.sh` commissions a fleet of fifteen claudex candidates (five gpt-5.6-sol at high effort, ten gpt-5.6-luna at xhigh), strips fences, tests each against the frozen substrate, ranks by fitness then reduction then brevity, and copies a provisional champion. The Claude supervisor that invoked it then reads `results.tsv`, inspects the top-fitness candidates, and may override the champion on engineering quality.

Three generations, propagated:
1. Stage A. Substrate B and C are goldens. Six agents each return a champion assembler; the head re-tests all six and freezes the global winner as `substrate/A.rs`.
2. Stage B. Substrate A is the winning assembler, C is golden. Same fan-out; global winner freezes as `substrate/B.rs`.
3. Stage C. Substrate A and B are the winning assembler and executor; reduction now decides among semantics-preserving candidates. Global winner freezes as `substrate/C.rs`.

Finally the three winners compose into `artifact/final_vm.rs` with a convenience `execute` and a demo `main`, compiled and run to confirm the pipeline end to end.

## Why blind, and why re-test
Agents never saw each other. Selection happened only at the head, and only the anonymized winning source propagated forward — no agent was told whose component it was building on. The head re-tested every finalist in its own harness rather than trusting the agents' self-reported fitness, which caught nothing dishonest here but is the correct posture for a tournament whose whole point is an objective metric.
