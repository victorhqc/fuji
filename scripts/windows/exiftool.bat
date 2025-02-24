@echo off
setlocal

  set VERSION=13.21
  set ARCH=64

  mkdir .\deps
  cd .\deps

  echo Removing previous installation
  rmdir /S /Q exiftool

  set URL="https://exiftool.org/exiftool-%VERSION%_%ARCh%.zip"
  set OUTPUT_FILE=exiftool.zip


  echo Downloading %URL%...
  powershell -Command "Invoke-WebRequest -Uri %URL% -OutFile %OUTPUT_FILE%"

  if exist %OUTPUT_FILE% (
      echo Download completed.
  ) else (
      echo Download failed!
      exit /b 1
  )

  set EXTRACT_DIR=exiftool
  mkdir %EXTRACT_DIR%

  echo Unzipping the contents to %EXTRACT_DIR%...
  powershell -Command "Expand-Archive -Path %OUTPUT_FILE% -DestinationPath %EXTRACT_DIR%"

  if exist %EXTRACT_DIR% (
      echo Unzipping completed.
  ) else (
      echo Unzipping failed!
      exit /b 1
  )

  echo Cleaning up zip file...
  del %OUTPUT_FILE%

  echo Renaming path to exiftool
  ren "%EXTRACT_DIR%\exiftool-%VERSION%_%ARCh%" exiftool

  echo Moving extracted files to deps
  xcopy %EXTRACT_DIR%\exiftool\* %EXTRACT_DIR% /Q /E /H /K /Y

  echo Deleting nested exiftool path
  rmdir /S /Q "%EXTRACT_DIR%\exiftool"

endlocal
