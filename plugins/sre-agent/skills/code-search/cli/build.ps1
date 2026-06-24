<#
.SYNOPSIS
    Build srch and stage the binary where the launcher's SRCH_CLI resolves it (cli\bin\srch.exe).
.DESCRIPTION
    bin\ is git-ignored, so every environment (including the sre-live snapshot step) builds its own
    binary. Prerequisites: Rust/cargo and Universal Ctags (ctags/readtags) on PATH.
#>
$ErrorActionPreference = 'Stop'
$cli = $PSScriptRoot
Push-Location $cli
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) { throw "cargo build failed ($LASTEXITCODE)" }
    $bin = Join-Path $cli 'bin'
    New-Item -ItemType Directory -Force -Path $bin | Out-Null
    Copy-Item (Join-Path $cli 'target\release\srch.exe') (Join-Path $bin 'srch.exe') -Force
    Write-Host "srch built and staged at $bin\srch.exe"
} finally {
    Pop-Location
}
