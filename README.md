<h1 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/82762656-18de0180-9db7-11ea-9676-ee6fcae615a1.png">
  <br />
 WSL Guide
</h1>

#### A guide for using WSL and all the tools/utilities that will make you a better and more efficient WSL developer.

# Table of Contents

1. [WSL Learning Resources](https://github.com/mikeroyal/WSL-Guide/blob/master/README.md#wsl-learning-resources)

2. [WSL Tools & Projects](https://github.com/mikeroyal/WSL-Guide/blob/master/README.md#wsl-tools--projects)

<img src="https://user-images.githubusercontent.com/45159366/107585931-e6c63700-6bb3-11eb-8f25-f07f8ff05688.png">


# WSL Learning Resources

[What is the Windows Subsystem for Linux?](https://docs.microsoft.com/en-us/windows/wsl/about)

[WSL 2 Linux Kernel on GitHub](https://github.com/microsoft/WSL2-Linux-Kernel) is the source for the Linux kernel used in Windows Subsystem for Linux 2 (WSL2).

[WSLConf](https://www.youtube.com/playlist?list=PLwFSk464RMxnZkvZ1HKrlNyj-s6Zq4fWe) is a community-initiated event on all things Windows Subsystem for Linux and WSL-related.

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
