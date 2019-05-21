Name "lindyndns"

# define name of installer
OutFile "lindyndns_setup_x86_64.exe"
 
# define installation directory
InstallDir "$PROGRAMFILES64\lindyndns"

Section 'Add configuration'
SectionEnd

Section 'Schedule recurring task'
SectionEnd

!include dialog.nsh
!include default.nsh
!include uninstall.nsh
