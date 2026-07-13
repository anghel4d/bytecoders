---
name: coding-campaign
description: A flexible, agentic, hierarchically orchestrated coder system — the GAN coder system. Use when the user calls for a coding campaign, when there is an incredibly large task with a lot of code churn ahead, or when a claudex fleet is mentioned. Spawns Squad Leaders (SLs) which each invokes its own cluster of claudex instances (gpt-5.6 sol/luna); SLs evaluate the implementations and synthesize the best one before reporting to the CMDR (you are the Commander), who picks winners in a RNN/GAN-style evolutionary contest.
---

# The GAN coder system

```
N *Claude* Squad Leader subagents that spawn and supervise claudex surgical strike groups, forecasted <200k tokens max each instance, to complete a complex multi-step task. 

What I want to see:
- N (eg 4 to 10) agents actually popping up in my claude worktree, supervised by you as the head orchestrator.
- Each agent invokes its own 4 Sol (high) and 14 Luna (xhigh) instances, and does so in separate invocations over the course of k steps.

Given a task of n to k total steps, foreach agent:
-- Step 1: Spawn all claudex instances in one invocation. Await for them to all finish. Collect and consolidate. First component of the current phase, report back pick winner.
-- Step n: Ditto. nth component of the current phase, report back, report winner.
-- Step k: Ditto. Final component and polishing round. Report then definitively implement winner.

Subagents are never told what the others are doing. Winners are picked in a RNN/GAN-style evolutionary contest.
```

## Layers of the Architecture

user → **commander (CMDR)** → **squadron leaders (SLs)** → **sol/luna clusters**

### Commander (CMDR)

The commander is the agent the user interfaces with, most likely *you*, the one invoking this skill. You are given a set of high-level strategic objectives by the user, and it is your job to figure out your operational priorities as you task a variety of SLs to do the job through each phase. You periodically tell the user what's going on, namely when a batch of SLs finish their jobs. When the campaign is done, you put together an Operational Report and present it to the user.

### Squad Leaders (SLs)

A squad leader is spawned as a subagent, designated to supervise and see through a tactical slice of the overall strategic mission. They do this by overseeing a cluster of agents and putting together their best work.
- Either a Fable or Opus 4.8 instance depending on task difficulty (CMDR's judgement).
- Cluster task shapes: 
-- **/work**: coding tasks under 200k tokens, commenting + documentation tone, any uses of the tersify skill
-- **/review** adversarial review (sol high or xhigh)
- Cluster mix is each SL's leeway (defaults: 4 sol/high + 14 luna/xhigh).
- Soft cap: forecast a maximum 200k tokens per subagent task.

### Individual Units (IUs)

Individual units are the agents enlisted to perform the dirty work through claudex invocations. There are many of them under each SL instance. Their prompts, model selection, and effort levels are left entirely to the discretion of supervising SL. 

## The shell

`claudex` on PATH (confirm: `claudex -p --effort low "reply OK"`). It does OK with extremely constrained scopes, context under 200k tokens, and it is very very fast.

One squad, one step:

```
PROMPT_FILE=prompts/parser.txt \
FITNESS_CMD='bash fit.sh "$1"' \
WORK=/contest/squad-1 NAME=parser bash reference/run_fleet.sh
```

Windows: `reference/run_fleet.ps1` (PowerShell 7+), same env-var contract; the candidate path arrives as `$args[0]` instead of `$1`. All knobs (`SOL`/`LUNA`, models, efforts, `FLEET`, `PARALLEL`, `CLAUDEX`, `EXT`) and the `FITNESS_CMD` output format are printed by each runner's `-h` (`bash reference/run_fleet.sh -h`, `pwsh reference/run_fleet.ps1 -h`). Both are also installed at `.claude/tools/`.
