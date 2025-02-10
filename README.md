# Installation

## macOs

```shell
curl -L -o moonshine https://github.com/moonshine-software/installer/releases/download/0.1.0/installer_0.1.0_x86_64-apple-darwin.zip
chmod +x moonshine
sudo mv moonshine /usr/local/bin/
```

## macOs Apple Silicon

```shell
curl -L -o moonshine https://github.com/moonshine-software/installer/releases/download/0.1.0/installer_0.1.0_aarch64-apple-darwin.zip
chmod +x moonshine
sudo mv moonshine /usr/local/bin/
```

## Linux

```shell
curl -L -o moonshine https://github.com/moonshine-software/installer/releases/download/0.1.0/installer_0.1.0_x86_64-unknown-linux-musl.tar.gz
chmod +x moonshine
sudo mv moonshine /usr/local/bin/
```

## Windows

```shell
Invoke-WebRequest -Uri "https://github.com/moonshine-software/installer/releases/download/0.1.0/installer_0.1.0_x86_64-pc-windows-gnu.zip" -OutFile "moonshine.zip"
Expand-Archive -Path moonshine.zip -DestinationPath C:\moonshine-installer
[System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\moonshine-installer", [System.EnvironmentVariableTarget]::Machine)
```

## Get started

```shell
moonshine new project-name
```