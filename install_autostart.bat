@echo off
setlocal
set SCRIPT_DIR=%~dp0

REM Check for zenith-clock.exe or fallback to rust-clock.exe
if exist "%SCRIPT_DIR%zenith-clock.exe" (
    set TARGET_EXE=%SCRIPT_DIR%zenith-clock.exe
) else if exist "%SCRIPT_DIR%rust-clock.exe" (
    set TARGET_EXE=%SCRIPT_DIR%rust-clock.exe
) else (
    echo [ERROR] No executable found! Please run 'cargo build --release' first.
    pause
    exit /b 1
)

set SHORTCUT_PATH=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\ZenithClock.lnk

echo Adding Zenith Clock Overlay to Windows Startup...
powershell -Command "$ws = New-Object -ComObject WScript.Shell; $s = $ws.CreateShortcut('%SHORTCUT_PATH%'); $s.TargetPath = '%TARGET_EXE%'; $s.WorkingDirectory = '%SCRIPT_DIR%'; $s.Save()"

if exist "%SHORTCUT_PATH%" (
    echo [SUCCESS] Zenith Clock will now start automatically with Windows!
) else (
    echo [ERROR] Failed to create startup shortcut.
)
pause
