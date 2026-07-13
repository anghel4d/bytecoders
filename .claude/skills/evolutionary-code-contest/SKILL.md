---
name: evolutionary-code-contest
description: Run a multi-model, multi-generation evolutionary contest to build a staged, objectively-testable piece of code. Use when a task decomposes into a few components with fixed interfaces (parser -> compiler -> runtime; assemble -> run -> optimize; table -> insert -> resize), each has an objective pass/fail fitness, and you want the strongest implementation per component chosen from many candidates. Claude orchestrates and judges; a cheap high-volume backend (claudex / gpt-5.6 sol/luna) writes every candidate. Bundles the fitness harness and per-generation runner.
---

# Evolutionary code contest

Grow a program by running blind supervisor agents that each commission many implementations of one component from a cheap generation backend, score them against an objective battery, and propagate the winner across generations. The expensive model spends tokens on decomposition, orchestration, and selection only; the bulk generation bills to the cheap backend and its output never enters billed context.

The worked example that ships with this skill is a stack-machine bytecode VM (assemble -> run -> optimize). Adapt the contract, prompts, and battery to your target.

## When to use
Reach for this when all three hold: the task splits into a small number of components with fixed interfaces that compose; each component has an objective fitness (a test battery, a compile-and-run check, a metric to maximize); and quality varies enough across attempts that picking the best of many beats one careful attempt. If the task is one indivisible blob, or has no objective fitness, this is the wrong tool.

## Prerequisites
- `claudex` on PATH (the CLI routing to the cheap backend). Confirm: `claudex -p --effort low "reply OK"`.
- The target toolchain for compile-and-run fitness (here `rustc`).
- This skill's `harness/` dir, copied to a writable contest root.

## The five pieces you define
1. `prelude.rs` — the fixed shared contract (types/ISA) every candidate targets and never redefines.
2. `golden/` — a correct reference implementation of each stage, used to score one stage in isolation while the others are held fixed. Include one reference that maximizes the metric (a real optimizer) to prove the metric is discriminating.
3. `testmain.rs` — the fitness battery. Cover the happy path, the edge that punishes the subtle bug (here: jump-target fixup), and error cases. Catch per-case panics; rely on an outer timeout for infinite loops. Print one `FITNESS ...` line.
4. `prompts/prompt_<SLOT>.txt` — one per component. State the exact signature, forbid redefining the contract, demand `pub fn`, demand raw code only (no fences, no prose).
5. `compose_and_test.sh` — composes prelude + the three stage sources (each wrapped in its own module to isolate imports) + testmain, compiles, runs, emits fitness. Ships ready; only change it if your composition differs.

## Validate the harness before spawning anything
This is not optional — a wrong battery makes every downstream selection garbage. From the harness dir as `CONTEST_ROOT`:
```
export CONTEST_ROOT=harness
bash harness/compose_and_test.sh harness/golden/goldenA.rs harness/golden/goldenB.rs harness/golden/goldenC.rs  # expect full pass, zero metric
bash harness/compose_and_test.sh harness/golden/goldenA.rs harness/golden/goldenB.rs harness/golden/refOpt.rs    # expect full pass, metric > 0
# then break the reference's target fixup and confirm it drops below full pass — the trap must actually fire
```

## Run a generation
Per agent, per stage:
```
CONTEST_ROOT=harness WORK=/contest/agent-1 bash harness/run_stage.sh A
```
It commissions the fleet (default 5x sol/high + 10x luna/xhigh; override with `FLEET`), tests each candidate against `harness/substrate/{A,B,C}.rs`, ranks by fitness then metric then brevity, and writes a provisional champion to `$WORK/champion_A.rs`. Tune `PARALLEL` (default 3) to bound concurrent claudex per agent.

## Orchestrate (the Claude layer)
1. Spawn N blind supervisor agents (Agent tool, one message, model opus). Each runs `run_stage.sh` for the current stage, reads its `results.tsv`, and picks a champion — confirming the provisional or overriding it on quality among top-fitness candidates. Tell agents nothing about each other or the contest.
2. Collect the N champions. Re-test all N in your own harness — do not trust self-reported fitness. Crown the global winner (fitness first; then the metric; then clarity/brevity). Freeze it as `harness/substrate/<SLOT>.rs`.
3. Continue the same agents to the next stage via SendMessage (persistent context) or spawn fresh. The next stage's substrate is the frozen winners; agents build on them without being told whose they are.
4. After the last stage, compose the winners into a standalone artifact, compile, run, confirm.

## Cost discipline
Keep each generation task under the cheap backend's economical context (here 250k). Never load candidate bodies or compile logs into the orchestrator or the supervisors — let scripts and disk hold them and feed the expensive model only tables, rankings, and the few finalists it must judge. Log any coverage the harness drops (timeouts, culled non-compilers) so a silent field failure never reads as success.

## Pitfalls learned
- Emit fitness to a real temp path, not `/dev/null` — some compilers cannot create metadata there.
- Export every variable a parallel worker reads, or its writes vanish into an empty path.
- Composition must isolate each stage's imports (module wrapping) or two real winners collide on a shared `use`.
- The battery must contain the case that punishes the plausible-but-wrong shortcut, or the contest rewards it.
