<#
.SYNOPSIS
    A/B ranking benchmark for srch over any corpus.
.DESCRIPTION
    Runs a query list against one or more srch binaries and reports, per query:
      * the rank of the first result whose relative path matches any -TargetPattern
      * the fraction of the top-N slots taken by results matching any -NoisePattern
      * wall-clock and in-process scan latency
    Corpus, queries and patterns are all parameters: the harness carries no
    knowledge of any particular corpus, schema or query set.
.EXAMPLE
    .\run-bench.ps1 -Exe 'old=C:\tmp\srch-old.exe','new=..\bin\srch.exe' -Root C:\corpus `
        -QueryFile .\queries-sample.txt -TargetPattern '^docs[\\/]' -NoisePattern '\.log$'
#>
[CmdletBinding()]
param(
    # label=path pairs, e.g. 'old=C:\tmp\srch-old.exe','new=..\bin\srch.exe'
    [Parameter(Mandatory)] [string[]] $Exe,
    [Parameter(Mandatory)] [string] $Root,
    [Parameter(Mandatory)] [string] $QueryFile,
    [string[]] $TargetPattern = @(),
    [string[]] $NoisePattern = @(),
    [int] $Limit = 20,
    [int] $Repeat = 3,
    [string[]] $ExtraArgs = @(),
    [string] $OutDir
)

$ErrorActionPreference = 'Stop'

function Resolve-FullPath {
    param([string] $Path)
    if ([IO.Path]::IsPathRooted($Path)) { return [IO.Path]::GetFullPath($Path) }
    [IO.Path]::GetFullPath((Join-Path (Get-Location) $Path))
}

function Resolve-Binaries {
    param([string[]] $Spec)
    $out = [ordered]@{}
    foreach ($s in $Spec) {
        $i = $s.IndexOf('=')
        if ($i -lt 1) { throw "-Exe entries must be 'label=path' (got '$s')" }
        $label = $s.Substring(0, $i)
        $path = Resolve-FullPath $s.Substring($i + 1)
        if (-not (Test-Path $path)) { throw "binary not found for '$label': $path" }
        $out[$label] = $path
    }
    $out
}

function Get-RelativePath {
    param([string] $Full, [string] $Root)
    $r = $Root.TrimEnd('\', '/')
    if ($Full.StartsWith($r, [StringComparison]::OrdinalIgnoreCase)) {
        return $Full.Substring($r.Length).TrimStart('\', '/')
    }
    $Full
}

function Test-AnyPattern {
    param([string] $Value, [string[]] $Patterns)
    foreach ($p in $Patterns) { if ($Value -match $p) { return $true } }
    $false
}

function Invoke-Srch {
    param([string] $Exe, [string] $Query, [string] $Root, [int] $Limit, [string[]] $ExtraArgs)
    $argv = @($Query, '-p', $Root, '--json', '-n', "$Limit") + $ExtraArgs
    $sw = [Diagnostics.Stopwatch]::StartNew()
    $raw = & $Exe @argv 2>$null
    $sw.Stop()
    $json = $null
    if ($raw) { $json = ($raw -join "`n") | ConvertFrom-Json }
    [pscustomobject]@{ Wall = $sw.Elapsed.TotalMilliseconds; Json = $json }
}

$root = [IO.Path]::GetFullPath($Root)
if (-not (Test-Path $root)) { throw "root not found: $root" }
$queries = @(Get-Content -LiteralPath $QueryFile | Where-Object { $_.Trim() -ne '' -and -not $_.StartsWith('#') })
if ($queries.Count -eq 0) { throw "no queries in $QueryFile" }
$bins = Resolve-Binaries -Spec $Exe
if (-not $OutDir) { $OutDir = Join-Path $PSScriptRoot 'out' }
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

