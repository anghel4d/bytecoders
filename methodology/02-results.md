# Results

## Per generation

### A — assembler
Six agents, ninety candidates. Five fields were perfect (15/15 compiled and 16/16); one had a single Luna compile failure (14/15). All six agents crowned a Sol-high assembler. Sol-high implementations ran 93-140 lines; Luna-xhigh ran 125-205 and often dropped line numbers from error messages. Global winner: agent 2's candidate (sol/high, 93L) — the shortest that still emits line-numbered diagnostics, with a centralized trailing-token check no opcode arm can forget. Two agents overrode the mechanical fewest-lines pick on quality grounds.

### B — executor
Flawless generation: all ninety candidates compiled and scored 16/16. All six winners Sol-high, tightly clustered at 83-100 lines (Luna 93-138). The recurring discriminator was a hazard the battery cannot see: several candidates hid the five arithmetic ops behind an inner re-match with a dead `unreachable!()` branch in the hot loop, and two agents explicitly rejected the shortest candidate for exactly that latent panic path. Global winner: agent 6's candidate (sol/high, 91L) — fully exhaustive per-op dispatch, `.get()`-bounded program counter, underflow checks on every pop, and `wrapping_*` arithmetic that dodges the `i64::MIN / -1` and `-i64::MIN` overflow panics, with zero panic paths.

### C — optimizer
The discriminating stage. Per field, 12-14 of 15 hit 16/16, 1-3 failed to compile, and one candidate scored 15/16 — a folder that broke jump semantics, the fixup trap firing on a real submission. Reduction ranged 19-23 among the passing candidates, and all six agents' local winners tied at the ceiling of 23. Five of the six finalists were Sol-high; one was Luna (agent 3). Global winner: agent 2's candidate (sol/high, 117L) — it makes the "never fold across a jump target" invariant structural by splitting the program into basic blocks at every jump destination and folding only within a block, in a single streaming pass, rather than relying on distributed per-window guards.

## The model finding
Every one of the six global winners was Sol-high. Across eighteen local contests, Luna-xhigh won exactly one (agent 3, the optimizer). Sol-high produced consistently leaner code at equal or better correctness, and Luna accounted for essentially all the compile failures — the only Stage-A failure and the Stage-C build failures were Luna. On tightly-scoped, self-contained algorithmic components at high/xhigh effort, Sol was the stronger generator; Luna became competitive only on the hardest stage.

## Luna's single win, in detail
Luna won by being the most cautious implementation in its field, not the cleverest. Its optimizer (agent 3, 118L) is a window-folder with two whole-pass bail-outs: if any jump target is out of range, or if after folding a jump would point into a removed region, it abandons optimization and returns the program verbatim. It also refuses to begin a fold at any index that is itself a jump target, which the aggressive Sol version (agent 4, 109L) permits and proves safe by a balanced-sequence argument. The structural Sol winner (agent 2, 117L) makes the unsafe case impossible by construction instead.

Consequences: on this battery all three tied at reduction 23, because no foldable window happens to start on a jump label, so Luna's caution cost nothing the tests could measure. On an adversarial program it would under-fold, and one malformed jump anywhere disables all of its optimization rather than a local region. Luna also carries a redundant defensive layer — because it never folds at a target, every target always survives, so its second bail-out is effectively dead code. The extra caution is also why it is longer than the aggressive Sol version, consistent with the whole contest: Luna verbose and defensive, Sol lean and either proven-aggressive or structurally safe.
