!include nsDialogs.nsh
!include LogicLib.nsh
!include Sections.nsh

Var Dialog
Var api_token_label
Var api_token
Var domain_label
Var domain
Var soa_email_label
Var soa_email

Page components
Page custom configurationPage configurationPageLeave
Page instfiles

Function configurationPage

    SetOutPath $INSTDIR
    File "createtask.ps1"
    File "removetask.ps1"
    File "taskwrapper.bat"

	${If} ${SectionIsSelected} 0

        nsDialogs::Create 1018
        Pop $Dialog

        ${If} $Dialog == error
            Abort
        ${EndIf}

        ${NSD_CreateLabel} 0 13u 100% 12u "API token:"
        Pop $api_token_label
        ${NSD_CreateText} 0 26u 100% 12u ""
        Pop $api_token
        
        ${NSD_CreateLabel} 0 41u 100% 12u "Domain:"
        Pop $domain_label
        ${NSD_CreateText} 0 54u 100% 12u ""
        Pop $domain

        ${NSD_CreateLabel} 0 69u 100% 12u "Email:"
        Pop $soa_email_label
        ${NSD_CreateText} 0 82u 100% 12u ""
        Pop $soa_email

        nsDialogs::Show

    ${EndIf}

    ${If} ${SectionIsSelected} 1
        ExpandEnvStrings $0 %COMSPEC%
        nsExec::ExecToStack `"$INSTDIR\taskwrapper.bat" "$INSTDIR\createtask.ps1" "$INSTDIR\lindyndns.exe"`
        Pop $0 # return value/error/timeout
        Pop $1 # printed text, up to ${NSIS_MAX_STRLEN}
        ${If} $0 == "0"
            MessageBox MB_OK 'Created scheduled job.$0:$1'
        ${Else}
            MessageBox MB_OK 'Scheduled task creation failed!$0:$1'
            
            Delete "$INSTDIR\uninstall.exe"
            Delete "$INSTDIR\lindyndns.exe"
            Delete "$INSTDIR\createtask.ps1"
            Delete "$INSTDIR\removetask.ps1"
            Delete "$INSTDIR\taskwrapper.bat"

            Abort "Install failed"
        ${EndIf}
    ${EndIf}
	
FunctionEnd

Function configurationPageLeave

    ${IfNot} ${SectionIsSelected} 0
        Abort
    ${EndIf}

    CreateDirectory "$LOCALAPPDATA\lindyndns"

    FileOpen $0 "$LOCALAPPDATA\lindyndns\config.toml" w

    ${NSD_GetText} $api_token $1
    ${NSD_GetText} $domain $2
    ${NSD_GetText} $soa_email $3

    FileWrite $0 'api_token = "$1"$\r$\n'
    FileWrite $0 'domain = "$2"$\r$\n'
    FileWrite $0 'soa_email = "$3"$\r$\n'

    FileClose $0

FunctionEnd
