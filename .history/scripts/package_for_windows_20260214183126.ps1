$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   POS SERVICE: BUILD & PACKAGE" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# 1. Build
Write-Host "1. Building Release Version..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) { Write-Error "Build Failed"; exit 1 }

# 2. Prepare Dist Folder
$DistDir = "$PSScriptRoot\..\dist"
if (Test-Path $DistDir) { Remove-Item $DistDir -Recurse -Force }
New-Item -ItemType Directory -Path $DistDir | Out-Null

# 3. Copy Files
Write-Host "2. Packaging files..." -ForegroundColor Yellow
$TargetDir = "$PSScriptRoot\..\target\release"

# Copy Executable
Copy-Item "$TargetDir\pos_hardware_service.exe" "$DistDir\"

# Copy Config
Copy-Item "$PSScriptRoot\..\config.toml" "$DistDir\"

# 4. Create "Double Click" Launchers
Write-Host "3. Creating Launchers..." -ForegroundColor Yellow

# RUN (Portable)
$RunBatContent = @"
@echo off
echo Starting POS Service...
echo Close this window to stop the service.
pos_hardware_service.exe
pause
"@
Set-Content "$DistDir\RUN_ME.bat" $RunBatContent


# INSTALL (Service)
# We embed the powershell logic into a simple bat file so they can just click it
# We use 'pushd %~dp0' to ensure we are in the correct directory even if run as Admin
$InstallBatContent = @"
@echo off
cd /d "%~dp0"
echo Installing POS Service...
PowerShell -NoProfile -ExecutionPolicy Bypass -Command "& {
    `$ExePath = '$PWD\pos_hardware_service.exe';
    `$WorkDir = '$PWD';
    `$TaskName = 'POS_Hardware_Background_Service';
    
    Unregister-ScheduledTask -TaskName `$TaskName -Confirm:`$false -ErrorAction SilentlyContinue;
    
    `$Action = New-ScheduledTaskAction -Execute `$ExePath -WorkingDirectory `$WorkDir;
    `$Trigger = New-ScheduledTaskTrigger -AtStartup;
    `$Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit (New-TimeSpan -Days 365) -RestartCount 3 -RestartInterval (New-TimeSpan -Minutes 1) -MultipleInstances IgnoreNew;
    
    Register-ScheduledTask -TaskName `$TaskName -Action `$Action -Trigger `$Trigger -Settings `$Settings -User 'SYSTEM' -RunLevel Highest;
    
    echo 'SUCCESS! Service Installed.';
    Start-Sleep -Seconds 3;
}"
pause
"@
Set-Content "$DistDir\INSTALL_AS_SERVICE.bat" $InstallBatContent

Write-Host "==========================================" -ForegroundColor Green
Write-Host "DONE! Your shiny new package is here:" -ForegroundColor Green
Write-Host "$DistDir" -ForegroundColor White
Write-Host "==========================================" -ForegroundColor Green
Get-Item $DistDir | Invoke-Item
