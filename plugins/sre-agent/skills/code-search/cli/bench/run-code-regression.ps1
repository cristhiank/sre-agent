<#
.SYNOPSIS
    Definition-ranking regression gate for srch.
.DESCRIPTION
    Reads a CSV of cases (query,expect,max_rank), runs each query with the live
    ranker (--usages forces the scan so the ctags fast path cannot mask a ranking
    regression), and asserts the first result matching `expect` ranks at or above
    `max_rank`. Exits non-zero on any failure.
    Cases and corpus are parameters; the harness knows nothing about either.
.EXAMPLE
    .\run-code-regression.ps1 -Exe ..\bin\srch.exe -Root C:\src\repo -CasesFile .\cases-sample.csv
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory)] [string] $Exe,
    [Parameter(Mandatory)] [string] $Root,
    [Parameter(Mandatory)] [string] $CasesFile,
    [int] $Limit = 20,
    [string[]] $ExtraArgs = @('--usages'),
    [string] $OutDir
)

$ErrorActionPreference = 'Stop'

function Resolve-FullPath {
    param([string] $Path)
    if ([IO.Path]::IsPathRooted($Path)) { return [IO.Path]::GetFullPath($Path) }
    [IO.Path]::GetFullPath((Join-Path (Get-Location) $Path))
}

$exePath = Resolve-FullPath $Exe
if (-not (Test-Path $exePath)) { throw "binary not found: $exePath" }
$root = [IO.Path]::GetFullPath($Root)
if (-not (Test-Path $root)) { throw "root not found: $root" }
$cases = @(Import-Csv -LiteralPath $CasesFile)
if ($cases.Count -eq 0) { throw "no cases in $CasesFile" }
if (-not $OutDir) { $OutDir = Join-Path $PSScriptRoot 'out' }
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

$rows = New-Object System.Collections.Generic.List[object]
$failures = 0
foreach ($c in $cases) {
    $argv = @($c.query, '-p', $root, '--json', '-n', "$Limit") + $ExtraArgs
    $sw = [Diagnostics.Stopwatch]::StartNew()
    $raw = & $exePath @argv 2>$null
    $sw.Stop()
    $json = if ($raw) { ($raw -join "`n") | ConvertFrom-Json } else { $null }
    $paths = @()
    if ($json -and $json.results) { $paths = @($json.results | ForEach-Object { $_.path }) }
    $rank = 0
    for ($i = 0; $i -lt $paths.Count; $i++) {
        if ($paths[$i] -match $c.expect) { $rank = $i + 1; break }
    }
    $max = if ($c.max_rank) { [int] $c.max_rank } else { 1 }
    $ok = ($rank -ge 1 -and $rank -le $max)
    if (-not $ok) { $failures++ }
    $rows.Add([pscustomobject]@{
            query    = $c.query
            expect   = $c.expect
            rank     = $rank
            max_rank = $max
            pass     = $ok
            top1     = if ($paths.Count) { Split-Path $paths[0] -Leaf } else { '' }
            wall_ms  = [math]::Round($sw.Elapsed.TotalMilliseconds, 1)
            scan_ms  = if ($json) { $json.elapsed_ms } else { $null }
        })
}

$rows | Format-Table -AutoSize query, rank, max_rank, pass, top1, wall_ms | Out-String | Write-Output
$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$rows | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $OutDir "code-regression-$stamp.json") -Encoding UTF8
$avg = [math]::Round(($rows | Measure-Object wall_ms -Average).Average, 1)
Write-Output "cases=$($rows.Count) failures=$failures wall_avg=${avg}ms"
if ($failures -gt 0) { exit 1 }
