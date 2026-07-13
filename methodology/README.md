# Methodology

Four documents cover how bytecoders was built, what it showed, and what the next run should do differently.

- `01-design.md` — the machinery: the fixed ISA contract, the golden references, the fitness battery, module-isolated composition, and the three-generation propagation loop.
- `02-results.md` — the per-generation outcomes, the model finding (Sol dominance; Luna's single win), and the concrete diff between Luna's and Sol's optimizers.
- `03-economics.md` — why 270 generation calls plus a full evolutionary tournament cost ~1% of one plan and a sliver of the other, and the reusable technique behind it.
- `04-recommendations.md` — changes for the next run, drawn from this one: supervisor-owned fleet sizing with attrition headroom (6 sol + 12 luna default, mix at the supervisor's discretion), and an explicit 250k-token soft cap on every bottom-level generation task.

The one-paragraph version: a head orchestrator (Opus) ran three generations of a blind six-agent contest. Each agent, in each generation, commissioned fifteen implementations of one VM component from a cheap generation backend, scored them against an objective battery, and returned a local champion. The head re-tested the six finalists, crowned a global winner, froze it as substrate, and moved to the next component. Selection was fitness first, then optimization strength, then code quality. The product is a working, tested bytecode VM whose three components were each chosen from ninety candidates.
