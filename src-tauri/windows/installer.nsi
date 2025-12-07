; Pseudo-portable NSIS Installer for Proxima
; - No registry writes
; - No shortcuts
; - No Add/Remove Programs entry
; - No uninstaller
; - By default, installs to the same directory as the installer executable
; - Only installs Webview2 if not installed on the target machine

Unicode true
ManifestDPIAware true
ManifestDPIAwareness PerMonitorV2

!if "{{compression}}" == "none"
  SetCompress off
!else
  SetCompressor /SOLID "{{compression}}"
!endif

!include MUI2.nsh
!include FileFunc.nsh
!include x64.nsh
!include "utils.nsh"

{{#if installer_hooks}}
!include "{{installer_hooks}}"
{{/if}}

!define WEBVIEW2APPGUID "{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}"

!define MANUFACTURER "{{manufacturer}}"
!define PRODUCTNAME "{{product_name}}"
!define VERSION "{{version}}"
!define VERSIONWITHBUILD "{{version_with_build}}"
!define INSTALLERICON "{{installer_icon}}"
!define MAINBINARYNAME "{{main_binary_name}}"
!define MAINBINARYSRCPATH "{{main_binary_path}}"
!define COPYRIGHT "{{copyright}}"
!define OUTFILE "{{out_file}}"
!define ADDITIONALPLUGINSPATH "{{additional_plugins_path}}"
!define INSTALLWEBVIEW2MODE "{{install_webview2_mode}}"
!define WEBVIEW2INSTALLERARGS "{{webview2_installer_args}}"
!define WEBVIEW2BOOTSTRAPPERPATH "{{webview2_bootstrapper_path}}"
!define WEBVIEW2INSTALLERPATH "{{webview2_installer_path}}"
!define MINIMUMWEBVIEW2VERSION "{{minimum_webview2_version}}"

Name "${PRODUCTNAME}"
BrandingText "${COPYRIGHT}"
OutFile "${OUTFILE}"

; Install directly to the installer's directory (no subfolder)
InstallDir "$EXEDIR"

VIProductVersion "${VERSIONWITHBUILD}"
VIAddVersionKey "ProductName" "${PRODUCTNAME}"
VIAddVersionKey "FileDescription" "${PRODUCTNAME}"
VIAddVersionKey "LegalCopyright" "${COPYRIGHT}"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "ProductVersion" "${VERSION}"

# additional plugins
!addplugindir "${ADDITIONALPLUGINSPATH}"

; Portable installer - no admin rights needed
RequestExecutionLevel user

; Installer icon
!if "${INSTALLERICON}" != ""
  !define MUI_ICON "${INSTALLERICON}"
!endif

; Installer pages - minimal for portable installer
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

!define MUI_FINISHPAGE_NOAUTOCLOSE
!define MUI_FINISHPAGE_RUN "$INSTDIR\${MAINBINARYNAME}.exe"
!insertmacro MUI_PAGE_FINISH

; Single language - English only for simplicity
!insertmacro MUI_LANGUAGE "English"

; WebView2 language strings
LangString webview2Downloading ${LANG_ENGLISH} "Downloading WebView2..."
LangString webview2DownloadSuccess ${LANG_ENGLISH} "WebView2 downloaded successfully"
LangString webview2DownloadError ${LANG_ENGLISH} "Failed to download WebView2"
LangString webview2AbortError ${LANG_ENGLISH} "WebView2 installation failed. The application cannot run without it."
LangString installingWebview2 ${LANG_ENGLISH} "Installing WebView2..."
LangString webview2InstallSuccess ${LANG_ENGLISH} "WebView2 installed successfully"
LangString webview2InstallError ${LANG_ENGLISH} "WebView2 installation error"

Section WebView2
  ; Check if Webview2 is already installed and skip this section
  ${If} ${RunningX64}
    ReadRegStr $4 HKLM "SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\${WEBVIEW2APPGUID}" "pv"
  ${Else}
    ReadRegStr $4 HKLM "SOFTWARE\Microsoft\EdgeUpdate\Clients\${WEBVIEW2APPGUID}" "pv"
  ${EndIf}
  ${If} $4 == ""
    ReadRegStr $4 HKCU "SOFTWARE\Microsoft\EdgeUpdate\Clients\${WEBVIEW2APPGUID}" "pv"
  ${EndIf}

  ${If} $4 == ""
    ; Webview2 installation
    !if "${INSTALLWEBVIEW2MODE}" == "downloadBootstrapper"
      Delete "$TEMP\MicrosoftEdgeWebview2Setup.exe"
      DetailPrint "$(webview2Downloading)"
      NSISdl::download "https://go.microsoft.com/fwlink/p/?LinkId=2124703" "$TEMP\MicrosoftEdgeWebview2Setup.exe"
      Pop $0
      ${If} $0 == "success"
        DetailPrint "$(webview2DownloadSuccess)"
      ${Else}
        DetailPrint "$(webview2DownloadError)"
        Abort "$(webview2AbortError)"
      ${EndIf}
      StrCpy $6 "$TEMP\MicrosoftEdgeWebview2Setup.exe"
      Goto install_webview2
    !endif

    !if "${INSTALLWEBVIEW2MODE}" == "embedBootstrapper"
      Delete "$TEMP\MicrosoftEdgeWebview2Setup.exe"
      File "/oname=$TEMP\MicrosoftEdgeWebview2Setup.exe" "${WEBVIEW2BOOTSTRAPPERPATH}"
      DetailPrint "$(installingWebview2)"
      StrCpy $6 "$TEMP\MicrosoftEdgeWebview2Setup.exe"
      Goto install_webview2
    !endif

    !if "${INSTALLWEBVIEW2MODE}" == "offlineInstaller"
      Delete "$TEMP\MicrosoftEdgeWebView2RuntimeInstaller.exe"
      File "/oname=$TEMP\MicrosoftEdgeWebView2RuntimeInstaller.exe" "${WEBVIEW2INSTALLERPATH}"
      DetailPrint "$(installingWebview2)"
      StrCpy $6 "$TEMP\MicrosoftEdgeWebView2RuntimeInstaller.exe"
      Goto install_webview2
    !endif

    Goto webview2_done

    install_webview2:
      DetailPrint "$(installingWebview2)"
      ; $6 holds the path to the webview2 installer
      ExecWait "$6 ${WEBVIEW2INSTALLERARGS} /install" $1
      ${If} $1 = 0
        DetailPrint "$(webview2InstallSuccess)"
      ${Else}
        DetailPrint "$(webview2InstallError)"
        Abort "$(webview2AbortError)"
      ${EndIf}
    webview2_done:
  ${Else}
    !if "${MINIMUMWEBVIEW2VERSION}" != ""
      ${VersionCompare} "${MINIMUMWEBVIEW2VERSION}" "$4" $R0
      ${If} $R0 = 1
        update_webview:
          DetailPrint "$(installingWebview2)"
          ${If} ${RunningX64}
            ReadRegStr $R1 HKLM "SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate" "path"
          ${Else}
            ReadRegStr $R1 HKLM "SOFTWARE\Microsoft\EdgeUpdate" "path"
          ${EndIf}
          ${If} $R1 == ""
            ReadRegStr $R1 HKCU "SOFTWARE\Microsoft\EdgeUpdate" "path"
          ${EndIf}
          ${If} $R1 != ""
            ExecWait `"$R1" /install appguid=${WEBVIEW2APPGUID}&needsadmin=true` $1
            ${If} $1 = 0
              DetailPrint "$(webview2InstallSuccess)"
            ${Else}
              MessageBox MB_ICONEXCLAMATION|MB_ABORTRETRYIGNORE "$(webview2InstallError)" IDIGNORE ignore IDRETRY update_webview
              Quit
              ignore:
            ${EndIf}
          ${EndIf}
      ${EndIf}
    !endif
  ${EndIf}
SectionEnd

Section Install
  SetOutPath $INSTDIR

  !ifmacrodef NSIS_HOOK_PREINSTALL
    !insertmacro NSIS_HOOK_PREINSTALL
  !endif

  ; Copy main executable
  File "${MAINBINARYSRCPATH}"

  ; Copy resources
  {{#each resources_dirs}}
    CreateDirectory "$INSTDIR\\{{this}}"
  {{/each}}
  {{#each resources}}
    File /a "/oname={{this.[1]}}" "{{no-escape @key}}"
  {{/each}}

  ; Copy external binaries
  {{#each binaries}}
    File /a "/oname={{this}}" "{{no-escape @key}}"
  {{/each}}

  !ifmacrodef NSIS_HOOK_POSTINSTALL
    !insertmacro NSIS_HOOK_POSTINSTALL
  !endif
SectionEnd
