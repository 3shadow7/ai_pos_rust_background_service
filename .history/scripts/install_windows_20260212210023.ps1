# ==================================================================================
# AUTOMATIC INSTALLER FOR WINDOWS
# ==================================================================================
# This script registers the POS Hardware Service to start automatically when Windows boots.
# It uses Windows Task Scheduler. No external tools needed.
#
# Usage: Right-click this file -> "Run with PowerShell" (Run as Administrator)
# ==================================================================================

$ErrorActionPreference = "Stop"

# 1. Get location of the Release Executable
$ScriptPath = $PSScriptRoot
$ProjectRoot = Split-Path -Parent $ScriptPath
$ExePath = "$ProjectRoot\target\release\pos_hardware_service.exe"
$ConfigPath = "$ProjectRoot\config.toml"
$WorkDir = "$ProjectRoot"

Write-Host "Checking for executable at: $ExePath" -ForegroundColor Cyan

if (-not (Test-Path $ExePath)) {
    Write-Error "Executable not found! Did you run 'cargo build --release' first?"
}

# 2. Define Task Name
$TaskName = "POS_Hardware_Background_Service"

# 3. Create the Action (Run the .exe)
$Action = New-ScheduledTaskAction -Execute $ExePath -WorkingDirectory $WorkDir

# 4. Create the Trigger (At System Startup)
$Trigger = New-ScheduledTaskTrigger -AtStartup

# 5. Settings (Restart if it fails, don't stop if on battery, etc.)
$Settings = New-ScheduledTaskSettingsSet `
    -AllowStartIfOnBatteries `
    -DontStopIfGoingOnBatteries `
    -ExecutionTimeLimit (New-TimeSpan -Days 365) `
    -RestartCount 3 `
    -RestartInterval (New-TimeSpan -Minutes 1) `
    -MultipleInstances IgnoreNew

# 6. Description
$Desc = "Runs the Rust POS Hardware Service in the background. Connects Web App to Printers."

# 7. Unregister old task if it exists
Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false -ErrorAction SilentlyContinue

# 8. Register the new Task
Write-Host "Registering System Task '$TaskName'..." -ForegroundColor Yellow
# Running as "System" (S4U) allows it to run in background without a visible window
Register-ScheduledTask `
    -TaskName $TaskName `
    -Action $Action `
    -Trigger $Trigger `
    -Settings $Settings `
    -Description $Desc `
    -User "SYSTEM" `
    -RunLevel Highest

Write-Host "===========================================================" -ForegroundColor Green
Write-Host "SUCCESS! The service is installed." -ForegroundColor Green
Write-Host "It will start automatically when you restart the computer." -ForegroundColor Green
Write-Host "To start it immediately, open Task Scheduler or restart." -ForegroundColor Green
Write-Host "===========================================================" -ForegroundColor Green
