; PwGen Windows Installer Script
; Uses NSIS (Nullsoft Scriptable Install System)

!define PRODUCT_NAME "PwGen"
!define PRODUCT_VERSION "1.2.0"
!define PRODUCT_PUBLISHER "HxHippy, Kief Studio, TRaViS"
!define PRODUCT_WEB_SITE "https://pwgenrust.dev"
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\pwgen-gui.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"

; Modern UI
!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "LogicLib.nsh"

; Installer properties
Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "pwgen-windows-x64-installer.exe"
InstallDir "$PROGRAMFILES64\PwGen"
InstallDirRegKey HKLM "${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show
RequestExecutionLevel admin

; Version information
VIProductVersion "1.2.0.0"
VIAddVersionKey "ProductName" "${PRODUCT_NAME}"
VIAddVersionKey "Comments" "Advanced Password & Secrets Manager built in Rust"
VIAddVersionKey "CompanyName" "${PRODUCT_PUBLISHER}"
VIAddVersionKey "LegalCopyright" "© 2025 ${PRODUCT_PUBLISHER}"
VIAddVersionKey "FileDescription" "${PRODUCT_NAME} Installer"
VIAddVersionKey "FileVersion" "${PRODUCT_VERSION}"
VIAddVersionKey "ProductVersion" "${PRODUCT_VERSION}"
VIAddVersionKey "InternalName" "pwgen-installer"
VIAddVersionKey "OriginalFilename" "pwgen-windows-x64-installer.exe"

; Interface settings
!define MUI_ABORTWARNING
!define MUI_ICON "assets\PWGenLogo.ico"
!define MUI_UNICON "assets\PWGenLogo.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "assets\PWGenLogo-Wide.bmp"
!define MUI_WELCOMEFINISHPAGE_BITMAP "assets\PWGenLogo-Wide.bmp"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_RUN "$INSTDIR\pwgen-gui.exe"
!define MUI_FINISHPAGE_SHOWREADME "$INSTDIR\README.txt"
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

; Languages
!insertmacro MUI_LANGUAGE "English"

; Installer sections
Section "PwGen Core" SecCore
  SectionIn RO
  SetOutPath "$INSTDIR"
  
  ; Main executables
  File "target\x86_64-pc-windows-msvc\min-size\pwgen-gui.exe"
  File "target\x86_64-pc-windows-msvc\min-size\pwgen-cli.exe"
  
  ; Documentation
  File "README.md"
  File "LICENSE"
  File "CHANGELOG.md"
  
  ; Create application data directory
  SetShellVarContext all
  CreateDirectory "$APPDATA\PwGen"
  
  ; Registry entries
  WriteRegStr HKLM "${PRODUCT_DIR_REGKEY}" "" "$INSTDIR\pwgen-gui.exe"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "DisplayName" "$(^Name)"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "UninstallString" "$INSTDIR\uninst.exe"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "DisplayIcon" "$INSTDIR\pwgen-gui.exe"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr HKLM "${PRODUCT_UNINST_KEY}" "Publisher" "${PRODUCT_PUBLISHER}"
  WriteRegDWORD HKLM "${PRODUCT_UNINST_KEY}" "NoModify" 1
  WriteRegDWORD HKLM "${PRODUCT_UNINST_KEY}" "NoRepair" 1
  
  ; Estimate install size
  ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
  IntFmt $0 "0x%08X" $0
  WriteRegDWORD HKLM "${PRODUCT_UNINST_KEY}" "EstimatedSize" "$0"
  
  ; Create uninstaller
  WriteUninstaller "$INSTDIR\uninst.exe"
SectionEnd

Section "Desktop Shortcut" SecDesktop
  SetShellVarContext all
  CreateShortcut "$DESKTOP\PwGen.lnk" "$INSTDIR\pwgen-gui.exe" "" "$INSTDIR\pwgen-gui.exe" 0
SectionEnd

Section "Start Menu Shortcuts" SecStartMenu
  SetShellVarContext all
  CreateDirectory "$SMPROGRAMS\PwGen"
  CreateShortcut "$SMPROGRAMS\PwGen\PwGen.lnk" "$INSTDIR\pwgen-gui.exe" "" "$INSTDIR\pwgen-gui.exe" 0
  CreateShortcut "$SMPROGRAMS\PwGen\PwGen CLI.lnk" "$INSTDIR\pwgen-cli.exe" "" "$INSTDIR\pwgen-cli.exe" 0
  CreateShortcut "$SMPROGRAMS\PwGen\Uninstall PwGen.lnk" "$INSTDIR\uninst.exe" "" "$INSTDIR\uninst.exe" 0
