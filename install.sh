#!/bin/bash

VERSION=0.1.0
OS=$(uname -s)
ARCH=$(uname -m)

if [[ "$OS" == "Linux" ]]; then
    ARCH="x86_64-unknown-linux-musl"
elif [[ "$OS" == "Darwin" && "$ARCH" == "arm64" ]]; then
    ARCH="aarch64-apple-darwin"
elif [[ "$OS" == "Darwin" ]]; then
    ARCH="x86_64-apple-darwin"
elif [[ "$OS" == "MINGW64_NT"* || "$OS" == "MSYS_NT"* || "$OS" == "CYGWIN_NT"* ]]; then
    echo "Detected Windows. Switching to PowerShell installer..."
    pwsh -Command "& { iwr -useb https://raw.githubusercontent.com/moonshine-software/installer/main/install.ps1 | iex }"
    exit 0
else
    echo "Unsupported OS: $OS"
    exit 1
fi

echo "Downloading MoonShine Installer for $OS ($ARCH)..."
curl -L -o moonshine.zip "https://github.com/moonshine-software/installer/releases/download/$VERSION/installer_${VERSION}_${ARCH}.zip"

echo "Extracting..."
unzip moonshine.zip
chmod +x installer_${VERSION}_${ARCH}
sudo mv installer_${VERSION}_${ARCH} /usr/local/bin/moonshine
rm moonshine.zip

echo "✅ Installation complete! Run 'moonshine new my-project'"
