# start default section
Section
 
    # set the installation directory as the destination for the following actions
    SetOutPath $INSTDIR

    File "lindyndns.exe"
    File "createtask.ps1"
    File "removetask.ps1"
    File "taskwrapper.bat"
 
    # create the uninstaller
    WriteUninstaller "$INSTDIR\uninstall.exe"

SectionEnd
