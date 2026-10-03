@echo off
setlocal
set SHORTCUT_PATH1=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\ZenithClock.lnk
set SHORTCUT_PATH2=%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\RustClockOverlay.lnk

set DELETED=0

if exist "%SHORTCUT_PATH1%" (
    del "%SHORTCUT_PATH1%"
    set DELETED=1
)
if exist "%SHORTCUT_PATH2%" (
    del "%SHORTCUT_PATH2%"
    set DELETED=1
)

if "%DELETED%"=="1" (
    echo [SUCCESS] Zenith Clock removed from Windows Startup.
) else (
    echo [INFO] No startup shortcut was found.
)
pause
