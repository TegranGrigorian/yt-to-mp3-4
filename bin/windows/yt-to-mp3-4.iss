; Inno Setup script for yt-to-mp3-4
; Generated: 2025-10-06
; Notes: Place this .iss file in the same folder as the files you want to include
; (ffmpeg.exe, ffprobe.exe, yt-dlp.exe). If you have the application executable
; (yt-to-mp3-4.exe), put it in this folder too before compiling the installer.

[Setup]
AppName=yt-to-mp3-4
AppVersion=0.1.0
DefaultDirName={pf}\yt-to-mp3-4
DefaultGroupName=yt-to-mp3-4
OutputDir=..\..\dist
OutputBaseFilename=yt-to-mp3-4-setup
Compression=lzma
SolidCompression=yes
PrivilegesRequired=admin

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
; Main application executable in root
Source: "yt-to-mp3-4.exe"; DestDir: "{app}"; Flags: ignoreversion

; Helper binaries in bin/windows subdirectory (as expected by the app)
Source: "ffmpeg.exe"; DestDir: "{app}\bin\windows"; Flags: ignoreversion
Source: "ffprobe.exe"; DestDir: "{app}\bin\windows"; Flags: ignoreversion
Source: "yt-dlp.exe"; DestDir: "{app}\bin\windows"; Flags: ignoreversion

[Tasks]
Name: desktopicon; Description: "Create a &desktop icon"; GroupDescription: "Additional icons:"; Flags: unchecked

[Icons]
; Shortcuts only created if the main executable is present.
; The following entries are conditional (see [Code] section) so they won't
; create broken shortcuts if `yt-to-mp3-4.exe` is not bundled in this folder.
Name: "{group}\\yt-to-mp3-4"; Filename: "{app}\\yt-to-mp3-4.exe"; WorkingDir: "{app}"
Name: "{commondesktop}\\yt-to-mp3-4"; Filename: "{app}\\yt-to-mp3-4.exe"; Tasks: desktopicon

[Run]
; Optionally run the app after install, only if executable exists
Filename: "{app}\\yt-to-mp3-4.exe"; Description: "Launch yt-to-mp3-4"; Flags: nowait postinstall skipifsilent

; End of script
