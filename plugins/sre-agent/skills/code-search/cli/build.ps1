<#
.SYNOPSIS
    Build srch and stage the binary where the launcher's SRCH_CLI resolves it (cli\bin\srch.exe).
.DESCRIPTION
    bin\ is git-ignored, so every environment (including the sre-live snapshot step) builds its own
    binary. Rust/cargo is required. Universal Ctags is optional and used only by `srch index`.
#>
[CmdletBinding()]
param(
    [string]$CargoCommand = 'cargo',
    [string]$CargoTargetDir
)

$ErrorActionPreference = 'Stop'
$cli = $PSScriptRoot
$cargo = Get-Command $CargoCommand -CommandType Application -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "Rust cargo is required to build srch.exe but '$CargoCommand' was not found. Install Rust on the release builder and retry."
}

Push-Location $cli
try {
    $targetDir = if ([string]::IsNullOrWhiteSpace($CargoTargetDir)) {
        Join-Path $cli 'target'
    } else {
        [IO.Path]::GetFullPath($CargoTargetDir)
    }
    & $cargo.Source build --release --target-dir $targetDir
    if ($LASTEXITCODE -ne 0) { throw "cargo build failed ($LASTEXITCODE)" }
    $bin = Join-Path $cli 'bin'
    New-Item -ItemType Directory -Force -Path $bin | Out-Null
    Copy-Item (Join-Path $targetDir 'release\srch.exe') (Join-Path $bin 'srch.exe') -Force
    Write-Host "srch built and staged at $bin\srch.exe"
} finally {
    Pop-Location
}
