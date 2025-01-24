Set-Location $PSScriptRoot
Start-Process -FilePath "dotnet" -ArgumentList @("build", "--configuration", "Release", "-v", "m") -WorkingDirectory $PSScriptRoot\src -NoNewWindow -Wait

$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
$newPath = $currentPath + ";$PSScriptRoot\src\bin\Release\net8.0"
[Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")
