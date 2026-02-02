# StarFetch install script (Windows PowerShell)
# Usage: irm https://raw.githubusercontent.com/Linus-Shyu/StarFetch_Core/master/install.ps1 | iex

$ErrorActionPreference = "Stop"
$Repo = "Linus-Shyu/StarFetch_Core"

Write-Host "StarFetch installer" -ForegroundColor Cyan

# Get latest release version
$apiUrl = "https://api.github.com/repos/$Repo/releases/latest"
try {
    $release = Invoke-RestMethod -Uri $apiUrl -UseBasicParsing
} catch {
    Write-Error "Could not get latest release: $_"
    exit 1
}

$version = $release.tag_name -replace "^v", ""
$assetName = "starfetch-x86_64-pc-windows-msvc.zip"
$asset = $release.assets | Where-Object { $_.name -eq $assetName }
if (-not $asset) {
    Write-Error "Asset $assetName not found in release $($release.tag_name)"
    exit 1
}

$downloadUrl = $asset.browser_download_url
$installDir = Join-Path $env:LOCALAPPDATA "starfetch"
$binDir = Join-Path $installDir "bin"

Write-Host "Installing starfetch v$version from $assetName"
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

$zipPath = Join-Path $env:TEMP "starfetch-$version.zip"
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
    Expand-Archive -Path $zipPath -DestinationPath $binDir -Force
} finally {
    if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
}

$exePath = Join-Path $binDir "starfetch.exe"
if (-not (Test-Path $exePath)) {
    Write-Error "starfetch.exe not found after extract"
    exit 1
}

# Add to user PATH if not already present
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$binDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$binDir", "User")
    $env:Path = "$env:Path;$binDir"
    Write-Host "Added $binDir to your user PATH."
}

Write-Host "Installed to $exePath" -ForegroundColor Green
Write-Host "Run 'starfetch' in a new terminal, or: $exePath"
& $exePath --version 2>$null
