Name "lindyndns"

# define name of installer
OutFile "lindyndns_setup_i686.exe"
 
# define installation directory
InstallDir "$PROGRAMFILES\lindyndns"

Section 'Add configuration'
SectionEnd

Section 'Schedule recurring task'
SectionEnd

!include dialog.nsh
!include default.nsh
!include uninstall.nsh
