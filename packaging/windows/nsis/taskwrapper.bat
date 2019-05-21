SET ThisScriptsDirectory=%~dp0
SET PowerShellScriptPath=%1
SET ExePath=%2
PowerShell -NoProfile -WindowStyle hidden -ExecutionPolicy Bypass -Command "Start-Process -Verb RunAs PowerShell" "'-NoProfile -ExecutionPolicy Bypass -File \"%PowerShellScriptPath%\" \"%ExePath%\"'";
