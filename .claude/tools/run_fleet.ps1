#!/usr/bin/env pwsh
#Requires -Version 7
# run_fleet.ps1 — PowerShell 7+ port of run_fleet.sh, same env-var contract.
# One squadron, one contest round. Domain-agnostic: PROMPT_FILE says what to generate,
# FITNESS_CMD says how to score it. Commissions a fleet of cheap-backend candidates,
# scores each, ranks, and copies a provisional champion for the squadron leader to
# review (results.tsv) and possibly override on engineering quality.
$ErrorActionPreference = 'Stop'

function Show-Usage { @'
usage:  $env:PROMPT_FILE = 'prompts\parser.txt'
        $env:FITNESS_CMD = '& .\fit.ps1 $args[0]'   # candidate path arrives as $args[0]
        $env:WORK = 'C:\contest\squad-1'; $env:NAME = 'parser'
        pwsh run_fleet.ps1

required (env)
  PROMPT_FILE  generation prompt (exact signature, forbid contract redefinition, raw code only)
  FITNESS_CMD  PowerShell snippet run once per candidate; the candidate path arrives as
               $args[0]. Must print one line: 'FITNESS pass=X/Y metric=N' (metric optional;
               'reduction=' accepted as an alias) or a bare BUILD_FAIL / TIMEOUT / CRASH.
  WORK         output dir: candidates in $WORK\$NAME\, results.tsv beside them,
               champion copied to $WORK\champion_$NAME.$EXT

optional (env)                                             [default]
  NAME / EXT   round label / candidate file extension      [round / rs]
  SOL / LUNA   candidate counts — the squadron leader      [6 / 12]
               owns this mix; size it to the round and to expected attrition
  SOL_MODEL  / SOL_EFFORT                                  [gpt-5.6-sol / high]
  LUNA_MODEL / LUNA_EFFORT                                 [gpt-5.6-luna / xhigh]
  FLEET        newline list of '<idx> <model> <effort>';   [built from SOL/LUNA]
               full override, ignores the knobs above
  PARALLEL     concurrent generations                      [3]
  CLAUDEX      backend CLI                                 [claudex on PATH, else ~/.local/bin/claudex]
'@ }
if ($args[0] -in '-h', '--help') { Show-Usage; exit 0 }

function Req([string]$Name) {
  $v = [Environment]::GetEnvironmentVariable($Name)
  if ([string]::IsNullOrWhiteSpace($v)) { Write-Error "run_fleet.ps1: `$env:$Name is not set (run_fleet.ps1 -h for usage)" }
  $v
}
function Opt([string]$Name, [string]$Default) {
  $v = [Environment]::GetEnvironmentVariable($Name)
  if ([string]::IsNullOrWhiteSpace($v)) { $Default } else { $v }
}

$Work       = Req 'WORK'
$PromptFile = Req 'PROMPT_FILE'
$FitnessCmd = Req 'FITNESS_CMD'
$RoundName  = Opt 'NAME' 'round'
$Ext        = Opt 'EXT'  'rs'
$Parallel   = [int](Opt 'PARALLEL' '3')
$OnPath     = Get-Command claudex -ErrorAction Ignore
$Claudex    = Opt 'CLAUDEX' ($(if ($OnPath) { $OnPath.Source } else { Join-Path $HOME '.local/bin/claudex' }))
if (-not (Get-Command $Claudex -ErrorAction Ignore)) {
  Write-Error "run_fleet.ps1: backend '$Claudex' not found — install claudex or set `$env:CLAUDEX"
}
if (-not (Test-Path $PromptFile)) { Write-Error "run_fleet.ps1: cannot read PROMPT_FILE '$PromptFile'" }

$RoundDir = Join-Path $Work $RoundName
New-Item -ItemType Directory -Force -Path $RoundDir | Out-Null
$Prompt = Get-Content -Raw -Path $PromptFile

$FleetSpec = [Environment]::GetEnvironmentVariable('FLEET')
if ([string]::IsNullOrWhiteSpace($FleetSpec)) {
  $Sol = [int](Opt 'SOL' '6'); $Luna = [int](Opt 'LUNA' '12')
  $SolModel  = Opt 'SOL_MODEL'  'gpt-5.6-sol';  $SolEffort  = Opt 'SOL_EFFORT'  'high'
  $LunaModel = Opt 'LUNA_MODEL' 'gpt-5.6-luna'; $LunaEffort = Opt 'LUNA_EFFORT' 'xhigh'
  $rows = @()
  if ($Sol  -gt 0) { $rows += 1..$Sol  | ForEach-Object { '{0:d2} {1} {2}' -f $_, $SolModel, $SolEffort } }
  if ($Luna -gt 0) { $rows += 1..$Luna | ForEach-Object { '{0:d2} {1} {2}' -f ($Sol + $_), $LunaModel, $LunaEffort } }
  $FleetSpec = $rows -join "`n"
}
$Fleet = @($FleetSpec -split "\r?\n" | Where-Object { $_.Trim() } | ForEach-Object {
  $p = -split $_
  [pscustomobject]@{ Idx = $p[0]; Model = $p[1]; Effort = $p[2] }
})

Write-Host "[$Work $RoundName] commissioning $($Fleet.Count) candidates ..."
$Fleet | ForEach-Object -Parallel {
  $out  = Join-Path $using:RoundDir ('cand_{0}.{1}' -f $_.Idx, $using:Ext)
  $body = & $using:Claudex -p --model $_.Model --effort $_.Effort $using:Prompt 2>$null |
    Where-Object { $_ -notmatch 'connectors are disabled' -and $_ -notmatch '^```' }
  Set-Content -Path $out -Value @($body)
} -ThrottleLimit $Parallel

$results = foreach ($c in Get-ChildItem -Path $RoundDir -Filter "cand_*.$Ext" | Sort-Object Name) {
  try { $fitOut = (& ([scriptblock]::Create($FitnessCmd)) $c.FullName) -join "`n" }
  catch { $fitOut = 'CRASH' }
  $passStr = $null; $passN = 0
  if ($fitOut -match 'pass=(\d+)/(\d+)') { $passStr = $Matches[0]; $passN = [int]$Matches[1] }
  $met = $null
  if ($fitOut -match '(?:metric|reduction)=(\d+)') { $met = [int]$Matches[1] }
  $status = ((($fitOut -split "\r?\n" | Where-Object { $_.Trim() } | Select-Object -First 1) ?? 'NO_OUTPUT') -split '\s+')[0]
  [pscustomobject]@{
    Idx     = $c.BaseName -replace '^cand_', ''
    Fitness = $passStr ?? $status
    Metric  = 'metric=' + ($met ?? 'NA')
    MetricN = $met ?? 0
    Lines   = @(Get-Content -Path $c.FullName).Count
    PassN   = $passN
  }
}
if (-not $results) { Write-Error "[$Work $RoundName] no candidates produced" }
$results | ForEach-Object { "{0}`t{1}`t{2}`t{3}L" -f $_.Idx, $_.Fitness, $_.Metric, $_.Lines } |
  Set-Content -Path (Join-Path $RoundDir 'results.tsv')

# rank: fitness numerator desc, metric desc, fewest lines
$best = $results |
  Sort-Object @{Expression='PassN';Descending=$true}, @{Expression='MetricN';Descending=$true}, @{Expression='Lines'} |
  Select-Object -First 1

Write-Host "[$Work $RoundName] ranked results:"
$results | Sort-Object Idx | ForEach-Object {
  Write-Host ('{0,-4} {1,-14} {2,-12} {3}L' -f $_.Idx, $_.Fitness, $_.Metric, $_.Lines)
}
$champ = Join-Path $Work "champion_$RoundName.$Ext"
Copy-Item (Join-Path $RoundDir "cand_$($best.Idx).$Ext") $champ -Force
Write-Host "PROVISIONAL_WINNER=$($best.Idx)  (champion at $champ)"
