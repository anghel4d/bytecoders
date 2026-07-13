# Bytecode VM evolutionary contest — head orchestrator ledger

6 persistent Opus supervisor agents. Each runs run_stage.sh <STAGE> <N>, commissioning 5 Sol-high + 10 Luna-xhigh claudex per step. Blind to each other. 3 steps; winner propagates as fixed substrate.

## Agent IDs (for SendMessage continuation)
- 1 -> a3afe26493bb641e0
- 2 -> a42dd973cc9edcdab
- 3 -> ad96f5862057315db
- 4 -> abac24aae203c70af
- 5 -> a53e7881088de20cb
- 6 -> a1d20880e548ffe8b

## Contract
- Op ISA fixed in prelude.rs. Stages: A=assemble, B=run, C=optimize. Compose = prelude + mod a{A}+mod b{B}+mod c{C} + testmain. 16-program battery. Golden trio = 16/16 reduction 0; ref optimizer = 16/16 reduction 21.
- Substrate dir holds current fixed components: substrate/{A,B,C}.rs
- Each agent copies its local champion to agents/<N>/winner_<STAGE>.rs

## Plan
- Step A (assembler): substrate B,C = goldens. -> collect 6 winner_A, head re-tests, pick global A -> substrate/A.rs
- Step B (VM): substrate A = winner A, C = goldenC. -> pick global B -> substrate/B.rs
- Step C (optimizer): substrate A,B = winners. -> pick global C -> assemble final + full battery + write final vm.rs

## Status
- [x] Step A: 6 agents, all Sol-high winners. Fields near-perfect (5x 15/15, 1x 14/15). Winners 93-114L.
- [x] Step A global winner = agent 2's cand_02 (Sol-high, 93L, line-numbered diagnostics) -> substrate/A.rs (verified 16/16)
- [x] Step B: all 6 Sol-high winners; ALL 90 VM candidates compiled+16/16 (flawless). Winners 83-98L.
- [x] Step B global winner = agent 6's cand_03 (Sol-high, 91L, exhaustive dispatch, zero panic paths) -> substrate/B.rs (verified 16/16; winnerA+winnerB+refOptC = 16/16 red21). NOTE agent4's 83L had an unreachable!() smell, not picked.
- [x] Step C: discriminating stage. Per field 12-14/15 at 16/16, 1-3 build fails, reduction 19-23. Trap fired (a 15/16 semantics-breaker in agent2 field). 5/6 finalists Sol, 1 Luna (agent3 cand08). All 6 finalists 16/16 reduction=23.
- [x] Step C global winner = agent 2's cand_04 (Sol-high, 117L, basic-block fold, reduction 23) -> substrate/C.rs
- [x] Final assembly: winnerA+winnerB+winnerC = 16/16 reduction=23. final_vm.rs (366L) compiles + runs: demo [20,3,2,1], 4 folded. DONE.

## RESULT: every global winner Sol-high. Luna won a local contest once (agent3, step C). Sol-high leaner + equally/more correct across all 3 self-contained components.

## Step B winners (per agent)
1:cand02/94 2:cand01/94 3:cand01/98 4:cand03/83(unreachable!) 5:cand04/98 6:cand03/91 WINNER  (all sol-high)
Finding: Step B flawless (0 compile fails across 90). Sol-high leaner (83-100L) vs Luna (93-138L). 2 agents rejected shortest for in-loop unreachable!() panic branch.

## Step A winners (per agent)
1:cand02 sol 99L | 2:cand02 sol 93L WINNER | 3:cand03 sol 114L | 4:cand01 sol 111L | 5:cand02 sol 98L | 6:cand01 sol 94L
Finding: all 6 agents preferred Sol-high (leaner 93-140L, line-numbered errors) over Luna-xhigh (125-205L, often dropped line numbers). Luna had the only compile failure (agent 3 cand07).
