# PwGen Password Manager - Windows Installer
# PowerShell script for automated installation on Windows

param(
    [switch]$Force,
    [switch]$NoDesktopShortcut,
    [switch]$Help
)

# Color functions for output
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    
    $colorMap = @{
        "Red" = "Red"
        "Green" = "Green" 
        "Yellow" = "Yellow"
        "Blue" = "Blue"
        "White" = "White"
        "Cyan" = "Cyan"
    }
    
    Write-Host $Message -ForegroundColor $colorMap[$Color]
}

function Log-Info {
    param([string]$Message)
    Write-ColorOutput "[INFO] $Message" "Blue"
}

function Log-Success {
    param([string]$Message)
    Write-ColorOutput "[SUCCESS] $Message" "Green"
}

function Log-Warning {
    param([string]$Message)
    Write-ColorOutput "[WARNING] $Message" "Yellow"
}

function Log-Error {
    param([string]$Message)
    Write-ColorOutput "[ERROR] $Message" "Red"
}

function Show-Help {
    Write-Host @"
PwGen Password Manager - Windows Installer

USAGE:
    .\install-windows.ps1 [OPTIONS]

OPTIONS:
    -Force              Force reinstallation even if already installed
    -NoDesktopShortcut  Don't create desktop shortcut
    -Help               Show this help message

EXAMPLES:
    .\install-windows.ps1
    .\install-windows.ps1 -Force
    .\install-windows.ps1 -NoDesktopShortcut

"@
}

# Check if running as administrator
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Check PowerShell execution policy
function Test-ExecutionPolicy {
    $policy = Get-ExecutionPolicy
    if ($policy -eq "Restricted") {
        Log-Warning "PowerShell execution policy is Restricted"
        Log-Info "Run this command as Administrator to allow script execution:"
        Log-Info "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope LocalMachine"
        return $false
    }
    return $true
}

# Download and install Chocolatey
function Install-Chocolatey {
    if (Get-Command choco -ErrorAction SilentlyContinue) {
        Log-Success "Chocolatey is already installed"
        return
    }
    
    Log-Info "Installing Chocolatey package manager..."
    
    if (-not (Test-Administrator)) {
        Log-Error "Administrator privileges required to install Chocolatey"
        Log-Info "Please run PowerShell as Administrator and try again"
        exit 1
    }
    
    try {
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
        
        # Refresh environment variables
        $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
        
        Log-Success "Chocolatey installed successfully"
    }
    catch {
        Log-Error "Failed to install Chocolatey: $($_.Exception.Message)"
        exit 1
    }
}

# Install Rust using Chocolatey or rustup
function Install-Rust {
    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        $rustVersion = (rustc --version).Split(' ')[1]
        Log-Success "Rust is already installed (version $rustVersion)"
        return
    }
    
    Log-Info "Installing Rust..."
    
    try {
        # Try rustup-init first (recommended method)
        if (-not (Get-Command rustup-init -ErrorAction SilentlyContinue)) {
            Log-Info "Downloading rustup-init..."
            $rustupUrl = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
            $rustupPath = "$env:TEMP\rustup-init.exe"
            
            (New-Object System.Net.WebClient).DownloadFile($rustupUrl, $rustupPath)
            
            Log-Info "Running rustup installer..."
            Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait
            
            # Clean up
            Remove-Item $rustupPath -ErrorAction SilentlyContinue
        }
        
        # Refresh environment variables
        $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "User") + ";" + $env:PATH
        
        Log-Success "Rust installed successfully"
    }
    catch {
        Log-Warning "Failed to install Rust via rustup, trying Chocolatey..."
        try {
            choco install rust -y
            Log-Success "Rust installed via Chocolatey"
        }
        catch {
            Log-Error "Failed to install Rust: $($_.Exception.Message)"
            exit 1
        }
    }
}

# Install required system dependencies
function Install-Dependencies {
    Log-Info "Installing system dependencies..."
    
    $packages = @(
        "git",
        "cmake",
        "llvm"
    )
    
    foreach ($package in $packages) {
        try {
            Log-Info "Installing $package..."
            choco install $package -y
        }
        catch {
            Log-Warning "Failed to install $package via Chocolatey: $($_.Exception.Message)"
        }
    }
    
    # Install Visual Studio Build Tools if not present
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (-not (Test-Path $vsWhere)) {
        Log-Info "Installing Visual Studio Build Tools..."
        try {
            choco install visualstudio2022buildtools -y
            choco install visualstudio2022-workload-vctools -y
        }
        catch {
            Log-Warning "Failed to install Visual Studio Build Tools"
            Log-Info "Please install Visual Studio Build Tools manually from:"
            Log-Info "https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022"
        }
    }
    else {
        Log-Success "Visual Studio Build Tools detected"
    }
}