$rows = New-Object System.Collections.Generic.List[object]
foreach ($label in $bins.Keys) {
    $exePath = $bins[$label]
    foreach ($q in $queries) {
        # warm the file cache once, then take the best of $Repeat timed runs
        $null = Invoke-Srch -Exe $exePath -Query $q -Root $root -Limit $Limit -ExtraArgs $ExtraArgs
        $best = $null
        for ($i = 0; $i -lt $Repeat; $i++) {
            $run = Invoke-Srch -Exe $exePath -Query $q -Root $root -Limit $Limit -ExtraArgs $ExtraArgs
            if ($null -eq $best -or $run.Wall -lt $best.Wall) { $best = $run }
        }
        $results = @()
        if ($best.Json -and $best.Json.results) { $results = @($best.Json.results) }
        $paths = @($results | ForEach-Object { Get-RelativePath -Full $_.path -Root $root })

        $targetRank = 0
        if ($TargetPattern.Count -gt 0) {
            for ($i = 0; $i -lt $paths.Count; $i++) {
                if (Test-AnyPattern -Value $paths[$i] -Patterns $TargetPattern) { $targetRank = $i + 1; break }
            }
        }
        $noise = 0
        if ($NoisePattern.Count -gt 0) {
            $noise = @($paths | Where-Object { Test-AnyPattern -Value $_ -Patterns $NoisePattern }).Count
        }
        $rows.Add([pscustomobject]@{
                binary        = $label
                query         = $q
                target_rank   = $targetRank
                target_path   = if ($targetRank -gt 0) { $paths[$targetRank - 1] } else { '' }
                top1          = if ($paths.Count -gt 0) { $paths[0] } else { '' }
                slots         = $paths.Count
                noise_slots   = $noise
                noise_frac    = if ($paths.Count -gt 0) { [math]::Round($noise / $paths.Count, 3) } else { 0 }
                wall_ms       = [math]::Round($best.Wall, 1)
                scan_ms       = if ($best.Json) { $best.Json.elapsed_ms } else { $null }
                files_scanned = if ($best.Json) { $best.Json.files_scanned } else { $null }
            })
    }
}

$stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$jsonPath = Join-Path $OutDir "bench-$stamp.json"
$rows | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath $jsonPath -Encoding UTF8

$labels = @($bins.Keys)
$md = New-Object System.Collections.Generic.List[string]
$md.Add("| query | " + (($labels | ForEach-Object { "$_ rank | $_ noise" }) -join ' | ') + ' |')
$md.Add('|---|' + (($labels | ForEach-Object { '---|---|' }) -join ''))
foreach ($q in $queries) {
    $cells = foreach ($l in $labels) {
        $r = $rows | Where-Object { $_.binary -eq $l -and $_.query -eq $q } | Select-Object -First 1
        $rank = if ($r.target_rank -gt 0) { "$($r.target_rank)" } else { 'miss' }
        "$rank | $($r.noise_slots)/$($r.slots)"
    }
    $md.Add("| $($q -replace '\|', '\|') | " + ($cells -join ' | ') + ' |')
}
$md.Add('')
foreach ($l in $labels) {
    $sub = @($rows | Where-Object { $_.binary -eq $l })
    $hits = @($sub | Where-Object { $_.target_rank -gt 0 })
    $top1 = @($sub | Where-Object { $_.target_rank -eq 1 }).Count
    $top3 = @($sub | Where-Object { $_.target_rank -ge 1 -and $_.target_rank -le 3 }).Count
    $mrr = 0.0
    foreach ($r in $sub) { if ($r.target_rank -gt 0) { $mrr += 1.0 / $r.target_rank } }
    $mrr = if ($sub.Count) { [math]::Round($mrr / $sub.Count, 3) } else { 0 }
    $noiseFrac = if ($sub.Count) { [math]::Round((($sub | Measure-Object noise_slots -Sum).Sum) / (($sub | Measure-Object slots -Sum).Sum), 3) } else { 0 }
    $wall = [math]::Round(($sub | Measure-Object wall_ms -Average).Average, 1)
    $scan = [math]::Round(($sub | Measure-Object scan_ms -Average).Average, 1)
    $md.Add("**$l**: target@1=$top1/$($sub.Count) target@3=$top3/$($sub.Count) found=$($hits.Count)/$($sub.Count) MRR=$mrr noise=$noiseFrac wall_avg=${wall}ms scan_avg=${scan}ms")
}
$mdPath = Join-Path $OutDir "bench-$stamp.md"
$md -join "`n" | Set-Content -LiteralPath $mdPath -Encoding UTF8
$md -join "`n" | Write-Output
Write-Host "`nraw: $jsonPath`nmd:  $mdPath"
