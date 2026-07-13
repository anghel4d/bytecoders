# Economics

## What it cost
The whole contest — 270 code generations across three generations, plus a full evolutionary tournament with adversarial fitness selection — cost about 1% of an OpenAI plan and a sliver of a Claude plan. Every generation task ran under 250k context. The Claude plan paid only for orchestration and selection.

## Why
Two mechanisms, one deliberate.

The bulk generation never touched the Claude plan. `claudex` is wired through a local `cli-proxy-api` to the gpt-5.6 `sol`/`luna` backend — its traffic goes to that provider's meter, not to Anthropic. The 270 heaviest jobs, which together wrote on the order of forty thousand lines of candidate Rust, billed entirely to the OpenAI side. That is the 1%.

The orchestration was context-frugal by construction. The fan-out's raw output — 270 candidate files, every compile log, all fitness runs — lived in bash and on disk, never in a billed context window. Each supervisor agent read only a results table and the two or three tied finalists; the head re-tested six files per stage and read a couple. The forty thousand lines of generated code were never loaded into any Opus context. That is why the eighteen supervisor invocations came to roughly 30-70k tokens each rather than hundreds of thousands, and why the whole Claude-side cost is a rounding error even against a long orchestration session with prompt caching.

## The technique, stated generally
Put the metered-expensive model where judgment lives — decomposition, orchestration, adversarial selection, verification — and push high-volume generation to a cheaper backend. Then keep the generation's bulk output out of the expensive model's context: let scripts and disk hold the candidates and the test logs, and feed the expensive model only summaries, rankings, and the handful of finalists it must actually judge. The expensive model's spend then scales with the number of decisions, not the volume of generated text.

## What that bought
Ninety independent implementations per component, scored by an objective battery, with the best propagated across three generations and re-verified at the head — yielding a working, tested artifact whose every part survived a fifteen-way contest. The judgment layer stayed thin and cheap; the generation layer stayed cheap on a different meter; neither carried the other's weight.
