# Specify the downloads folder
$downloads_folder = (New-Object -ComObject Shell.Application).NameSpace('shell:Downloads').Self.Path
# Set the path for the DNAnalyzer directory in the downloads folder
$dir_path = "$downloads_folder/PasswordLLM"
# Creates a temporary windows progam file
$TempFile = New-TemporaryFile
# Accepts parameters for Github Workflow
$GHW = $args[0]
# Get's the current directory location of where the powershell script is located
#$ScriptLocation = Split-Path -Path (Get-Location) -Leaf

$program_name = "PasswordLLM"
$repo = "VerisimilitudeX/$program_name"
$file = "PasswordLLM-64x.exe"
$releases = "https://api.github.com/repos/$repo/releases"
$RockYou = "https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt"

#$ErrorActionPreference = 'SilentlyContinue' 
$ProgressPreference = 'SilentlyContinue'  # adds increased downloading speed

# if ($ScriptLocation -ne "$program_name") {
 
# } todo
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


Write-Host "Checking latest release of program..."
$tag = (Invoke-WebRequest $releases | ConvertFrom-Json)[0].tag_name
$download = "https://github.com/$repo/releases/download/$tag/$file"

if (!(Test-Path -Path ("$dir_path/$file") -PathType Leaf)) {
Write-Host "Downloading latest release for PasswordGPT to $dir_path/$file..."
Invoke-WebRequest -Uri $download -OutFile "$dir_path/$file"
}
else {
    Write-Host "Executable already exists, not downloading again!"
}

Write-Host "Downloading RockYou password database..."
Invoke-WebRequest -Uri $RockYou -Out $TempFile

$Rock_You_Path = $TempFile.FullName
$ShortcutPath = "$dir_path\RockYou.lnk"
Write-Host $Rock_You_Path

$WsScriptObj = New-Object -ComObject ("WScript.Shell")
$Shortcut = $WsScriptObj.CreateShortcut($ShortcutPath)
$Shortcut.TargetPath = $Rock_You_Path
$Shortcut.Save()

if (Test-Path "$dir_path\$file") {
    Write-Host "Installed sucessfully, running program now!" -ForegroundColor Green
    Set-Location -Path $dir_path
    $processTimer = [System.Diagnostics.Stopwatch]::StartNew()
    if ($GHW) {
        Write-Host "Running in a Github Workflow Windows machine" -ForegroundColor Cyan
        $PasswordLLM_Program = Start-Process -FilePath "$file" -ArgumentList "TestFrog" -PassThru -RedirectStandardOutput output.txt
        Start-Sleep -Seconds 50
        Stop-Process $PasswordLLM_Program

        Write-Output $PasswordLLM_Program
        $output = get-content "output.txt"
        Write-Output $output
        Write-Host "This took $($processTimer.Elapsed.TotalSeconds) seconds to run"
        $processTimer.Stop()
    }
    else {
        Start-Process -FilePath "$file" -Wait
        Write-Host "This took $($processTimer.Elapsed.TotalSeconds) seconds to run"
        $processTimer.Stop()
    }

    Write-Host "Cleaning up... Deleting temp files"
    try {
        Remove-Item $TempFile.FullName
        Remove-Item $ShortcutPath
    } catch [System.Exception] {
        Write-Host "Something went wrong..." -ForegroundColor Cyan
        Write-Error $_.Exception.Message
    }
}

