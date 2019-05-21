!include Sections.nsh

Section "uninstall"

    Delete "$INSTDIR\uninstall.exe"
    Delete "$INSTDIR\lindyndns.exe"

    ExpandEnvStrings $0 %COMSPEC%
    nsExec::ExecToStack `"$INSTDIR\taskwrapper.bat" "$INSTDIR\removetask.ps1" "$INSTDIR\lindyndns.exe"`
    Pop $0 # return value/error/timeout
    Pop $1 # printed text, up to ${NSIS_MAX_STRLEN}
    ${If} $0 == "0"
        MessageBox MB_OK 'Removed scheduled job.'
    ${EndIf}
    
    Delete "$INSTDIR\createtask.ps1"
    Delete "$INSTDIR\removetask.ps1"
    Delete "$INSTDIR\taskwrapper.bat"
 
SectionEnd
