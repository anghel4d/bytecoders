# Recommendations for the next run

Two changes for future contests, both grounded in what the first one showed. The harness and the packaged skill already implement them; the historical docs (01-03) describe the first contest as it actually ran.

## Fleet sizing belongs to the supervisor, with attrition headroom

The first contest dictated every fleet from the head: 5 sol/high + 10 luna/xhigh, identical for all six supervisors and all three stages. That was wrong twice over.

The counts were too tight. Attrition was real and one-sided — every compile failure in the contest was Luna (the single Stage-A failure, all Stage-C build fails), so the discriminating stage ran effective fields of 12-14 rather than the nominal 15. One extra sol and two extra luna per fleet — 6+12, now the harness default — restore a full-strength effective field at essentially no cost, because a non-compiling candidate spends one generation call and nothing downstream. Overprovisioning beats re-running.

The rigidity sat at the wrong level. Which mix suits a stage is a stage-local judgment: the contest's own results say sol-heavy for tight, self-contained components (sol took 17 of 18 local contests), and a stronger luna share only on the hardest, most open-ended stage — the single place luna competed. The head cannot see that from above before the fact; the supervisor reading its own results table can. So the head now hands each supervisor a budget envelope, and the supervisor chooses its mix (`SOL`/`LUNA`, or a full `FLEET` list), reporting what it chose and why. Selection stays central; provisioning goes local.

To reproduce the original contest exactly, run with `SOL=5 LUNA=10`.

## A 250k-token soft cap on every bottom-level task

Every generation task in the first contest ran under 250k tokens, but only as a happy accident of the VM's clean decomposition. Make it a stated constraint: at decomposition time, forecast each bottom-level subagent's task — prompt, fixed contract, expected candidate size, and the backend's reasoning — and keep the forecast at or under 250k tokens. A component that projects above the cap gets split, or its contract thinned, before anything is commissioned.

The cap is soft: an individual run drifting over it is a signal to re-scope that task, not a failure of the run. But a task *designed* over the cap is a decomposition bug — it means a "component" is really two, and the contest will pay for the ambiguity in noisier fields and weaker selection, not just in tokens.
