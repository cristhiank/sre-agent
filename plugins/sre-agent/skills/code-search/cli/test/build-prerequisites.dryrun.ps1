[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$build = Join-Path (Split-Path -Parent $PSScriptRoot) 'build.ps1'
$missing = '__missing_cargo_for_srch_test__'

try {
    & $build -CargoCommand $missing
    throw 'Expected the srch build to reject a missing cargo command.'
} catch {
    if ($_.Exception.Message -notlike '*Rust cargo is required*') {
        throw
    }
}

Write-Host '[PASS] missing cargo fails with an actionable release-builder error'