# Build and install PwGen
function Build-Install {
    Log-Info "Building PwGen Password Manager..."
    
    # Ensure we're in the project directory
    if (-not (Test-Path "Cargo.toml")) {
        Log-Error "Cargo.toml not found. Please run this script from the project root directory."
        exit 1
    }
    
    try {
        # Build the project
        Log-Info "Compiling project (this may take a few minutes)..."
        cargo build --release
        
        if ($LASTEXITCODE -ne 0) {
            throw "Cargo build failed with exit code $LASTEXITCODE"
        }
        
        # Create installation directory
        $installDir = "$env:LOCALAPPDATA\PwGen"
        if (-not (Test-Path $installDir)) {
            New-Item -ItemType Directory -Path $installDir -Force | Out-Null
        }
        
        # Copy binaries
        Copy-Item "target\release\pwgen-cli.exe" "$installDir\" -Force
        Copy-Item "target\release\pwgen-gui.exe" "$installDir\" -Force
        
        # Copy assets
        if (Test-Path "ui\PWGenLogo.png") {
            Copy-Item "ui\PWGenLogo.png" "$installDir\" -Force
        }
        elseif (Test-Path "assets\PWGenLogo.png") {
            Copy-Item "assets\PWGenLogo.png" "$installDir\" -Force
        }
        
        # Add to PATH
        $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        if ($userPath -notlike "*$installDir*") {
            [Environment]::SetEnvironmentVariable("PATH", "$userPath;$installDir", "User")
            Log-Info "Added $installDir to user PATH"
        }
        
        # Create Start Menu shortcut
        $startMenuDir = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs"
        $shortcutPath = "$startMenuDir\PwGen Password Manager.lnk"
        
        $WScriptShell = New-Object -ComObject WScript.Shell
        $shortcut = $WScriptShell.CreateShortcut($shortcutPath)
        $shortcut.TargetPath = "$installDir\pwgen-gui.exe"
        $shortcut.WorkingDirectory = $installDir
        $shortcut.Description = "PwGen Password Manager"
        if (Test-Path "$installDir\PWGenLogo.png") {
            $shortcut.IconLocation = "$installDir\PWGenLogo.png"
        }
        $shortcut.Save()
        
        # Create Desktop shortcut (optional)
        if (-not $NoDesktopShortcut) {
            $desktopPath = "$env:USERPROFILE\Desktop\PwGen Password Manager.lnk"
            $desktopShortcut = $WScriptShell.CreateShortcut($desktopPath)
            $desktopShortcut.TargetPath = "$installDir\pwgen-gui.exe"
            $desktopShortcut.WorkingDirectory = $installDir
            $desktopShortcut.Description = "PwGen Password Manager"
            if (Test-Path "$installDir\PWGenLogo.png") {
                $desktopShortcut.IconLocation = "$installDir\PWGenLogo.png"
            }
            $desktopShortcut.Save()
            Log-Success "Desktop shortcut created"
        }
        
        Log-Success "PwGen installed successfully to $installDir"
    }
    catch {
        Log-Error "Installation failed: $($_.Exception.Message)"
        exit 1
    }
}

# Create uninstaller
function Create-Uninstaller {
    $installDir = "$env:LOCALAPPDATA\PwGen"
    $uninstallerPath = "$installDir\uninstall.ps1"
    
    $uninstallerScript = @"
# PwGen Uninstaller
Write-Host "Uninstalling PwGen Password Manager..." -ForegroundColor Yellow

# Remove installation directory
if (Test-Path "$installDir") {
    Remove-Item "$installDir" -Recurse -Force
    Write-Host "Removed installation files" -ForegroundColor Green
}

# Remove from PATH
`$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if (`$userPath -like "*$installDir*") {
    `$newPath = `$userPath -replace [regex]::Escape(";$installDir"), ""
    `$newPath = `$newPath -replace [regex]::Escape("$installDir;"), ""
    `$newPath = `$newPath -replace [regex]::Escape("$installDir"), ""
    [Environment]::SetEnvironmentVariable("PATH", `$newPath, "User")
    Write-Host "Removed from PATH" -ForegroundColor Green
}

# Remove shortcuts
`$shortcuts = @(
    "`$env:APPDATA\Microsoft\Windows\Start Menu\Programs\PwGen Password Manager.lnk",
    "`$env:USERPROFILE\Desktop\PwGen Password Manager.lnk"
)

foreach (`$shortcut in `$shortcuts) {
    if (Test-Path `$shortcut) {
        Remove-Item `$shortcut -Force
        Write-Host "Removed shortcut: `$shortcut" -ForegroundColor Green
    }
}

Write-Host "PwGen uninstalled successfully!" -ForegroundColor Green
Write-Host "Note: User data in %LOCALAPPDATA%\pwgen\ was preserved" -ForegroundColor Blue

pause
"@

    $uninstallerScript | Out-File -FilePath $uninstallerPath -Encoding UTF8
    Log-Info "Uninstaller created at $uninstallerPath"
}

# Main installation function
function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    Write-Host "======================================" -ForegroundColor Cyan
    Write-Host "  PwGen Password Manager Installer" -ForegroundColor Cyan
    Write-Host "======================================" -ForegroundColor Cyan
    Write-Host
    
    if (-not (Test-ExecutionPolicy)) {
        exit 1
    }
    
    Log-Info "Starting installation process..."
    
    Install-Chocolatey
    Install-Rust
    Install-Dependencies
    Build-Install
    Create-Uninstaller
    
    Write-Host
    Write-Host "======================================" -ForegroundColor Cyan
    Log-Success "Installation completed successfully!"
    Write-Host "======================================" -ForegroundColor Cyan
    Write-Host
    Log-Info "You can now:"
    Write-Host "  • Find 'PwGen Password Manager' in the Start Menu" -ForegroundColor White
    Write-Host "  • Run 'pwgen-gui' from Command Prompt or PowerShell" -ForegroundColor White
    Write-Host "  • Run 'pwgen-cli --help' for command-line usage" -ForegroundColor White
    if (-not $NoDesktopShortcut) {
        Write-Host "  • Use the desktop shortcut" -ForegroundColor White
    }
    Write-Host "  • Run the uninstaller from the installation directory" -ForegroundColor White
    Write-Host
    Log-Info "Note: You may need to restart your command prompt to use the new PATH"
}

# Run main function
try {
    Main
}
catch {
    Log-Error "Installation failed: $($_.Exception.Message)"
    exit 1
}