SectionEnd

Section "Add to PATH" SecPath
  ; Add installation directory to PATH for CLI access
  EnVar::SetHKLM
  EnVar::AddValue "PATH" "$INSTDIR"
SectionEnd

Section "File Associations" SecFileAssoc
  ; Associate .pwgen files with PwGen
  WriteRegStr HKCR ".pwgen" "" "PwGenVault"
  WriteRegStr HKCR "PwGenVault" "" "PwGen Vault File"
  WriteRegStr HKCR "PwGenVault\DefaultIcon" "" "$INSTDIR\pwgen-gui.exe,0"
  WriteRegStr HKCR "PwGenVault\shell\open\command" "" '"$INSTDIR\pwgen-gui.exe" "%1"'
SectionEnd

; Section descriptions
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecCore} "Core PwGen application files (required)"
  !insertmacro MUI_DESCRIPTION_TEXT ${SecDesktop} "Create a desktop shortcut for PwGen"
  !insertmacro MUI_DESCRIPTION_TEXT ${SecStartMenu} "Create Start Menu shortcuts"
  !insertmacro MUI_DESCRIPTION_TEXT ${SecPath} "Add PwGen to system PATH for CLI access"
  !insertmacro MUI_DESCRIPTION_TEXT ${SecFileAssoc} "Associate .pwgen vault files with PwGen"
!insertmacro MUI_FUNCTION_DESCRIPTION_END

; Uninstaller section
Section Uninstall
  SetShellVarContext all
  
  ; Remove files
  Delete "$INSTDIR\pwgen-gui.exe"
  Delete "$INSTDIR\pwgen-cli.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\CHANGELOG.md"
  Delete "$INSTDIR\uninst.exe"
  
  ; Remove shortcuts
  Delete "$DESKTOP\PwGen.lnk"
  Delete "$SMPROGRAMS\PwGen\*.*"
  RMDir "$SMPROGRAMS\PwGen"
  
  ; Remove registry entries
  DeleteRegKey HKLM "${PRODUCT_UNINST_KEY}"
  DeleteRegKey HKLM "${PRODUCT_DIR_REGKEY}"
  DeleteRegKey HKCR ".pwgen"
  DeleteRegKey HKCR "PwGenVault"
  
  ; Remove from PATH
  EnVar::SetHKLM
  EnVar::DeleteValue "PATH" "$INSTDIR"
  
  ; Remove installation directory
  RMDir "$INSTDIR"
  
  ; Note: We don't remove user data directory for safety
  MessageBox MB_YESNO|MB_ICONQUESTION "Do you want to remove your PwGen vault and configuration files?$\r$\n$\r$\nThis will permanently delete all your passwords and data!" IDNO skip_userdata
    RMDir /r "$APPDATA\PwGen"
  skip_userdata:
SectionEnd

; Functions
Function .onInit
  ; Check if already installed
  ReadRegStr $R0 HKLM "${PRODUCT_UNINST_KEY}" "UninstallString"
  StrCmp $R0 "" done
  
  MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION \
  "${PRODUCT_NAME} is already installed. $\n$\nClick OK to remove the \
  previous version or Cancel to cancel this upgrade." \
  IDOK uninst
  Abort
  
  uninst:
    ClearErrors
    ExecWait '$R0 _?=$INSTDIR'
    
    IfErrors no_remove_uninstaller done
    no_remove_uninstaller:
  
  done:
FunctionEnd

Function .onInstSuccess
  ; Optional: Check for Windows Defender exclusion
  MessageBox MB_YESNO|MB_ICONQUESTION "Would you like to add PwGen to Windows Defender exclusions?$\r$\n$\r$\nThis can improve performance and prevent false positives." IDNO skip_defender
    ExecShell "runas" "powershell.exe" 'Add-MpPreference -ExclusionPath "$INSTDIR"' SW_HIDE
  skip_defender:
FunctionEnd