$ErrorActionPreference = 'Stop'

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$yamlParser = Get-Command ConvertFrom-Yaml -ErrorAction SilentlyContinue
if ($null -eq $yamlParser) {
  throw 'ConvertFrom-Yaml is required to validate the host-parsed skill descriptions.'
}

$failures = [System.Collections.Generic.List[string]]::new()
$checked = 0

Get-ChildItem (Join-Path $repoRoot 'plugins') -Directory | ForEach-Object {
  $manifestPath = Join-Path $_.FullName 'plugin.json'
  if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf)) {
    return
  }

  $plugin = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
  $skillsRoot = Join-Path $_.FullName $plugin.skills
  if (-not (Test-Path -LiteralPath $skillsRoot -PathType Container)) {
    $failures.Add("$($_.Name): configured skills directory '$($plugin.skills)' is missing")
    return
  }

  Get-ChildItem $skillsRoot -Directory | ForEach-Object {
    $skillPath = Join-Path $_.FullName 'SKILL.md'
    if (-not (Test-Path -LiteralPath $skillPath -PathType Leaf)) {
      return
    }

    $checked++
    $skillName = "$($plugin.name)/$($_.Name)"
    $text = Get-Content -LiteralPath $skillPath -Raw
    $frontmatterMatch = [regex]::Match(
      $text,
      '\A---\r?\n(?<body>.*?)\r?\n---(?:\r?\n|$)',
      [System.Text.RegularExpressions.RegexOptions]::Singleline
    )
    if (-not $frontmatterMatch.Success) {
      $failures.Add("${skillName}: missing YAML frontmatter")
      return
    }

    try {
      $metadata = ConvertFrom-Yaml -Yaml $frontmatterMatch.Groups['body'].Value
    }
    catch {
      $failures.Add("${skillName}: invalid YAML frontmatter ($($_.Exception.Message))")
      return
    }

    if (-not $metadata.ContainsKey('description') -or $metadata.description -isnot [string]) {
      $failures.Add("${skillName}: description must be a YAML string")
      return
    }

    $description = $metadata.description
    if ([string]::IsNullOrWhiteSpace($description)) {
      $failures.Add("${skillName}: empty description")
    }
    elseif ($description.Length -gt 1024) {
      $failures.Add("${skillName}: description is $($description.Length) characters; maximum is 1024")
    }
  }
}

if ($failures.Count -gt 0) {
  throw "Skill discovery metadata validation failed:`n$($failures -join "`n")"
}

Write-Host "Skill discovery metadata OK ($checked skills; descriptions <= 1024 characters)."
