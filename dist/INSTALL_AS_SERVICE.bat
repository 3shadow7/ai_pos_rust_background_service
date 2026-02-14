@echo off
cd /d "%~dp0"
echo Installing POS Service...
PowerShell -NoProfile -ExecutionPolicy Bypass -Command "& {
    $ExePath = '$PWD\pos_hardware_service.exe';
    $WorkDir = '$PWD';
    $TaskName = 'POS_Hardware_Background_Service';
    
    Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false -ErrorAction SilentlyContinue;
    
    $Action = New-ScheduledTaskAction -Execute $ExePath -WorkingDirectory $WorkDir;
    $Trigger = New-ScheduledTaskTrigger -AtStartup;
    $Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit (New-TimeSpan -Days 365) -RestartCount 3 -RestartInterval (New-TimeSpan -Minutes 1) -MultipleInstances IgnoreNew;
    
    Register-ScheduledTask -TaskName $TaskName -Action $Action -Trigger $Trigger -Settings $Settings -User 'SYSTEM' -RunLevel Highest;
    
    echo 'SUCCESS! Service Installed.';
    Start-Sleep -Seconds 3;
}"
pause
