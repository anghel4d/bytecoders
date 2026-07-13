# bytecoders

A stack-machine bytecode VM in Rust, grown by a three-generation evolutionary contest across two AI providers. Claude Opus orchestrated and judged; a separate cheap backend (gpt-5.6 `sol`/`luna` via `claudex`) wrote every candidate. The bulk generation cost ~1% of an OpenAI plan; the Claude plan paid only to orchestrate and select.

## Result
The assembled winner at `artifact/final_vm.rs` compiles clean, passes the full 16-program fitness battery, and folds 23 instructions across it. Live demo:

```
$ rustc --edition 2021 -O artifact/final_vm.rs -o /tmp/vm && /tmp/vm
assembled : 15 instructions
optimized : 11 instructions (4 folded away)
output    : [20, 3, 2, 1]
```

`push 2; push 3; add; push 4; mul` folded to `push 20`, and the countdown loop's jump targets survived the fold (it still prints 3, 2, 1).

## How it was built
Three components, each built in its own generation, the winner frozen as substrate for the next:

| Gen | Component | Global winner | Field |
|-----|-----------|---------------|-------|
| A | assembler `assemble(&str) -> Result<Vec<Op>, String>` | agent 2, sol/high, 93L | 5 fields perfect, 1x 14/15 |
| B | executor `run(&[Op]) -> Result<Vec<i64>, String>` | agent 6, sol/high, 91L | all 90 candidates 16/16 |
| C | optimizer `optimize(&[Op]) -> Vec<Op>` | agent 2, sol/high, 117L, reduction 23 | 1-3 build fails/field, a 15/16 semantics-breaker (the jump trap) |

Every global winner was Sol-high. Luna won one local contest of eighteen (agent 3, the optimizer) — by being the most cautious implementation in the field. See `methodology/`.

## Layout
The directory tree is the agentic hierarchy: `orchestrator/` (head) contains `supervisors/agent-N/` (six blind Opus agents), each containing `stage-{A,B,C}/candidates/` (fifteen claudex implementations) and `champions/`. `harness/` holds the shared tooling; `.claude/skills/` packages the method for reuse. Start with `CLAUDE.md`.
