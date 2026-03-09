# avdoc Windows installer
# Usage: irm https://raw.githubusercontent.com/Aryan-202/avdoc/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$Repo = "Aryan-202/avdoc"
$Binary = "avdoc.exe"
$Asset = "avdoc-windows-x86_64.exe"
$InstallDir = "$env:LOCALAPPDATA\Programs\avdoc"

Write-Host "Installing avdoc..." -ForegroundColor Blue

# Get latest release
$Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
$Version = $Release.tag_name

Write-Host "Latest version: $Version"
Write-Host "Downloading $Asset..."

$DownloadUrl = "https://github.com/$Repo/releases/download/$Version/$Asset"

# Create install directory
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# Download binary
$Destination = Join-Path $InstallDir $Binary
Invoke-WebRequest -Uri $DownloadUrl -OutFile $Destination

# Add to PATH if not already there
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "PATH",
        "$CurrentPath;$InstallDir",
        "User"
    )
    Write-Host "Added $InstallDir to your PATH." -ForegroundColor Yellow
    Write-Host "Restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
}

Write-Host "avdoc $Version installed successfully!" -ForegroundColor Green
Write-Host "Run 'avdoc --help' to get started."