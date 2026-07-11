$ErrorActionPreference = 'Stop'

$Owner = "ukangaekom"
$Repo = "hisho_2.0"
$BinaryName = "hisho"

Write-Host "Setting up $BinaryName installation..."

# 1. Configure paths
$DestDir = "$env:LOCALAPPDATA\Programs\$BinaryName"
$DestFile = "$DestDir\$BinaryName.exe"
$Url = "https://github.com/$Owner/$Repo/releases/latest/download/$BinaryName-windows-amd64.exe"

# 2. Create target directory
if (!(Test-Path -Path $DestDir)) {
    New-Item -ItemType Directory -Force -Path $DestDir | Out-Null
}

# 3. Download the executable
Write-Host "Downloading binary from GitHub..."
Invoke-WebRequest -Uri $Url -OutFile $DestFile

Write-Host ""
Write-Host "=============================================" -ForegroundColor Green
Write-Host "Successfully installed $BinaryName!" -ForegroundColor Green
Write-Host "=============================================" -ForegroundColor Green
Write-Host ""
Write-Host "To finish setup, add the folder to your User PATH environment variable:"
Write-Host "[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ';$DestDir', 'User')"
Write-Host "Restart your PowerShell window to apply changes."