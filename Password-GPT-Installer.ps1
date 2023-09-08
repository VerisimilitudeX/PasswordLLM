# Specify the downloads folder
$downloads_folder = (New-Object -ComObject Shell.Application).NameSpace('shell:Downloads').Self.Path
# Set the path for the DNAnalyzer directory in the downloads folder
$dir_path = "$downloads_folder/PasswordGPT"
# Creates a temporary windows progam file
$TempFile = New-TemporaryFile

$repo = "VerisimilitudeX/PasswordGPT"
$file = "RealPass-x64.exe"
$releases = "https://api.github.com/repos/$repo/releases"
$RockYou = "https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt"

$ErrorActionPreference = 'SilentlyContinue' 
$ProgressPreference = 'SilentlyContinue'  # adds increased downloading speed

try {
    # Check if the directory already exists
    if ([System.IO.Directory]::Exists($dir_path)) {
        Write-Host "The directory already exists."
    } else {
        # Try to create the directory for DNAnalyzer in the downloads folder
        New-Item -Path "$dir_path" -ItemType Directory
    }
} catch [System.Exception] {
    # Catch any errors and print a message
    Write-Host "Something went wrong..." -ForegroundColor Red
    Write-Error $_.Exception.Message
}
$tag = (Invoke-WebRequest $releases | ConvertFrom-Json)[0].tag_name
$download = "https://github.com/$repo/releases/download/$tag/$file"

Write-Host "Downloading latest release for PasswordGPT..."

Invoke-WebRequest -Uri $download -OutFile "$dir_path/RealPass-x64.exe"
Invoke-WebRequest -Uri $RockYou -Out $TempFile

$Rock_You_Path = $TempFile.FullName
$ShortcutPath = "$dir_path\RockYou.lnk"
Write-Host $Rock_You_Path

$WsScriptObj = New-Object -ComObject ("WScript.Shell")
$Shortcut = $WsScriptObj.CreateShortcut($ShortcutPath)
$Shortcut.TargetPath = $Rock_You_Path
$Shortcut.Save()

if (Test-Path "$dir_path/RealPass-x64.exe") {
    Write-Host "Installed sucessfully, running program now!" -ForegroundColor Green
    Start-Process -FilePath "$dir_path/RealPass-x64.exe"
}

