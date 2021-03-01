<h1 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/82762656-18de0180-9db7-11ea-9676-ee6fcae615a1.png">
  <br />
 WSL Guide
</h1>

#### A guide for using WSL and all the tools/utilities that will make you a better and more efficient WSL developer.

# Table of Contents

1. [WSL Learning Resources](https://github.com/mikeroyal/WSL-Guide/blob/master/README.md#wsl-learning-resources)

2. [WSL Tools & Projects](https://github.com/mikeroyal/WSL-Guide/blob/master/README.md#wsl-tools--projects)

3. [Setting up WSL Linux Distributions](https://github.com/mikeroyal/WSL-Guide/blob/master/README.md#setting-up-wsl-linux-distributions)

4. [Setting up macOS Workspace VM](https://github.com/mikeroyal/WSL-Guide/blob/master/README.md#seting-up-macos-workspace-vm)

<img src="https://user-images.githubusercontent.com/45159366/107585931-e6c63700-6bb3-11eb-8f25-f07f8ff05688.png">


# WSL Learning Resources

[Back to the Top]()

[WSL 2 Linux Kernel on GitHub](https://github.com/microsoft/WSL2-Linux-Kernel) is the source for the Linux kernel used in Windows Subsystem for Linux 2 (WSL2).

[WSLConf](https://www.youtube.com/playlist?list=PLwFSk464RMxnZkvZ1HKrlNyj-s6Zq4fWe) is a community-initiated event on all things Windows Subsystem for Linux and WSL-related.

[What is the Windows Subsystem for Linux?](https://docs.microsoft.com/en-us/windows/wsl/about)

[Pro Windows Subsystem for Linux (WSL): Powerful Tools and Practices for Cross-Platform Development and Collaboration Book](https://www.amazon.com/Windows-Subsystem-Linux-Cross-Platform-Collaboration/dp/1484268725/ref=sr_1_1?dchild=1&keywords=Pro+Windows+Subsystem+for+Linux+%28WSL%29&qid=1614379171&s=digital-text&sr=1-1-catcorr)

[Windows Subsystem for Linux 2 (WSL 2) Tips, Tricks, and Techniques Book](https://www.amazon.com/Windows-Subsystem-Linux-Tricks-Techniques-ebook/dp/B08K98C7DB/ref=sr_1_1?dchild=1&keywords=WSL+book&qid=1614379053&s=digital-text&sr=1-1)

[Comparing WSL 2 and WSL 1 ](https://docs.microsoft.com/en-us/windows/wsl/compare-versions)

[GPU accelerated machine learning training in the Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/tutorials/gpu-compute)

[CUDA on Windows Subsystem for Linux(WSL) 2](https://developer.nvidia.com/blog/announcing-cuda-on-windows-subsystem-for-linux-2/)

[Developing in Windows Subsystem for Linux (WSL)](https://code.visualstudio.com/docs/remote/wsl)

[Using WSL 2 with Visual Studio Code](https://code.visualstudio.com/blogs/2019/09/03/wsl2)

[WSLG: X11 and Wayland Applications in WSL](https://linuxplumbersconf.org/event/9/contributions/611/attachments/702/1298/XDC2020_-_X11_and_Wayland_applications_in_WSL.pdf)

[How to run Podman on Windows with WSL 2](https://www.redhat.com/sysadmin/podman-windows-wsl2)

[Create a development container in Visual Studio Code](https://code.visualstudio.com/docs/remote/create-dev-container)

[Getting started with MySQL, MongoDB, PostgreSQL, SQLite, Microsoft SQL Server, or Redis to set up a database on WSL](https://docs.microsoft.com/en-us/windows/wsl/tutorials/wsl-database)

[Setting up SAP HANA, express edition into WSL 2 (Windows Subsystem for Linux)](https://blogs.sap.com/2020/09/30/installing-sap-hana-express-edition-into-wsl2-windows-subsystem-for-linux/)

[Set up your Node.js development environment with WSL 2](https://docs.microsoft.com/en-us/windows/nodejs/setup-on-wsl2)

[Getting started mounting a Linux disk in WSL 2](https://docs.microsoft.com/en-us/windows/nodejs/setup-on-wsl2)

[Using Fedora with Microsoft's WSL 2](https://fedoramagazine.org/wsl-fedora-33/)

# WSL Tools & Projects

[Back to the Top]()

[wslu](https://github.com/wslutilities/wslu) is a collection of utilities for Windows 10 Linux Subsystem, such as retrieving Windows 10 environment variables or creating your favorite Linux GUI application shortcuts on Windows 10 Desktop.

[Ubuntu on WSL](https://wiki.ubuntu.com/WSL) is a wiki guide on getting started with the latest version of Ubuntu installed and setup on WSL for Windows 10.

[Ubuntu on Windows Community Preview](https://www.microsoft.com/en-us/p/ubuntu-on-windows-community-preview/9p9q5zh1hrr0) is a special build of Ubuntu for WSL as a sandbox for testing new features and getting community feedback. This build is intended for early adopters in the WSL community.

[Ubuntu Pro for Azure](https://azuremarketplace.microsoft.com/en-us/marketplace/apps/canonical.0001-com-ubuntu-pro-focal?tab=Overview) is a premium image designed by Canonical optimized for production environments running on Azure. It includes security and compliance services, enabled by default, in a form suitable for small to large-scale Linux enterprise operations — with no contract needed. 

[Azure CLI](https://docs.microsoft.com/en-us/cli/azure/?view=azure-cli-latest) is a set of commands used to create and manage Azure resources. The Azure CLI is available across Azure services and is designed to get you working quickly with Azure, with an emphasis on automation. 

[Visual Studio Code Remote - WSL extension](https://code.visualstudio.com/docs/remote/wsl) lets you use the Windows Subsystem for Linux (WSL) as your full-time development environment right from VS Code. You can develop in a Linux-based environment, use Linux-specific toolchains and utilities, and run and debug your Linux-based applications all from the comfort of Windows. The extension runs commands and other extensions directly in WSL so you can edit files located in WSL or the mounted Windows filesystem (for example /mnt/c) without worrying about pathing issues, binary compatibility, or other cross-OS challenges.

[Visual Studio Code Remote Development and GitHub Codespaces](https://github.com/Microsoft/vscode-dev-containers) is a  repository of development container definitions for the VS Code Remote - Containers extension and GitHub Codespaces. A development container is a running [Docker](https://www.docker.com/) container with a well-defined tool/runtime stack and its prerequisites. The [VS Code Remote Containers](https://aka.ms/vscode-remote/download/containers) extension allows you to clone a repository or open any folder mounted into (or already inside) a dev container and take advantage of VS Code's full development feature set. [GitHub Codespaces](https://github.com/features/codespaces) both use this same concept to quickly create customized, cloud-based development environments accessible from VS Code or the web.

[Windows Terminal](https://github.com/microsoft/terminal) is a new, modern, feature-rich, productive terminal application for command-line users. It includes many of the features most frequently requested by the Windows command-line community including support for tabs, rich text, globalization, configurability, theming & styling, and more.

[PowerShell Core](https://github.com/PowerShell/PowerShell) is a cross-platform (Windows, Linux, and macOS) automation and configuration tool/framework that works well with your existing tools and is optimized for dealing with structured data (e.g. JSON, CSV, XML, etc.), REST APIs, and object models. It includes a command-line shell, an associated scripting language and a framework for processing cmdlets.

[Docker Desktop WSL 2 backend](https://docs.docker.com/docker-for-windows/wsl/) creates an  architectural change that gvies a full Linux kernel built by Microsoft, allowing Linux containers to run natively without emulation. With Docker Desktop running on WSL 2, users can leverage Linux workspaces and avoid having to maintain both Linux and Windows build scripts. In addition, WSL 2 provides improvements to file system sharing, boot time, and allows access to some cool new features for Docker Desktop users.

[Dxgkrnl](https://devblogs.microsoft.com/directx/directx-heart-linux/) is a brand-new kernel driver for Linux that exposes the /dev/dxg device to user mode Linux. /dev/dxg exposes a set of IOCTL that closely mimic the native WDDM D3DKMT kernel service layer on Windows. Dxgkrnl inside of the Linux kernel connects over the VM Bus to its big brother on the Windows host and uses this VM bus connection to communicate with the physical GPU.

[Ansible-WSL](https://github.com/Wintus/Ansible-WSL) is an open source program that makes it easier to provision your Windows from inside of WSL by Ansible.

[WSL-DistroLauncher](https://github.com/Microsoft/WSL-DistroLauncher) is a sample/reference launcher app for WSL distro Microsoft Store packages. 

[Pengwin](https://github.com/WhitewaterFoundry/Pengwin) is a Linux distro optimized for WSL based on Debian. 

[Pengwin Enterprise](https://github.com/WhitewaterFoundry/Pengwin-Enterprise) is an enterprise Linux solution for Windows Subsystem for Linux (WSL) that is compatible with mainstream enterprise Linux distributions.


# Setting up WSL Linux Distributions

[Back to the Top]()

**Requirements**

Before you install any of the Linux Distributions make sure to install/enable the Windows Subsystem for Linux on your Windows 10 machine. Following the instructions below:

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563743-0b1b8200-7a95-11eb-9929-dd39abe63175.png">
<br />
Installing Windows Subsystem Linux
</p>

**Run this command in Powershell:**

```sh
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
```

[Ubuntu 20.04 LTS](https://www.microsoft.com/en-us/p/ubuntu-2004-lts/9n6svws3rx71?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563752-0eaf0900-7a95-11eb-9fd7-9be123b57cc1.png">
</p>


[Debian](https://www.microsoft.com/en-us/p/debian/9msvkqc78pk6?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563781-14a4ea00-7a95-11eb-8008-61867e38cf1e.png">
</p>

[Fedora 33 fo WSL](https://fedoramagazine.org/wsl-fedora-33/)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563789-15d61700-7a95-11eb-9c99-c1e68ead8388.png">
</p>

[SUSE Linux Enterprise Server 15 SP1](https://www.microsoft.com/en-us/p/suse-linux-enterprise-server-15-sp1/9pn498vpmf3z?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563809-1bcbf800-7a95-11eb-8be7-e94b2feb74a7.png">
</p>


[openSUSE Leap 15.2](https://www.microsoft.com/en-us/p/opensuse-leap-152/9mzd0n9z4m4h?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563802-1a9acb00-7a95-11eb-9c6a-14bbcde765dd.png">
</p>

[Kali Linux](https://www.microsoft.com/en-us/p/kali-linux/9pkr34tncv07?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563822-1ec6e880-7a95-11eb-8d4b-a051178da3e2.png">
</p>

[Pengwin](https://www.microsoft.com/en-us/p/pengwin/9nv1gv1pxz6p?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563833-225a6f80-7a95-11eb-93e8-f40799469bcc.png">
</p>


[Fedora Remix for WSL](https://www.microsoft.com/en-us/p/fedora-remix-for-wsl/9n6gdm4k2hnc?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563839-238b9c80-7a95-11eb-8a41-fc9775c810bb.png">
</p>


[GWSL](https://www.microsoft.com/en-us/p/gwsl/9nl6kd1h33v3?activetab=pivot:overviewtab) is an XServer that lets you easily run graphical Linux apps on Windows 10.

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563870-2e463180-7a95-11eb-93b1-77c1ff0ef39d.png">
</p>

# Setting up a macOS Workspace VM

[Back to the Top]()

**REQUIREMENTS**

   - [Ubuntu 20.04 LTS for WSL](https://www.microsoft.com/en-us/p/ubuntu-2004-lts/9n6svws3rx71?activetab=pivot:overviewtab)
   - QEMU > 2.11.1
   - A CPU with Intel VT-x / AMD SVM support is required
   - A CPU with SSE4.1 support is required for >= macOS Sierra
   - A CPU with AVX2 support is required for >= macOS Mojave
   - Internet access for the installation process

**Open the terminal and run: **
```sh
sudo apt install qemu uml-utilities virt-manager dmg2img git wget libguestfs-tools p7zip
```

[OpenCore for macOS](https://dortania.github.io/OpenCore-Install-Guide/)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/109563872-2f775e80-7a95-11eb-9b8a-0afe8cd4627f.png">
 </p>
