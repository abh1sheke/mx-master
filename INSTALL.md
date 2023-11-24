# Install MX Master
Here are the steps to install MX Master on your machine.

## Building from source
### Installing Node.js
Download the installer from the [Node.js](https://nodejs.org/en/download) website and follow along to install it.
### Setting up Tauri
#### Windows
- You will need to install Microsoft Visual Studio C++ build tools. The easiest way is to install [Build Tools for Visual Studio 2022](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
When asked which workloads to install, ensure "C++ build tools" and the Windows 10 SDK are selected.
- Tauri heavily depends on WebView2 to render web content on Windows, therefore you must have WebView2 installed. The easiest way is to download and run the Evergreen Bootstrapper
  from [Microsoft's website](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section).
> On Windows 10 (Version 1803 and later with all updates applied) and Windows 11, the WebView2 runtime is distributed as part of the operating system.
- Lastly, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) to install rustup (the Rust installer). Note that you have to restart your terminal, and in some cases, Windows itself, for the changes to take effect.
> Alternatively, you could use winget to install rustup using the following command in PowerShell: <p>``winget install --id Rustlang.Rustup``</p>

> <h3>⚠️ MSVC TOOLCHAIN AS DEFAULT ⚠️</h3>
> For full support for Tauri and tools like trunk make sure the MSVC Rust toolchain is the selected default host triple in the installer dialog. Depending on your system it should be either x86_64-pc-windows-msvc, i686-pc-windows-msvc, or aarch64-pc-windows-msvc.
> If you already have Rust installed, you can make sure the correct toolchain is installed by running this command: 
> <p><code>rustup default stable-msvc</code></p>

#### macOS
- You will need to install CLang and macOS development dependencies. To do this, run the following command in your terminal:
```
xcode-select --install
```
- To install Rust on macOS, open a terminal and enter the following command:
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
> Make sure to restart your terminal for the changes to take effect.

#### Linux
Find out how to setup tauri for Linux [here](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux).

### Building
- Clone the repo
```
git clone https://github.com/abh1sheke/mx-master
```
- Install node dependencies
```
npm install
```
- To build and bundle your whole Tauri application into a single installer simply run the following command:
```
npm run tauri build
```

#### Additional platform specific configuration:
- [Windows](https://tauri.app/v1/guides/building/windows)
- [macOS](https://tauri.app/v1/guides/building/macos)
- [Linux](https://tauri.app/v1/guides/building/linux)
