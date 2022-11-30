<div align="center">
  <h1>ruscode</h1>

  <p>
    A visual studio code's workspaces organizer written in Rust<br />Supports Linux, and Windows.
  </p>


  <p align="center" >
    <a href="https://github.com/MissterHao/ruscode" style="margin-right: 8px;">
      <img
        src="https://img.shields.io/github/issues/MissterHao/ruscode?style=for-the-badge" alt="Github Issue badge" />
    </a>
    <a href="https://github.com/MissterHao/ruscode/actions/workflows/release.yml" style="margin-right: 8px;">
      <img
        src="https://img.shields.io/github/workflow/status/MissterHao/ruscode/release?style=for-the-badge" alt="Github Release Action status badge" />
    </a>
    <a href="https://codecov.io/gh/MissterHao/ruscode" style="margin-right: 8px;">
      <img
        src="https://img.shields.io/codecov/c/gh/MissterHao/ruscode?style=for-the-badge&token=8TU7FS0R56&logo=Codecov" alt="codecov" >
    </a>
    <a href="https://github.com/MissterHao/ruscode">
      <img
        src="https://img.shields.io/badge/Language-Rust-%23EB6400?style=for-the-badge&logo=Rust" alt="Rust badge" >
    </a>
    <br>
    <a href="https://github.com/MissterHao/ruscode" style="margin-right: 8px;">
      <img
        src="https://img.shields.io/github/license/MissterHao/ruscode?style=for-the-badge" alt="Lience badge" />
    </a>
    <a href="https://github.com/MissterHao/ruscode" style="margin-right: 8px;">
      <img
        src="https://img.shields.io/github/downloads/MissterHao/ruscode/total?style=for-the-badge&logo=Rust" alt="Rust badge" >
    </a>

   <br>
    <a href="https://github.com/MissterHao/ruscode">
      <img
        src="https://img.shields.io/badge/Window Version-Latest-blue?style=for-the-badge&logo=Windows" alt="Rust supported badge" >
    </a>
    <a href="https://github.com/MissterHao/ruscode">
      <img
        src="https://img.shields.io/badge/Ubuntu%20Version-Latest-blue?style=for-the-badge&logo=Ubuntu" alt="Ubuntu supported badge" >
    </a>
    <!-- <a href="https://github.com/MissterHao/ruscode">
      <img
        src="https://img.shields.io/badge/MacOS%20Version-Latest-blue?style=for-the-badge&logo=macOS" alt="macOS supported badge" >
    </a> -->
    <br>
        
  </p>
</div>


## What is a workspaces organizer?

After months of or even years of hard working, have you notice that there are tons of folder which are open by vscode before?  
You only want to find out a small experimental project in the workspaces history list but it hard to find because of the numbers of list. 

**Ruscode** is the best soluation for you! 

You can give your workspace tags to help you manage your workspaces.   
You can search your workspaces by path, by folder name, or by tags which you gave before.  
You can use terminal-UI application with beautiful color theme without hurting your eyes.  

Awesome!

## Table of contents

- [🎯 Features](#-features)
- [📦 How to install](#-how-to-install)
- [🏹 Usage](#-usage)
- [📜 Licence](#-licence)
- [✨ Creator](#-creator)
- [🌈 Contributors](#-contributors)
- [🌟 Star History](#-star-history)

## 🎯 Features

There are two mode in management page:
+ Search Mode
+ Detail Mode

You can use arrow key to change between these two modes.  
Also, you can find more detail of help text in the middle of screen.
<!-- A GIF to explain how to change mode -->

### Search Mode

Just type anything to search your workspaces!  

ruscode will use the searching text that you typed to filter any workspaces matched. Current version ( v1.0.0 ) support searching by path. In the future version, ruscode will supported tag-searching soon.  

### Detail Mode

You can enter detail mode by using right arrow key on the selected workspace.

In detail mode, you can find out more detail information about the selected workspace and information of folder also.  

## 📦 How to install

### Prerequisite

ruscode is a utility for **[Visual Studio Code](https://code.visualstudio.com/download)**. Therefore, you must install Visual Studio Code.  

Make sure that the path of Visual Studio Code binary file is in the system path, so that ruscode can use `code <workspace path>` while you hitting `enter` key.

### Cargo
+ `cargo install ruscode`

### Download

You can also download release binary from [github release page](https://github.com/MissterHao/ruscode/releases).

#### Ubuntu

If you are a Ubuntu user
```bash
# Use curl to download binary file
# curl -sSLJO https://github.com/MissterHao/ruscode/releases/download/v1.0.0/ubuntu-latest-binary.zip
# Use wget to download binary file
wget --no-check-certificate --content-disposition https://github.com/MissterHao/ruscode/releases/download/v1.0.0/ubuntu-latest-binary.zip -q

unzip ubunti-latest-binary.zip
chmod +x ruscode
./ruscode
```

#### Windows

If you are a Windows user

1. Download zip file from [release page](https://github.com/MissterHao/ruscode/releases).
2. Unzip it!
3. double click the ruscode.exe file or use CLI to open it.


## 🏹 Usage
```
ruscode - A visual studio code's workspaces organizer written in Rust

Usage: ruscode.exe [OPTIONS]

Options:
  -d, --disable-splash-screen  Show splash screen or not
  -h, --help                   Print help information
  -V, --version                Print version information
```

## 📜 Licence
This project is licensed under the [MIT License](https://github.com/MissterHao/ruscode/blob/master/LICENSE)

## ✨ Creator
- [MissterHao ( Hao-Wei )](https://www.linkedin.com/in/hao-wei-li/)

## 🌈 Contributors
- [MissterHao ( Hao-Wei )](https://www.linkedin.com/in/hao-wei-li/)

## 🌟 Star History
[![Star History Chart](https://api.star-history.com/svg?repos=MissterHao/ruscode&type=Date)](https://star-history.com/#MissterHao/ruscode&Date)
