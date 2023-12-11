#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

# Create bin directory for Rivet
$RivetInstall = $env:RIVET_INSTALL
$BinDir = if ($RivetInstall) {
	"${RivetInstall}\bin"
} else {
	"${Home}\.rivet\bin"
}

if (!(Test-Path $BinDir)) {
	New-Item $BinDir -ItemType Directory | Out-Null
}

$RivetZip = "$BinDir\rivet.zip"  # Path where the ZIP is downloaded
$RivetExeCi = "$BinDir\rivet-cli.exe"  # Default name generated by CI
$RivetExe = "$BinDir\rivet.exe"  # Name we will move binary to
$AssetName = 'rivet-cli-x86_64-pc-windows-msvc'
$FileName = "${AssetName}.zip"

# Auto-select version to install
#
# We have to find last version with an asset so we don't break the installer
# when the assets for a new version are still generating
$Version = $env:RIVET_CLI_VERSION
if (!$Version) {
	$Releases = Invoke-RestMethod -Uri "https://api.github.com/repos/rivet-gg/cli/releases"

	foreach ($Release in $Releases) {
		if (-not $Release.prerelease) {
			$SelectedAssets = $Release.assets | Select-Object -ExpandProperty name | Where-Object { $_ -eq $FileName }
			if ($SelectedAssets) {
				$Version = $Release.tag_name
				Break
			}
		}
	}

	if (!$Version) {
		throw 'Failed to determine version to install'
	}
}

Write-Host
Write-Host "> Installing Rivet CLI ${Version}"

# Download CLI
$DownloadUrl = "https://github.com/rivet-gg/cli/releases/download/${Version}/${FileName}"
Write-Host
Write-Host "> Downloading ${DownloadUrl}"
Invoke-WebRequest $DownloadUrl -OutFile $RivetZip -UseBasicParsing

# Extract archive
Write-Host
Write-Host "> Extracting rivet.zip"
Remove-Item $RivetExe -ErrorAction SilentlyContinue
Remove-Item $RivetExeCi -ErrorAction SilentlyContinue
if (Get-Command Expand-Archive -ErrorAction SilentlyContinue) {
	Expand-Archive $RivetZip -Destination $BinDir -Force
} else {
	Add-Type -AssemblyName System.IO.Compression.FileSystem
	[IO.Compression.ZipFile]::ExtractToDirectory($RivetZip, $BinDir)
}
Remove-Item $RivetZip

# Rename from rivet-cli to rivet, since rivet-cli is the default name generated by CI
Rename-Item -Path $RivetExeCi -NewName $RivetExe

# Install CLI
Write-Host
Write-Host "> Installing rivet"
$User = [System.EnvironmentVariableTarget]::User
$Path = [System.Environment]::GetEnvironmentVariable('Path', $User)
if (!(";${Path};".ToLower() -like "*;${BinDir};*".ToLower())) {
	[System.Environment]::SetEnvironmentVariable('Path', "${Path};${BinDir}", $User)
	$Env:Path += ";${BinDir}"
}

Write-Host
Write-Host "> Checking installation"
rivet.exe --version

Write-Host
Write-Host "Rivet was installed successfully to ${RivetExe}."
Write-Host "Run 'rivet --help' to get started."
Write-Host
