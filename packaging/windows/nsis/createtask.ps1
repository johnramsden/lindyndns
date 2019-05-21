$exe = $args[0]

$name = "lindyndns Dynamic DNS update"

$repeat = New-TimeSpan -Minutes 30
$trigger = New-JobTrigger -Once -At (Get-Date).Date -RepeatIndefinitely -RepetitionInterval $repeat

$option = New-ScheduledJobOption -ContinueIfGoingOnBattery -StartIfOnBattery

$scriptblock = [scriptblock]::Create( "Start-Process -WindowStyle hidden -FilePath '$exe'" )

Register-ScheduledJob -Name $name -Trigger $trigger -ScheduledJobOption $option -ScriptBlock $scriptblock
