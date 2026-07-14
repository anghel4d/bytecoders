#Requires -Version 7
$ErrorActionPreference = 'Stop'

$here = $PSScriptRoot
$runner = Join-Path (Split-Path $here -Parent) 'run_fleet.ps1'
$work = Join-Path ([System.IO.Path]::GetTempPath()) ("fleet-test-" + [guid]::NewGuid())
try {
  $env:PROMPT_FILE = Join-Path $here 'prompt.txt'
  $env:FITNESS_CMD = "if ((Get-Content -Raw -LiteralPath `$args[0]).Trim() -eq 'PASS') { 'FITNESS pass=1/1 metric=1' } else { 'FITNESS pass=0/1 metric=0' }"
  $env:WORK = $work
  $env:NAME = 'self'
  $env:EXT = 'txt'
  $env:FLEET = "01 fast-pass high`n02 fast-fail high`n03 slow-pass high"
  $env:SOL = '1'
  $env:LUNA = '2'
  $env:ALLOW_SMALLER_FLEET = '1'
  $env:CLAUDEX = Join-Path $here 'fake_claudex.cmd'
  $env:ROUND_TIMEOUT = '20'
  $env:FITNESS_TIMEOUT = '5'
  $env:CAMPAIGN_TEST_MODE = '1'
  $env:TEST_SOL_PASS_GRACE = '3'
  $env:TEST_SOL_PASS_WARNING = '2'
  & $runner

  $results = Get-Content -LiteralPath (Join-Path $work 'self\results.tsv')
  if (-not ($results -match '^01\tpass=1/1\t')) { throw 'passing result missing' }
  if (-not ($results -match '^02\tpass=0/1\t')) { throw 'failing result missing' }
  if (-not ($results -match '^03\tCULLED\t')) { throw 'culled result missing' }
  if ((Get-Content -Raw -LiteralPath (Join-Path $work 'champion_self.txt')).Trim() -ne 'PASS') { throw 'wrong champion' }
  if (-not (Test-Path -LiteralPath (Join-Path $work 'self\first_sol_pass'))) { throw 'passing Sol did not arm the deadline' }
  foreach ($idx in '01', '02', '03') {
    if (-not (Test-Path -LiteralPath (Join-Path $work "self\warning_$idx.txt"))) { throw "IU $idx did not receive the group warning" }
  }
  if (-not ((Get-Content -LiteralPath (Join-Path $work 'self\group_events.tsv')) -match 'SOL_PASS_ONE_MINUTE_WARNING')) { throw 'SL-visible warning event missing' }

  $env:WORK = Join-Path $work 'luna-trigger'
  $env:FLEET = "01 fast-fail high`n02 fast-pass high"
  $env:SOL = '1'
  $env:LUNA = '1'
  & $runner
  if (Test-Path -LiteralPath (Join-Path $env:WORK 'self\first_sol_pass')) { throw 'passing Luna armed the Sol-only deadline' }
  if (Test-Path -LiteralPath (Join-Path $env:WORK 'self\group_events.tsv')) { throw 'passing Luna emitted a deadline warning' }

  $env:WORK = Join-Path $work 'fitness'
  $env:FLEET = "01 fast-pass high`n02 fast-fail high"
  $env:SOL = '1'
  $env:LUNA = '1'
  $env:FITNESS_CMD = "Start-Sleep -Seconds 8; 'FITNESS pass=1/1 metric=1'"
  $env:FITNESS_TIMEOUT = '1'
  $rejected = $false
  try { & $runner *> $null } catch { $rejected = $true }
  if (-not $rejected) { throw 'timed-out fitness command produced a champion' }
  if (-not ((Get-Content -LiteralPath (Join-Path $env:WORK 'self\results.tsv')) -match '^01\tTIMEOUT\t')) { throw 'fitness timeout was not recorded' }

  $env:WORK = $work
  $env:FITNESS_CMD = 'true'
  $env:FITNESS_TIMEOUT = '5'
  $env:FLEET = "01 fast-pass high`n02 fast-fail high`n03 slow-pass high"
  $env:SOL = '1'
  $env:LUNA = '2'
  $env:ALLOW_SMALLER_FLEET = '0'
  $rejected = $false
  try { & $runner *> $null } catch { $rejected = $true }
  if (-not $rejected) { throw 'unauthorized smaller fleet was accepted' }

  $env:FLEET = (1..11 | ForEach-Object { '{0:d2} fast-pass high' -f $_ }) -join "`n"
  $env:SOL = '4'
  $env:LUNA = '7'
  $env:ALLOW_SMALLER_FLEET = '1'
  $rejected = $false
  try { & $runner *> $null } catch { $rejected = $true }
  if (-not $rejected) { throw 'eleven-unit fleet was accepted' }

  $env:FLEET = (1..10 | ForEach-Object { '{0:d2} fast-pass high' -f $_ }) -join "`n"
  $env:SOL = '5'
  $env:LUNA = '5'
  $env:ALLOW_SMALLER_FLEET = '0'
  $rejected = $false
  try { & $runner *> $null } catch { $rejected = $true }
  if (-not $rejected) { throw 'non-4/6 ten-unit fleet was accepted' }

  'run_fleet.ps1 self-test passed'
} finally {
  Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction Ignore
}
