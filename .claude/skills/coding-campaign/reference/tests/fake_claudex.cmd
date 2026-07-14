@echo off
set "model="
:args
if "%~1"=="" goto run
if "%~1"=="--model" (
  set "model=%~2"
  shift
  shift
  goto args
)
shift
goto args
:run
if "%model%"=="fast-pass" echo PASS& exit /b 0
if "%model%"=="fast-fail" echo FAIL& exit /b 0
if "%model%"=="slow-pass" (
  ping -n 9 127.0.0.1 >nul
  echo PASS
  exit /b 0
)
exit /b 3
