<h1 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/146082215-7d80b8b7-4da6-4280-900d-4ce4cd30df7e.png">
  <br />
 WSL Guide
</h1>

<a href="https://github.com/mikeroyal?tab=followers">
         <img alt="followers" title="Follow me for Updates" src="https://custom-icon-badges.demolab.com/github/followers/mikeroyal?color=236ad3&labelColor=1155ba&style=for-the-badge&logo=person-add&label=Follow&logoColor=white"/></a> 	

![Maintenance](https://img.shields.io/maintenance/yes/2023?style=for-the-badge)
![Last-Commit](https://img.shields.io/github/last-commit/mikeroyal/wsl-guide?style=for-the-badge)

#### A guide for using WSL and all the tools/utilities that will make you a better and more efficient WSL developer.

**Note: You can easily convert this markdown file to a PDF in [VSCode](https://code.visualstudio.com/) using this handy extension [Markdown PDF](https://marketplace.visualstudio.com/items?itemName=yzane.markdown-pdf).**

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/146692531-b811593e-031d-451a-8807-746363ad08a9.png">
  <br />
</p>

# Table of Contents

1. [Getting Started with WSL](https://github.com/mikeroyal/WSL-Guide#getting-started-with-wsl)
    * [Developer Resources](https://github.com/mikeroyal/WSL-Guide#developer-resources)
    * [Books](https://github.com/mikeroyal/WSL-Guide#books)
    * [YouTube Videos](#youtube-videos)
    * [Creating Backup & Restore Images in WSL 2](https://github.com/mikeroyal/WSL-Guide#creating-backup--restore-images-in-wsl-2)
    * [Setting up Zsh and Oh My Zsh in WSL 2](https://github.com/mikeroyal/WSL-Guide#setting-up-zsh-and-oh-my-zsh-in-wsl-2)
    * [Linux Software Package Formats & Package Managers](https://github.com/mikeroyal/WSL-Guide#linux-software-package-formats--package-managers)
    * [Linux Kernel](#linux-kernel)
    * [File systems](https://github.com/mikeroyal/WSL-Guide#file-systems)
    * [Systemd](https://github.com/mikeroyal/WSL-Guide#systemd)  
    * [Init](https://github.com/mikeroyal/WSL-Guide#init)

2. [WSL Tools & Projects](https://github.com/mikeroyal/WSL-Guide#wsl-tools--projects)

3. [Setting up WSL Linux Distributions](https://github.com/mikeroyal/WSL-Guide#setting-up-wsl-linux-distributions)

4. [Azure Development](https://github.com/mikeroyal/WSL-Guide#azure-development)

5. [Amazon Web Services (AWS)](https://github.com/mikeroyal/WSL-Guide#amazon-web-services-aws)

6. [Google Cloud](https://github.com/mikeroyal/WSL-Guide#google-cloud-platform-gcp)

7. [Kubernetes](https://github.com/mikeroyal/WSL-Guide#kubernetes)
    * [Red Hat CodeReady Containers (CRC) OpenShift](https://github.com/mikeroyal/WSL-Guide#Red-Hat-CodeReady-Containers-CRC)
    * [Setting up Podman](https://github.com/mikeroyal/WSL-Guide#setting-up-podman)
    * [Setting up Buildah](https://github.com/mikeroyal/WSL-Guide#setting-up-buildah)
    * [Installing Kubernetes on WSL with Rancher Desktop](https://github.com/mikeroyal/WSL-Guide#installing-kubernetes-on-wsl-with-rancher-desktop)
    * [Installing Kubernetes on WSL with Docker Desktop](https://github.com/mikeroyal/WSL-Guide#installing-kubernetes-on-wsl-with-docker-desktop)
    * [Installing Kubernetes on WSL with Microk8s](https://github.com/mikeroyal/WSL-Guide#installing-kubernetes-on-wsl-with-microk8s)
   
8. [PowerShell Development](https://github.com/mikeroyal/WSL-Guide#powershell-development)

9. [Wayland Development](https://github.com/mikeroyal/WSL-Guide#wayland-development)

10. [Networking](https://github.com/mikeroyal/WSL-Guide#networking)

11. [Databases](https://github.com/mikeroyal/WSL-Guide#databases)

12. [Setting up macOS Workspace VM](https://github.com/mikeroyal/WSL-Guide#setting-up-a-macos-workspace-vm)


# Getting Started with WSL

[Back to the Top](https://github.com/mikeroyal/WSL-Guide/blob/master/README.md#table-of-contents)

[Back to the Top](#table-of-contents)

<img src="https://user-images.githubusercontent.com/45159366/107585931-e6c63700-6bb3-11eb-8f25-f07f8ff05688.png">

### Developer Resources

 * [What is the Windows Subsystem for Linux?](https://docs.microsoft.com/en-us/windows/wsl/about)

 * [WSL 2 Linux Kernel on GitHub](https://github.com/microsoft/WSL2-Linux-Kernel) is the source for the Linux kernel used in Windows Subsystem for Linux 2 (WSL2).
 
 * [Developing in Windows Subsystem for Linux (WSL) 2](https://code.visualstudio.com/docs/remote/wsl)

 * [Using WSL 2 with Visual Studio Code](https://code.visualstudio.com/blogs/2019/09/03/wsl2)
 
 * [GnuPG Tool for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=JHeilingbrunner.vscode-gnupg-tool)

 * [GPU accelerated machine learning training in the Windows Subsystem for Linux](https://docs.microsoft.com/en-us/windows/wsl/tutorials/gpu-compute)

 * [CUDA on Windows Subsystem for Linux(WSL) 2](https://developer.nvidia.com/blog/announcing-cuda-on-windows-subsystem-for-linux-2/)

* [Set up a Linux Service Fabric cluster via WSL2 on your Windows developer machine](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-local-linux-cluster-windows-wsl2)
 
 * [WSLG: X11 and Wayland Applications in WSL](https://linuxplumbersconf.org/event/9/contributions/611/attachments/702/1298/XDC2020_-_X11_and_Wayland_applications_in_WSL.pdf)

* [How to run Podman on Windows with WSL 2](https://www.redhat.com/sysadmin/podman-windows-wsl2)

* [Create a development container in Visual Studio Code](https://code.visualstudio.com/docs/remote/create-dev-container)

* [Getting started with MySQL, MongoDB, PostgreSQL, SQLite, Microsoft SQL Server, or Redis to set up a database on WSL](https://docs.microsoft.com/en-us/windows/wsl/tutorials/wsl-database)

* [Setting up SAP HANA, express edition into WSL 2 (Windows Subsystem for Linux)](https://blogs.sap.com/2020/09/30/installing-sap-hana-express-edition-into-wsl2-windows-subsystem-for-linux/)

* [Set up your Node.js development environment with WSL 2](https://docs.microsoft.com/en-us/windows/nodejs/setup-on-wsl2)

* [Getting started mounting a Linux disk in WSL 2](https://docs.microsoft.com/en-us/windows/nodejs/setup-on-wsl2)

* [Using Fedora with Microsoft's WSL 2](https://fedoramagazine.org/wsl-fedora-33/)
 
 ### Books
 
 [Back to the Top](#table-of-contents)

* [Pro Windows Subsystem for Linux (WSL): Powerful Tools and Practices for Cross-Platform Development and Collaboration Book by Hayden Barnes](https://www.amazon.com/Windows-Subsystem-Linux-Cross-Platform-Collaboration/dp/1484268725/ref=sr_1_1?dchild=1&keywords=Pro+Windows+Subsystem+for+Linux+%28WSL%29&qid=1614379171&s=digital-text&sr=1-1-catcorr)

* [Windows Subsystem for Linux 2 (WSL 2) Tips, Tricks, and Techniques Book by Stuart Leeks](https://www.amazon.com/Windows-Subsystem-Linux-Tricks-Techniques-ebook/dp/B08K98C7DB/ref=sr_1_1?dchild=1&keywords=WSL+book&qid=1614379053&s=digital-text&sr=1-1)

### YouTube videos

[Back to the Top](#table-of-contents)
 
[![I Coded with WSL2 for a Week](https://ytcards.demolab.com/?id=LktFP0Dpl-c&lang=en&background_color=%230d1117&title_color=%23ffffff&stats_color=%23dedede&width=240 "I Coded with WSL2 for a Week")](https://www.youtube.com/watch?v=LktFP0Dpl-c)
[![How to Install Ubuntu on Windows 11 (WSL)](https://ytcards.demolab.com/?id=wjbbl0TTMeo&lang=en&background_color=%230d1117&title_color=%23ffffff&stats_color=%23dedede&width=240 "How to Install Ubuntu on Windows 11 (WSL)")](https://www.youtube.com/watch?v=wjbbl0TTMeo)
[![Ubuntu Desktop/GUI Apps on WSL | Updated Guide](https://ytcards.demolab.com/?id=7Sym3uL6YWo&lang=en&background_color=%230d1117&title_color=%23ffffff&stats_color=%23dedede&width=240 "Ubuntu Desktop/GUI Apps on WSL | Updated Guide")](https://www.youtube.com/watch?v=7Sym3uL6YWo)
[![How to install and get started with WSL 2 on Windows 11](https://ytcards.demolab.com/?id=YByZ_sOOWsQ&lang=en&background_color=%230d1117&title_color=%23ffffff&stats_color=%23dedede&width=240 "How to install and get started with WSL 2 on Windows 11")](https://www.youtube.com/watch?v=YByZ_sOOWsQ)
[![Windows development setup with WSL2, ZSH, VSCode, and more](https://ytcards.demolab.com/?id=oF6gLyhQDdw&lang=en&background_color=%230d1117&title_color=%23ffffff&stats_color=%23dedede&width=240 "Windows development setup with WSL2, ZSH, VSCode, and more")](https://www.youtube.com/watch?v=oF6gLyhQDdw)
[![Docker Complete Setup on Windows (With WSL Ubuntu)](https://ytcards.demolab.com/?id=2ezNqqaSjq8&lang=en&background_color=%230d1117&title_color=%23ffffff&stats_color=%23dedede&width=240 "Docker Complete Setup on Windows (With WSL Ubuntu)")](https://www.youtube.com/watch?v=2ezNqqaSjq8)

### Creating Backup & Restore Images in WSL 2

[Back to the Top](#table-of-contents)

List the images that you have installed. You will need to know the name of the image that you want to backup. To do that, you can use the command in **PowerShell(Admin)**:

```wsl --list```

Now that you have listed out the image you want to backup using the export functionality. Make sure that you don’t have any WSL command environments open. You can also shutdown WSL instances using:

```wsl --shutdown```

To perform the backup/export, the syntax is as follows:

```wsl --export <Image Name> <Export location file name.tar>```


You can take a look in the folder you specify for the backup to see the resulting backup file in the form of a .tar file. 


The command you use to import the WSL2 image back into WSL2, either on the same machine or a different machine is as follows:

```wsl --import <Image Name> <Directory where you want to store the imported image> <Directory where the exported .tar file exists>```

### Setting up Zsh and Oh My Zsh in WSL 2

[Back to the Top](#table-of-contents)

* **[Zsh (Z shell)](https://www.zsh.org/)** is an extended version of the Bourne Shell (sh) that includes plenty of new features, and support for plugins and themes.

* **[Oh My Zsh](https://ohmyz.sh/)** is an open source, community-driven framework for managing your zsh configuration. It provides thousands of helpful functions, helpers, plugins, themes.

**Install Zsh and Oh My Zsh:**

```sudo apt update```

``` sudo apt upgrade```

```sudo apt install git zsh -y```

```sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh-master/tools/install.sh)"```

It should prompt you and ask if you want to enable zsh. **Enter 'Y' to continue**.

<p align="center">
  <img src="https://user-images.githubusercontent.com/45159366/192134414-65fc8e87-0d73-47e4-88a1-eb7070cf4136.png">
  <br />
</p>

**Note:** If you have trouble getting Zsh to start you may want to try the following:

```zsh```

```chsh -s $(which zsh)```

**Adding Aliases to Zsh**

**To add aliases, you’ll have to edit the ```.zshrc file``` in your home directory. For this, run the following command to open up the file:**

```vi ~/.zshrc```

**This will open the file in your editor. Scroll to the end of the file and add the following lines:**

```alias gd="git diff"```

```alias gcmsg="git commit -m"```

```alias gitc="git checkout"```

```alias gitm="git checkout master"```
          
### Linux Software Package Formats & Package Managers
[Back to the Top](#table-of-contents)

[dpkg(Debian Package)](https://www.digitalocean.com/community/tutorials/dpkg-command-in-linux) is the package management system in Debian and its OS derivatives. 

[DEB](https://www.debian.org/distrib/packages) is a Debian Software Package file used on Debian-based Linux systems such Debian, Ubuntu, Linux Mint, and Pop!_OS.

[APT (Advanced Package Tool)](https://en.wikipedia.org/wiki/APT_(software)) is a higher-level package management systemtool, that is more commonly used than dpkg as it can fetch packages from remote locations.

[TDNF(Tiny Dandified) package manager](https://microsoft.github.io/CBL-Mariner/docs/) is a C based successor of the DNF package manager, which itself is the successor to Fedora’s YUM package manager. TDNF is included in the base CBL-Mariner image by default.

[DNF(Dandified Packaging Tool)](https://docs.fedoraproject.org/en-US/quick-docs/dnf/) is a software package manager that installs, updates, and removes packages on Fedora and is the successor to YUM (Yellow-Dog Updater Modified). DNF makes it easy to maintain packages by automatically checking for dependencies and determines the actions required to install packages.

[Micro DNF](https://fedoraproject.org/wiki/Changes/MajorUpgradeOfMicrodnf) is a lightweight C implementation of DNF, designed to be used for doing simple packaging actions when you don't need full-blown DNF and you want the tiniest useful environments possible. Checkout the [Micro DNF GitHub](https://github.com/rpm-software-management/microdnf).

[RPM Package Manager (RPM)](https://rpm.org/) is a powerful package management system capable of building computer software from source into easily distributable packages installing, updating and uninstalling packaged software querying detailed information about the packaged software, whether installed or not.

[EPEL (Extra Packages for Enterprise Linux)](https://docs.fedoraproject.org/en-US/epel/) is an free and open source community-based repository project from the Fedora team which provides 100% high-quality add-on software packages for Linux distribution including RHEL (Red Hat Enterprise Linux) and CentOS Stream.

[YUM (Yellowdog Updater, Modified)](https://www.digitalocean.com/community/tutorials/what-is-yum) is a software package-management utility for Linux operating system using the RPM Package Manager.

[ROM OSTree](https://ostreedev.github.io/ostree/) is a hybrid image/package system. It combines libostree as a base image format, and accepts RPM on both the client and server side, sharing code with the dnf project; specifically libdnf. Thus bringing many of the benefits of both projects together.

[YaST](https://yast.opensuse.org/) is a installation and configuration tool for openSUSE and the SUSE Linux Enterprise distributions. It features an easy-to-use interface and powerful configuration capabilities.

[Zypper](https://software.opensuse.org/package/zypper) is a command line package manager which makes use of libzypp. It provides functions like repository access, dependency solving, package installation, etc. Zypper repositories are similar to the ones used in YaST, which also makes use of libzypp.

[Pacman](https://archlinux.org/pacman/) is a utility which manages software packages in Arch Linux. It uses simple compressed files as a package format, and maintains a text-based package database (more of a hierarchy), just in case some hand tweaking is necessary. 

### Linux Kernel

[Back to the Top](#table-of-contents)

The Linux kernel is the main component of a Linux operating system (OS) and is the core interface between a computer’s hardware and its processes. It communicates between the 2, managing resources as efficiently as possible.

 * [WSL2-Linux-Kernel](https://github.com/microsoft/WSL2-Linux-Kernel), is a repo contains the kernel source code and configuration files for the WSL2 kernel.
 

<p align="center">
  <img src="https://user-images.githubusercontent.com/45159366/219989150-5b6d5f1d-e7f8-4c5e-8fb9-5359f1acc54d.png">
  <br />
Linux kernel layout
</p>

**Categories for Linux Kernel Releases:**

* **Prepatch:** Prepatch or "RC(Realease Candidate)" kernels are mainline kernel pre-releases that are mostly aimed at other kernel developers and Linux enthusiasts. They must be compiled from source and usually contain new features that must be tested before they can be put into a stable release. Prepatch kernels are maintained and released by Linus Torvalds.
    
* **Mainline:** Mainline tree is maintained by [Linus Torvalds](https://en.wikipedia.org/wiki/Linus_Torvalds). It's the tree where all new features are introduced and where all the exciting new development happens. New mainline kernels are released every 9-10 weeks.
    
* **Stable:** After each mainline kernel is released, it is considered "stable." Any bug fixes for a stable kernel are backported from the mainline tree and applied by a designated stable kernel maintainer. There are usually only a few bugfix kernel releases until next mainline kernel becomes available -- unless it is designated a "longterm maintenance kernel." Stable kernel updates are released on as-needed basis, usually once a week.
    
* **Longterm:** There are usually several "longterm maintenance" kernel releases provided for the purposes of backporting bugfixes for older kernel trees. Only important bugfixes are applied to such kernels and they don't usually see very frequent releases, especially for older trees. 

**Current Longterm release kernels**
 
|Version |Maintainer |	Released |Projected EOL|
|------|------|-----|-----|
|6.6 |	Greg Kroah-Hartman & Sasha Levin |	2023-10-29 |	Dec, 2026|
|6.1 |Greg Kroah-Hartman & Sasha Levin |2022-12-11 |Dec, 2026|
|5.15 |Greg Kroah-Hartman & Sasha Levin |2021-10-31 |Oct, 2026|
|5.10 |Greg Kroah-Hartman & Sasha Levin |2020-12-13 |Dec, 2026|
|5.4 |Greg Kroah-Hartman & Sasha Levin 	|2019-11-24 |Dec, 2025|
|4.19 |Greg Kroah-Hartman & Sasha Levin |2018-10-22 |Dec, 2024|

### File systems

[Back to the Top](#table-of-contents)

[GlusterFS](https://www.gluster.org/) is a free and open source scalable network filesystem. Gluster is a scalable network filesystem. Using common off-the-shelf hardware, you can create large, distributed storage solutions for media streaming, data analysis, and other data- and bandwidth-intensive tasks.

[Ceph](https://ceph.io/) is a software-defined storage solution designed to address the object, block, and file storage needs of data centers adopting open source as the new norm for high-growth block storage, object stores and data lakes.

[Hadoop Distributed File System (HDFS)](https://www.ibm.com/analytics/hadoop/hdfs) is a distributed file system that handles large data sets running on commodity hardware. It is used to scale a single Apache Hadoop cluster to hundreds (and even thousands) of nodes. HDFS is one of the major components of Apache Hadoop, the others being [MapReduce](https://www.ibm.com/analytics/hadoop/mapreduce) and [YARN](https://hadoop.apache.org/docs/current/hadoop-yarn/hadoop-yarn-site/YARN.html).

[ZFS](https://docs.oracle.com/cd/E19253-01/819-5461/zfsover-2/) is an enterprise-ready open source file system and volume manager with unprecedented flexibility and an uncompromising commitment to data integrity.

[OpenZFS](https://openzfs.org/wiki/Main_Page ) is an open-source storage platform. It includes the functionality of both traditional file systems and volume manager. It has many advanced features including:

  - Protection against data corruption.
  - Integrity checking for both data and metadata.
  - Continuous integrity verification and automatic "self-healing" repair.

[Btrfs](https://btrfs.wiki.kernel.org/index.php/Main_Page) is a modern copy on write (CoW) filesystem for Linux aimed at implementing advanced features while also focusing on fault tolerance, repair and easy administration. Its main features and benefits are:

  * Snapshots which do not make the full copy of files
  * RAID - support for software-based RAID 0, RAID 1, RAID 10
  * Self-healing - checksums for data and metadata, automatic detection of silent data corruptions
  
[MergerFS](https://github.com/trapexit/mergerfs) is a union filesystem geared towards simplifying storage and management of files across numerous commodity storage devices. It is similar to mhddfs, unionfs, and aufs.
  
[Bcachefs](https://bcachefs.org/) is an advanced new filesystem for Linux, with an emphasis on reliability and robustness and the complete set of features one would expect from a modern filesystem. Scalability has been tested to 50+ TB, will eventually scale far higher. 

[Ext4](https://ext4.wiki.kernel.org/index.php/Ext4_Howto) is a journaling file system for Linux, developed as the successor to ext3

[Squashfs](https://www.kernel.org/doc/html/latest/filesystems/squashfs.html) is a compressed read-only filesystem for Linux. It uses zlib, lz4, lzo, or xz compression to compress files, inodes and directories. Inodes in the system are very small and all blocks are packed to minimize data overhead.

[NTFS(New Technology File System)](https://docs.microsoft.com/en-us/windows-server/storage/file-server/ntfs-overview) is the primary file system for recent versions of Windows and Windows Server—provides a full set of features including security descriptors, encryption, disk quotas, and rich metadata, and can be used with Cluster Shared Volumes (CSV) to provide continuously available volumes that can be accessed simultaneously from multiple nodes of a failover cluster.

[exFAT(Extended File Allocation Table )](https://docs.microsoft.com/en-us/windows/win32/fileio/exfat-specification) is the file system that was the successor to FAT32 in the FAT family of file systems. It was optimized for flash memory such as USB flash drives and SD cards.

### Systemd
[Back to the Top](#table-of-contents)

**[Systemd support is now available in WSL.](https://devblogs.microsoft.com/commandline/systemd-support-is-now-available-in-wsl/)**

**WSL Systemd requirements:** Windows 11 and a version of WSL 0.67.6 or above. 

[systemd](https://systemd.io/) is a suite of basic building blocks for a Linux system. It provides a system and service manager that runs as PID 1 and starts the rest of the system. It provides aggressive parallelization capabilities, uses socket and D-Bus activation for starting services, offers on-demand starting of daemons, keeps track of processes using Linux control groups, maintains mount and automount points, and implements an elaborate transactional dependency-based service control logic. systemd supports SysV and LSB init scripts and works as a replacement for [SysVinit](https://wiki.archlinux.org/title/SysVinit).

<p align="center">
  <img src="https://user-images.githubusercontent.com/45159366/190265681-2266a967-096f-43be-a7ca-ae48815aa5e2.png">
  <br />
Systemd Overview
</p>

### init

[Back to the Top](#table-of-contents)

[init](https://en.wikipedia.org/wiki/Init) is parent of all Linux processes with PID or process ID of 1. It is the first process to start when a computer boots up and runs until the system shuts down. **init stands for initialization**. 

<p align="center">
  <img src="https://user-images.githubusercontent.com/45159366/190264913-d77974ff-d96f-4092-a6c0-465ecb6d608b.png">
  <br />
Linux Boot Process
</p>

# WSL Tools & Projects

[Back to the Top](https://github.com/mikeroyal/WSL-Guide#table-of-contents)

[CBL-Mariner](https://github.com/microsoft/CBL-Mariner) is an internal Linux distribution for Microsoft’s cloud infrastructure and edge products and services. CBL-Mariner is designed to provide a consistent platform for these devices and services and will enhance Microsoft’s ability to stay current on Linux updates. This initiative is part of Microsoft’s increasing investment in a wide range of Linux technologies, such as [SONiC](https://azure.microsoft.com/en-us/blog/sonic-the-networking-switch-software-that-powers-the-microsoft-global-cloud/), [Azure Sphere OS](https://docs.microsoft.com/en-us/azure-sphere/product-overview/what-is-azure-sphere) and [Windows Subsystem for Linux (WSL)](https://docs.microsoft.com/en-us/windows/wsl/about).

[WSLg](https://github.com/microsoft/wslg) is the Windows Subsystem for Linux GUI and the purpose of the project is to enable support for running Linux GUI applications (X11 and [Wayland](https://wayland.freedesktop.org/)) on Windows in a fully integrated desktop experience. WSLg uses the [Weston compositor](https://github.com/wayland-project/weston), which is the standard Weston compositor with a heavily expanded RDP backend, a new RAIL/VAIL shell and various bug fixes. 

[SONiC](https://azure.github.io/SONiC/) is an open source network operating system based on Linux that runs on switches from multiple vendors and ASICs. It offers a full-suite of network functionality, like BGP and RDMA, that has been production-hardened in the data centers of some of the largest cloud-service providers.

[Azure Sphere](https://docs.microsoft.com/en-us/azure-sphere/product-overview/what-is-azure-sphere) is a secured, high-level application platform with built-in communication and security features for internet-connected devices. It comprises a secured, connected, crossover microcontroller unit (MCU), a custom high-level Linux-based operating system (OS), and a cloud-based security service that provides continuous, renewable security.

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

[DockerFLuent](https://github.com/mariotoffia/FluentDocker) is a library that enables [docker](https://www.docker.com/) and [docker-compose](https://docs.docker.com/compose/) interactions using a Fluent API. It is supported on Linux, Windows and MacOS. It also has support for the legazy [docker-machine](https://docker-docs.netlify.app/machine/overview/) interactions.

[Ansible-WSL](https://github.com/Wintus/Ansible-WSL) is an open source program that makes it easier to provision your Windows from inside of WSL by Ansible.

[WSL-DistroLauncher](https://github.com/Microsoft/WSL-DistroLauncher) is a sample/reference launcher app for WSL distro Microsoft Store packages. 

[Pengwin](https://github.com/WhitewaterFoundry/Pengwin) is a Linux distro optimized for WSL based on Debian. 

[Pengwin Enterprise](https://github.com/WhitewaterFoundry/Pengwin-Enterprise) is an enterprise Linux solution for Windows Subsystem for Linux (WSL) that is compatible with mainstream enterprise Linux distributions.


# Setting up WSL Linux Distributions

[Back to the Top](https://github.com/mikeroyal/WSL-Guide#table-of-contents)

<img src="https://user-images.githubusercontent.com/45159366/107585931-e6c63700-6bb3-11eb-8f25-f07f8ff05688.png">

[Pengwin](https://www.microsoft.com/en-us/p/pengwin/9nv1gv1pxz6p?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563833-225a6f80-7a95-11eb-93e8-f40799469bcc.png">
</p>

[Ubuntu 22.04.1 LTS](https://apps.microsoft.com/store/detail/ubuntu-22041-lts/9PN20MSR04DW?hl=en-us&gl=us)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/192119114-7ccdb71f-17ba-46e1-9cc9-c8e4a989450b.png">
</p>

[Ubuntu 20.04 LTS](https://www.microsoft.com/en-us/p/ubuntu-2004-lts/9n6svws3rx71?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563752-0eaf0900-7a95-11eb-9fd7-9be123b57cc1.png">
</p>


[Debian](https://www.microsoft.com/en-us/p/debian/9msvkqc78pk6?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563781-14a4ea00-7a95-11eb-8008-61867e38cf1e.png">
</p>

[SUSE Linux Enterprise Server 15 SP1](https://www.microsoft.com/en-us/p/suse-linux-enterprise-server-15-sp1/9pn498vpmf3z?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563809-1bcbf800-7a95-11eb-8be7-e94b2feb74a7.png">
</p>


[openSUSE Leap 15.4](https://apps.microsoft.com/store/detail/opensuse-leap-154/9PJPFJHRM62V?hl=en-us&gl=us)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/192119113-dea4ffc2-2e31-4006-b69d-a1129a672dcb.png">
</p>

[Fedora Linux for WSL](https://fedoramagazine.org/wsl-fedora-33/)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563789-15d61700-7a95-11eb-9c99-c1e68ead8388.png">
</p>

[Kali Linux](https://www.microsoft.com/en-us/p/kali-linux/9pkr34tncv07?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563822-1ec6e880-7a95-11eb-8d4b-a051178da3e2.png">
</p>

[Fedora Remix for WSL](https://www.microsoft.com/en-us/p/fedora-remix-for-wsl/9n6gdm4k2hnc?activetab=pivot:overviewtab)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563839-238b9c80-7a95-11eb-8a41-fc9775c810bb.png">
</p>


[GWSL](https://www.microsoft.com/en-us/p/gwsl/9nl6kd1h33v3?activetab=pivot:overviewtab) is an XServer that lets you easily run graphical Linux apps on Windows 10/11.

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/109563870-2e463180-7a95-11eb-93b1-77c1ff0ef39d.png">
</p>

## Azure Development
[Back to the Top](#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/143783606-58f4f708-dfe3-40f5-b9e3-c737479dc320.png">
  <br />
</p>

**Note:** WSL can run in Azure Windows VMs or you can also run a nested VM with a linux dsitro of your choice.

### Azure Learning Resources

[Microsoft Azure](https://azure.microsoft.com/en-us/) is a cloud computing service created by Microsoft for building, testing, deploying, and managing applications and services through Microsoft-managed data centers.

 * [Get started with Azure](https://azure.microsoft.com/en-us/get-started/)

 * [Azure Demo and Q&A](https://azure.microsoft.com/en-us/get-started/webinar/)

 * [Microsoft Azure Training & Certification Courses](https://www.microsoft.com/en-us/learning/azure-training-certification.aspx)

 * [Azure on Microsoft Learn](https://docs.microsoft.com/en-us/learn/azure/)

 * [Microsoft Certified: Azure Fundamentals](https://docs.microsoft.com/en-us/learn/certifications/azure-fundamentals/)

 * [Microsoft Certified: DevOps Engineer Expert Cert.](https://docs.microsoft.com/en-us/learn/certifications/devops-engineer)

 * [Introduction to Azure DevOps from A Cloud Guru](https://acloud.guru/learn/introduction-to-azure-devops)

 * [Microsoft Certified: Azure IoT Developer Specialty](https://docs.microsoft.com/en-us/learn/certifications/azure-iot-developer-specialty)

 * [Microsoft Certified: Azure Security Engineer Associate](https://docs.microsoft.com/en-us/learn/certifications/azure-security-engineer)


### Azure Tools

[Azure command-line interface (Azure CLI)](https://docs.microsoft.com/en-us/cli/azure/) is a command line that provides a set of commands used to create and manage Azure resources.

[Azure Service Fabric command-line interface (Azure Service Fabric CLI)](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-cli) is a command-line utility for interacting with and managing Service Fabric entities. The Service Fabric CLI can be used with either Windows or Linux clusters.

[Azure Functions](https://azure.microsoft.com/en-us/services/functions/) is a solution for easily running small pieces of code, or "functions," in the cloud. You can write just the code you need for the problem at hand, without worrying about a whole application or the infrastructure to run it.

[Azure DevOps](https://azure.microsoft.com/en-us/services/devops/?nav=min) is a set of services for teams to share code, track work, and ship software; CLIs Build, deploy, diagnose, and manage multi-platform, scalable apps and services; Azure Pipelines Continuously build, test, and deploy to any platform and cloud; Azure Lab Services Set up labs for classrooms, trials, development and testing, and other scenarios.

[Azure Data Studio](https://docs.microsoft.com/en-us/sql/azure-data-studio/what-is-azure-data-studio?view=sql-server-ver15) is a cross-platform database tool for data professionals using on-premises and cloud data platforms on Windows, macOS, and Linux. It offers a modern editor experience with IntelliSense, code snippets, source control integration, and an integrated terminal. It's engineered with the data platform user in mind, with built-in charting of query result sets and customizable dashboards.

[Azure Active Directory (Azure AD)](https://azure.microsoft.com/en-us/services/active-directory/) is Microsoft's cloud-based identity and access management service, which helps your employees sign in and access resources in: External resources, such as Microsoft 365, the Azure portal, and thousands of other SaaS applications.

[Azure Monitor](https://docs.microsoft.com/en-us/azure/azure-monitor/overview) is a client tool that helps you maximize the availability and performance of your applications and services. It delivers a comprehensive solution for collecting, analyzing, and acting on telemetry from your cloud and on-premises environments.

[Azure Cognitive Cognitive Services](https://azure.microsoft.com/en-us/services/cognitive-services/) is a set of cloud-based services with REST APIs and client library SDKs available to help you build cognitive intelligence into your applications. You can add cognitive features to your applications without having artificial intelligence (AI) or data science skills. All it takes is an API call to embed the ability to see, hear, speak, search, understand, and accelerate decision-making into your apps.

[Azure Data Lake Storage](https://azure.microsoft.com/en-us/solutions/data-lake/) is a storage repository that holds a large amount of data in its native, raw format. Data lake stores are optimized for scaling to terabytes and petabytes of data. The data typically comes from multiple heterogeneous sources, and may be structured, semi-structured, or unstructured.

[Azure Service Fabric](https://azure.microsoft.com/en-us/services/service-fabric/) is a distributed systems platform that makes it easy to package, deploy, and manage scalable and reliable microservices and containers. Service Fabric also addresses the significant challenges in developing and managing cloud native applications. It powers core Azure infrastructure as well as other Microsoft services such as Skype for Business, Intune, Azure Event Hubs, Azure Data Factory, Azure Cosmos DB, Azure SQL Database, Dynamics 365, and Cortana.

[Microsoft Azure Storage Emulator](https://docs.microsoft.com/en-us/azure/storage/common/storage-use-emulator) is a tool that emulates the Azure Blob, Queue, and Table services for local development purposes. You can test your application against the storage services locally without creating an Azure subscription or incurring any costs.

[Azure Cosmos DB Emulator](https://docs.microsoft.com/en-us/azure/cosmos-db/local-emulator?tabs=cli,ssl-netstd21) is a tool that provides a local environment that emulates the Azure Cosmos DB service for development purposes. Using the Azure Cosmos DB Emulator, you can develop and test your application locally, without creating an Azure subscription or incurring any costs.

[Microsoft Azure Storage Explorer](https://www.storageexplorer.com/) is a standalone app that makes it easy to work with Azure Storage data on Windows, macOS, and Linux.

[Azure BatchExplorer](https://github.com/Azure/BatchExplorer#batchexplorer) is a  client tool to help create, debug and monitor Azure Batch Applications.

[Azure Key Vault Explorer](https://github.com/microsoft/AzureKeyVaultExplorer/blob/master/README.md)  is a  client tool to help be productive when working with secrets.

[Azurite](https://github.com/Azure/Azurite/blob/master/README.md) is an open source Azure Storage API compatible server (emulator). Based on Node.js, Azurite provides cross platform experiences for customers wanting to try Azure Storage easily in a local environment. Azurite simulates most of the commands supported by Azure Storage with minimal dependencies.

[Azure Cloud Shell](https://shell.azure.com/) is an interactive, authenticated, browser-accessible shell for managing Azure resources. It provides the flexibility of choosing the shell experience that best suits the way you work, either Bash or PowerShell.

[Azure Lab Services](https://azure.microsoft.com/en-us/services/lab-services/) is an easy to set up and provide on-demand access to preconfigured virtual machines (VMs) to support your scenarios. Teach a class, train professionals, run a hackathon or a hands-on lab, and more.

[Azure Pipelines](https://azure.microsoft.com/en-us/services/devops/pipelines/) is a cloud-hosted pipelines for Linux, macOS, and Windows. Where you can build web, desktop and mobile applications. Deploy to any cloud or on‑premises.

[Azure Bots Service](https://azure.microsoft.com/en-us/services/bot-services/) is a service that develops intelligent, enterprise-grade bots that help you enrich the customer experience while maintaining control of your data. Build any type of bot—from a Q&A bot to your own branded virtual assistant—to quickly connect your users to the answers they need.

[Azure PlayFab](https://azure.microsoft.com/en-us/services/playfab/) is a service that enables developers to use the intelligent cloud to build and operate games, analyze gaming data and improve overall gaming experiences. Along with the Microsoft Game Stack that includes platforms, tools, and services like Visual Studio, DirectX, Havok, and Xbox.

[Azure Databricks](https://azure.microsoft.com/en-us/services/databricks/) is a tool that makes it fast, easy, and collaborative Apache Spark-based analytics platform. Azure Databricks, set up your Apache Spark™ environment in minutes, autoscale, and collaborate on shared projects in an interactive workspace. Azure Databricks supports Python, Scala, R, Java, and SQL, as well as data science frameworks and libraries including TensorFlow, PyTorch, and scikit-learn.

[Azure Machine Learning](https://azure.microsoft.com/en-us/services/machine-learning/) is an enterprise-grade machine learning service to build and deploy models faster. It empowers data scientists and developers with a wide range of productive experiences to build, train, and deploy machine learning models and foster team collaboration. Accelerate time to market with industry-leading MLOps—DevOps for machine learning. Innovate on a secure, trusted platform, designed for responsible machine learning.

[Azure Open Datasets](https://azure.microsoft.com/en-us/services/open-datasets/) is a tool that curates open data made easily accessible on Azure.

[Azure Percept](https://azure.microsoft.com/en-us/services/azure-percept/) is a comprehensive, easy-to-use platform with added security for creating edge AI solutions.

[Azure Data Share](https://azure.microsoft.com/en-us/services/data-share/) is a simple and safe service for sharing big data. It  provides full visibility into your data sharing relationships with a user-friendly interface. Share data in just a few clicks, or build your own application using the REST API.

[Azure Data Factory](https://azure.microsoft.com/en-us/services/data-factory/) is a fully managed, serverless data integration solution for ingesting, preparing, and transforming all your data at scale.

[Azure Synapse Analytics](https://azure.microsoft.com/en-us/services/synapse-analytics/) is a limitless analytics service that brings together data integration, enterprise data warehousing, and big data analytics. It gives you the freedom to query data on your terms, using either serverless or dedicated resources at scale.

[Azure HDInsight](https://azure.microsoft.com/en-us/services/hdinsight/) is an enterprise-ready, managed cluster service for open-source analytics.It let's you run popular open-source frameworks including Apache Hadoop, Spark, Hive, Kafka, and more.

[Azure Blockchain Service](https://azure.microsoft.com/en-us/services/blockchain-service/) is a service that simplifies the formation, management, and governance of consortium blockchain networks so you can focus on business logic and app development.

[Azure Logic Apps](https://azure.microsoft.com/en-us/services/logic-apps/) is a leading integration platform as a service (iPaaS) enables key enterprise scenarios for developers. Built on a containerized runtime that increases scale and portability while automating business-critical workflows anywhere.

[Azure Quantum](https://azure.microsoft.com/en-us/services/quantum/) is an innovative quantum computing and optimization solutions converge in a single marketplace quantum service.

[Azure VMware Solution](https://azure.microsoft.com/en-us/services/azure-vmware/) is a service that seamlessly moves VMware-based workloads from your datacenter to Azure and integrate your VMware environment with Azure. Keep managing your existing environments with the same VMware tools you already know while you modernize your applications with Azure native services.

[Azure Spring Cloud](https://azure.microsoft.com/en-us/services/spring-cloud/) is a fully managed Spring Cloud service, jointly built and operated with VMware.

[Azure CycleCloud](https://azure.microsoft.com/en-us/features/azure-cyclecloud/) is a serviece that creates, manages, operates, and optimizes HPC and big compute clusters of any scale.

[Azure API Apps](https://azure.microsoft.com/en-us/services/app-service/api/) is a service that quickly builds and consumes APIs in the cloud using the language of your choice.

[Azure Web Apps](https://azure.microsoft.com/en-us/services/app-service/web/) is an easy way to create and deploy mission-critical web applications that scale with your business.

[Windows Virtual Desktop](https://azure.microsoft.com/en-us/services/virtual-desktop/) is a service that enables a secure, remote desktop experience from anywhere.

[VMware Horizon Cloud on Microsoft Azure](https://azure.microsoft.com/en-us/services/virtual-desktop/vmware-horizon-cloud/) is a desktop virtualization service available in Azure Marketplace. Simplify your delivery of on-premises and cloud virtual desktops and applications by connecting your instance of Azure to VMware.

[Citrix Virtual Apps and Desktops for Azure](https://azure.microsoft.com/en-us/services/virtual-desktop/citrix-virtual-apps-desktops-for-azure/) is a desktop and app virtualization service available through Azure Marketplace or agreements with Citrix. Use familiar tools to manage on-premises Citrix deployments alongside Windows Virtual Desktop on Azure, supporting cloud modernization while maximizing your existing investment.

[Azure Container Registry](https://azure.microsoft.com/en-us/services/container-registry/) is a registry of Docker and Open Container Initiative (OCI) images, with support for all OCI artifacts.

[Azure Web App for Containers](https://azure.microsoft.com/en-us/services/app-service/containers/) is an easily deploy and run containerized applications on Windows and Linux.

[Azure SQL Edge](https://azure.microsoft.com/en-us/services/sql-edge/) is a service that makes a small-footprint, edge-optimized SQL database engine with built-in AI. This productivity tool for edge computing combines new capabilities such as data streaming and time series with in-database machine learning and graph features. Develop your application once and deploy anywhere across the edge, your datacenter, and Azure.

[Azure Arc](https://azure.microsoft.com/en-us/services/azure-arc/) is a service that offers simplified management, faster app development, and consistent Azure services. Standardize visibility, operations, and compliance across a wide range of resources and locations by extending the Azure control plane. Build cloud-native apps anywhere, at scale.

[Azure Artifacts](https://azure.microsoft.com/en-us/services/devops/artifacts/) is a services that provides fully integrated package management to your continuous integration/continuous delivery (CI/CD) pipelines with a single click. Create and share Maven, npm, NuGet, and Python package feeds from public and private sources with teams of any size.

[Azure Boards](https://azure.microsoft.com/en-us/services/devops/boards/) is a service that helps you plan, track, and discuss work across your teams. It let's you track work with Kanban boards, backlogs, team dashboards, and custom reporting.

[Azure ExpressRoute](https://azure.microsoft.com/en-us/services/expressroute/) is a tool that helps you experience a faster, private connection to Azure.

[Azure Sentinel](https://azure.microsoft.com/en-us/services/azure-sentinel/) is your birds-eye view across the enterprise. It uses the cloud and large-scale intelligence from decades of Microsoft security experience to work. Making your threat detection and response smarter and faster with artificial intelligence (AI).

[Azure Stack](https://azure.microsoft.com/en-us/overview/azure-stack/) is a service that builds and runs hybrid apps across datacenters, edge locations, remote offices, and the cloud.

[Azure Stack HCI](https://azure.microsoft.com/en-us/products/azure-stack/hci/) is a new hyperconverged infrastructure (HCI) operating system delivered as an Azure service that provides the latest security, performance, and feature updates. Deploy and run Windows and Linux virtual machines (VMs) in your datacenter or at the edge using your existing tools, processes, and skill sets.

[Azure Sphere](https://azure.microsoft.com/en-us/services/azure-sphere/) is a comprehensive IoT security solution including hardware (crossover microcontroller), OS, and cloud components for IoT device security to actively protect your devices, your business, and your customers.

[Azure IoT Hub](https://azure.microsoft.com/en-us/services/iot-hub/) is a service that provides a cloud-hosted solution back end to connect virtually any device. Extend your solution from the cloud to the edge with per-device authentication, built-in device management, and scaled provisioning.

[Azure IoT Edge](https://azure.microsoft.com/en-us/services/iot-edge/) is a fully managed service built on Azure IoT Hub. Deploy your cloud workloads—artificial intelligence, Azure and third-party services, or your own business logic to run on Internet of Things (IoT) edge devices via standard containers.

[Azure Lighthouse](https://azure.microsoft.com/en-us/services/azure-lighthouse/) is a secure managed services and access control for partners and customers.

[Azure Backup](https://azure.microsoft.com/en-us/services/backup/) is a cost-effective, secure, one-click backup solution that’s scalable based on your backup storage needs. The centralized management interface makes it easy to define backup policies and protect a wide range of enterprise workloads, including Azure Virtual Machines, SQL and SAP databases, and Azure file shares.

[Azure Resource Manager](https://azure.microsoft.com/en-us/features/resource-manager/) is a tool that enables you to repeatedly deploy your app and have confidence your resources are deployed in a consistent state. You define the infrastructure and dependencies for your app in a single declarative template. This template is flexible enough to use for all of your environments such as test, staging or production.

[Azure Automanage](https://azure.microsoft.com/en-us/services/azure-automanage/) is a tool that implifies IT management with optimized, automated operations across the entire lifecycle of dev/test and production virtual machines (VMs).

[Azure Network Watcher](https://azure.microsoft.com/en-us/services/network-watcher/) is a tool that monitors, diagnoses, and gains insights to your network performance and health.

[Azure Resource Mover](https://azure.microsoft.com/en-us/services/resource-mover/) is a that that Simplifies how you move multiple resources between Global Azure regions.

[Azure Bastion](https://azure.microsoft.com/en-us/services/azure-bastion/) is a fully managed PaaS service that provides secure and seamless RDP and SSH access to your virtual machines directly through the Azure Portal. Azure Bastion is provisioned directly in your Virtual Network (VNet) and supports all VMs in your Virtual Network (VNet) using SSL without any exposure through public IP addresses.

[Azure Load balancing](https://azure.microsoft.com/en-us/products/azure-load-balancing/) is a service that instantly scale your applications with Azure load balancing services for high availability and high performance. Get started with a quick needs assessment and load balancing recommendation—using the service selection tool.

[Azure Orbital](https://azure.microsoft.com/en-us/services/orbital/) is a Ground Station As-a-Service that provides communication and control of your satellite. Orbital enables easy and integrated data processing and scale for your operations directly from Azure. Leverage familiar Azure services to process and store your data at scale.

[Azure Route Server](https://azure.microsoft.com/en-us/services/route-server/) is a tool that enables network appliances to exchange route information with Azure virtual networks dynamically. Configure your network appliances and Azure ExpressRoute and VPN gateways to automatically take the latest route information from Azure Route Server instead of manually talking to each network.

[Azure VPN Gateway](https://azure.microsoft.com/en-us/services/vpn-gateway/) is a service that connects your on-premises networks to Azure through Site-to-Site VPNs in a similar way that you set up and connect to a remote branch office. The connectivity is secure and uses the industry-standard protocols Internet Protocol Security (IPsec) and Internet Key Exchange (IKE).

[Microsoft Azure Attestation](https://azure.microsoft.com/en-us/services/azure-attestation/) is a unified solution for remotely verifying the trustworthiness of a platform and integrity of the binaries running inside it. Azure Attestation receives evidence from the platform, validates it with security standards, evaluates it against configurable policies, and produces an attestation token for claims-based applications. The service supports attestation of trusted platform modules (TPMs) and trusted execution environments (TEEs) like Intel® Software Guard Extensions (SGX) and virtualization-based security (VBS) enclaves.

[Azure Data Box](https://azure.microsoft.com/en-us/services/databox/) is a device that easily moves data to Azure when busy networks aren’t an option. Move large amounts of data to Azure when you're limited by time, network availability, or costs, using common copy tools such as Robocopy. All data is AES-encrypted, and the devices are wiped clean after upload, in accordance with NIST Special Publication 800-88 revision 1 standards.

[Azure Blob Storage](https://azure.microsoft.com/en-us/services/storage/blobs/) is a massively scalable and secure object storage for cloud-native workloads, archives, data lakes, high-performance computing, and machine learning.

[PowerShell/PowerShell Core](https://docs.microsoft.com/en-us/powershell/) is a cross-platform (Windows, Linux, and macOS) automation and configuration tool/framework that works well with your existing tools and is optimized for dealing with structured data (e.g. JSON, CSV, XML, etc.), REST APIs, and object models. It includes a command-line shell, an associated scripting language and a framework for processing cmdlets.

[Hyper-V](https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/) creates virtual machines on Windows 10. Hyper-V can be enabled in many ways including using the Windows 10 control panel, PowerShell or using the Deployment Imaging Servicing and Management tool (DISM).

[GitHub Codespaces](https://docs.github.com/en/free-pro-team@latest/github/developing-online-with-codespaces) is an integrated development environment(IDE) on GitHub. That allows developers to develop entirely in the cloud using Visual Studio and Visual Studio Code.

[GitHub Actions](https://docs.github.com/en/actions) will automate, customize, and execute your software development workflows right in your repository with GitHub Actions. You can discover, create, and share actions to perform any job you'd like, including CI/CD, and combine actions in a completely customized workflow.[GitHub Actions for Azure](https://docs.microsoft.com/en-us/azure/developer/github/github-actions) you can create workflows that you can set up in your repository to build, test, package, release and deploy to Azure.


# Amazon Web Services (AWS)
[Back to the Top](#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/114322508-7d8c7280-9ad5-11eb-807e-4dc63c9bc0e1.png">
  <br />
</p>


### AWS Learning Resources

[Amazon Web Services](https://aws.amazon.com/about-aws/) is a reliable, scalable, and inexpensive on-demand cloud computing platforms, services and APIs to individuals, companies, and governments, on a metered pay-as-you-go basis.

 * [Developing on Amazon Linux 2 using Windows](https://aws.amazon.com/blogs/developer/developing-on-amazon-linux-2-using-windows/)

 * [Connect to your Linux instance from Windows using Windows Subsystem for Linux](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/WSL.html)

 * [AWS Training and Certification](https://aws.amazon.com/training/)

 * [Getting Started with Amazon Web Services (AWS)](https://aws.amazon.com/getting-started/)

 * [Hands-On Tutorials for Amazon Web Services (AWS)](https://aws.amazon.com/getting-started/hands-on/)

 * [Getting started with AWS IoT Core ](https://docs.aws.amazon.com/iot/latest/developerguide/iot-gs.html)

 * [AWS Academy - Amazon Web Services (AWS)](https://aws.amazon.com/training/awsacademy/)

 * [AWS Educate](https://aws.amazon.com/education/awseducate/)

 * [Architecting on AWS Classroom Training](https://aws.amazon.com/training/classroom/architecting-on-aws/)

 * [DevOps Engineering on AWS from AWS Training](https://aws.amazon.com/training/course-descriptions/devops-engineering/)

 * [AWS Certified DevOps Engineer - Professional from A Cloud Guru](https://acloud.guru/learn/aws-certified-devops-engineer-professional)

 * [AWS Certified Security - Specialty Certification](https://aws.amazon.com/certification/certified-security-specialty/)


### AWS Tools

[AWS Toolkit for Visual Studio Code](https://aws.amazon.com/visualstudiocode/)  is an open source plug-in for the Visual Studio Code that makes it easier to create, debug, and deploy applications on Amazon Web Services. 

[AWS Marketplace](https://aws.amazon.com/) is a digital catalog with thousands of software listings from independent software vendors that make it easy to find, test, buy, and deploy software that runs on AWS.

[AWS Cloud9](https://aws.amazon.com/cloud9/) is a cloud-based integrated development environment (IDE) that lets you write, run, and debug your code with just a browser. It includes a code editor, debugger, and terminal. Cloud9 comes prepackaged with essential tools for popular programming languages, including JavaScript, Python, PHP, and more, so you don’t need to install files or configure your development machine to start new projects.

[AWS Command Line Interface (CLI)](https://aws.amazon.com/cli/) is a unified tool to manage your AWS services through the command line interface.

[AWS Amplify Command Line Interface (CLI)](https://docs.amplify.aws/cli) is a unified toolchain to create, integrate, and manage the AWS cloud services for your app.

[AWS Serverless Application Model (SAM) CLI](https://github.com/aws/aws-sam-cli) is a command line tool for an open-source framework for building serverless applications. It provides shorthand syntax to express functions, APIs, databases, and event source mappings. With just a few lines of configuration, you can define the application you want and model it.

[AWS Copilot command line interface (CLI)](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/AWS_Copilot.html) is a command line tool that simplifies building, releasing, and operating production-ready containerized applications on Amazon ECS from a local development environment. The AWS Copilot CLI aligns with developer workflows that support modern application best practices: from using infrastructure as code to creating a CI/CD pipeline provisioned on behalf of a user.

[Amazon Elastic Container Service (Amazon ECS) command line interface (CLI)](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ECS_CLI.html) is a command line tool that  provides high-level commands to simplify creating, updating, and monitoring clusters and tasks from a local development environment. The Amazon ECS CLI supports Docker Compose files, a popular open-source specification for defining and running multi-container applications.

[AWS ECS](https://aws.amazon.com/ecs/) is a highly scalable, high-performance container orchestration service that supports Docker containers and allows you to easily run and scale containerized applications on AWS. Amazon ECS eliminates the need for you to install and operate your own container orchestration software, manage and scale a cluster of virtual machines, or schedule containers on those virtual machines.

[Amazon Simple Storage Service (Amazon S3)](https://aws.amazon.com/s3/) is an object storage service that offers industry-leading scalability, data availability, security, and performance. This means customers of all sizes and industries can use it to store and protect any amount of data for a range of use cases, such as data lakes, websites, mobile applications, backup and restore, archive, enterprise applications, IoT devices, and big data analytics.

[AWS Cloud Development Kit (AWS CDK)](https://aws.amazon.com/cdk/) is an open-source software development framework to define cloud infrastructure in code and provision it through AWS CloudFormation. It offers a high-level object-oriented abstraction to define AWS resources imperatively using the power of modern programming languages.

[AWS Lambda](https://aws.amazon.com/lambda/) is an event-driven, serverless computing platform provided by Amazon as a part of the Amazon Web Services. It is a computing service that runs code in response to events and automatically manages the computing resources required by that code.

[AWS Elastic Beanstalk](https://aws.amazon.com/elasticbeanstalk/) is an easy-to-use service for deploying and scaling web applications and services developed with Java,.NET, PHP, Node.js, Python, Ruby, Go, and Docker on familiar servers such as Apache, Nginx, Passenger, and IIS.

[AWS IoT Greengrass](https://aws.amazon.com/greengrass/) is an Internet of Things (IoT) open source edge runtime and cloud service that helps you build, deploy, and manage device software. It is used for IoT applications on millions of devices in homes, factories, vehicles, and businesses.

[AWS CodeArtifact](https://aws.amazon.com/codeartifact/) is a fully managed artifact repository service that makes it easy for organizations of any size to securely store, publish, and share software packages used in their software development process. CodeArtifact can be configured to automatically fetch software packages and dependencies from public artifact repositories so developers have access to the latest versions.

[AWS CodeCommit](https://aws.amazon.com/codecommit/) is a fully-managed source control service that hosts secure Git-based repositories. It makes it easy for teams to collaborate on code in a secure and highly scalable ecosystem. CodeCommit eliminates the need to operate your own source control system or worry about scaling its infrastructure.

[AWS CodePipeline](https://aws.amazon.com/codepipeline/) is a fully managed [continuous delivery](https://aws.amazon.com/devops/continuous-delivery/) service that helps you automate your release pipelines for fast and reliable application and infrastructure updates. CodePipeline automates the build, test, and deploy phases of your release process every time there is a code change, based on the release model you define. This enables you to rapidly and reliably deliver features and updates. You can easily integrate AWS CodePipeline with third-party services such as GitHub or with your own custom plugin.

[AWS CodeStar](https://aws.amazon.com/codestar/) is a unified user interface, enabling you to easily manage your software development activities in one place. With AWS CodeStar, you can set up your entire [continuous delivery](https://aws.amazon.com/devops/continuous-delivery/) toolchain in minutes, allowing you to start releasing code faster.

[AWS X-Ray](https://aws.amazon.com/xray/) is a tool that traces user requests as they travel through your entire application. It aggregates the data generated by the individual services and resources that make up your application, providing you an end-to-end view of how your application is performing. It helps developers analyze and debug production, distributed applications, such as those built using a microservices architecture.

[AWS CodeDeploy](https://aws.amazon.com/codedeploy/) is a fully managed deployment service that automates software deployments to a variety of compute services such as Amazon EC2, AWS Fargate, AWS Lambda, and your on-premises servers. AWS CodeDeploy makes it easier for you to rapidly release new features, helps you avoid downtime during application deployment, and handles the complexity of updating your applications.

[AWS CodeBuild](https://aws.amazon.com/codebuild/) is a fully managed continuous integration service that compiles source code, runs tests, and produces software packages that are ready to deploy. With CodeBuild, you don't need to provision, manage, and scale your own build servers.

[Red Hat OpenShift Service on AWS (ROSA)](https://www.openshift.com/products/amazon-openshift) is a fully-managed and jointly supported Red Hat OpenShift offering that combines the power of Red Hat OpenShift, the industry's most comprehensive enterprise Kubernetes platform, and the AWS public cloud.

[Amazon API Gateway](https://aws.amazon.com/api-gateway/) is a fully managed service that makes it easy for developers to create, publish, maintain, monitor, and secure APIs at any scale.

[AWS Storage Gateway](https://aws.amazon.com/storagegateway/) is a hybrid cloud storage service that gives you on-premises access to virtually unlimited cloud storage.

[AWS Transit Gateway](https://aws.amazon.com/transit-gateway/) is a tool that connects VPCs and on-premises networks through a central hub. This simplifies your network and puts an end to complex peering relationships. It acts as a cloud router - each new connection is only made once.

[Gateway Load Balancer (GWLB)](https://aws.amazon.com/elasticloadbalancing/gateway-load-balancer/) is a tool that makes it easy to deploy, scale, and manage your third-party virtual appliances. It gives you one gateway for distributing traffic across multiple virtual appliances, while scaling them up, or down, based on demand.

[AWS Chalice](https://aws.amazon.com/blogs/developer/deploying-aws-chalice-application-using-aws-cloud-development-kit/) is a Python Serverless Microframework for AWS and allows you to quickly create and deploy applications that use Amazon API Gateway and AWS Lambda.

[AWS ParallelCluster](https://aws.amazon.com/hpc/parallelcluster/) is an AWS supported Open Source cluster management tool to deploy and manage HPC clusters in the AWS cloud.

[AWS Copilot CLI](https://aws.amazon.com/containers/copilot/) is a tool for developers to build, release and operate production ready containerized applications on Amazon ECS and AWS Fargate.

[AWS Fargate](https://aws.amazon.com/fargate/) is a serverless compute engine for containers that works with both Amazon Elastic Container Service (ECS) and Amazon Elastic Kubernetes Service (EKS).

[Amazon Chime](https://aws.amazon.com/chime/) is a communications service that lets you meet, chat, and place business calls inside and outside your organization, all using a single application.

[Amazon Virtual Private Cloud (Amazon VPC)](https://console.aws.amazon.com/vpc) is a service that lets you launch AWS resources in a logically isolated virtual network that you define. You have complete control over your virtual networking environment, including selection of your own IP address range, creation of subnets, and configuration of route tables and network gateways.

[AWS Lightsail](https://aws.amazon.com/lightsail/) is an easy-to-use virtual private server (VPS) that offers you everything needed to build an application or website, plus a cost-effective, monthly plan.

[Amazon Relational Database Service (Amazon RDS)](https://console.aws.amazon.com/rds/home) is a tool that makes it easy to set up, operate, and scale a relational database in the cloud. It provides cost-efficient and resizable capacity while automating time-consuming administration tasks such as hardware provisioning, database setup, patching and backups.

[Amazon Aurora](https://console.aws.amazon.com/rds/home) is a MySQL and PostgreSQL-compatible relational database built for the cloud, that combines the performance and availability of traditional enterprise databases with the simplicity and cost-effectiveness of open source databases.

[Amazon Athena](https://aws.amazon.com/athena/) is an interactive query service that makes it easy to analyze data in Amazon S3 using standard SQL. Athena is serverless, so there is no infrastructure to manage, and you pay only for the queries that you run.

[Amazon CloudSearch](https://aws.amazon.com/cloudsearch/) is a managed service in the AWS Cloud that makes it simple and cost-effective to set up, manage, and scale a search solution for your website or application.

[Amazon Kinesis](https://aws.amazon.com/kinesis/) is a tool that makes it easy to collect, process, and analyze real-time, streaming data so you can get timely insights and react quickly to new information. Amazon Kinesis offers key capabilities to cost-effectively process streaming data at any scale, along with the flexibility to choose the tools that best suit the requirements of your application. With Amazon Kinesis, you can ingest real-time data such as video, audio, application logs, website clickstreams, and IoT telemetry data for machine learning, analytics, and other applications.

[Amazon EMR](https://aws.amazon.com/emr/) is the industry-leading cloud big data platform for processing vast amounts of data using open source tools such as [Apache Spark](https://aws.amazon.com/emr/features/spark/), [Apache Hive](https://aws.amazon.com/emr/features/hive/), [Apache HBase](https://aws.amazon.com/emr/features/hbase/), [Apache Flink](https://aws.amazon.com/blogs/big-data/use-apache-flink-on-amazon-emr/),[Apache Hudi](https://aws.amazon.com/emr/features/hudi/), and [Presto](https://aws.amazon.com/emr/features/presto/).

[AWS RedShift](https://aws.amazon.com/redshift/) is a data warehouse tool that makes it as easy to gain new insights from all your data. With Redshift, you can easily query and combine exabytes of structured and semi-structured data across your data warehouse, operational database, and data lake using standard SQL. It lets you easily save the results of your queries back to your S3 data lake using open formats, like Apache Parquet, so that you can do additional analytics from other analytics services like Amazon EMR, Amazon Athena, and Amazon SageMaker.

[AWS Data Pipeline](https://aws.amazon.com/datapipeline/) is a web service that helps you reliably process and move data between different AWS compute and storage services, as well as on-premises data sources, at specified intervals. AWS Data Pipeline, let's you regularly access your data where it’s stored, transform and process it at scale, and efficiently transfer the results to AWS services such as Amazon S3, Amazon RDS, Amazon DynamoDB, and Amazon EMR.

[AWS Glue](https://aws.amazon.com/glue/) is a serverless data integration service that makes it easy to discover, prepare, and combine data for analytics, machine learning, and application development.

[AWS Lake Formation](https://aws.amazon.com/lake-formation/) is a service that makes it easy to set up a secure data lake in days. A data lake is a centralized, curated, and secured repository that stores all your data, both in its original form and prepared for analysis.

[Amazon Managed Blockchain](https://aws.amazon.com/managed-blockchain/) is a fully managed service that makes it easy to join public networks or create and manage scalable private networks using the popular open-source frameworks [Hyperledger Fabric](https://aws.amazon.com/blockchain/what-is-hyperledger-fabric/) and Ethereum.

[AWS Wavelength](https://aws.amazon.com/wavelength/) is an AWS Infrastructure offering optimized for mobile edge computing applications. Wavelength Zones are AWS infrastructure deployments that embed AWS compute and storage services within communications service providers’ (CSP) datacenters at the edge of the 5G network, so application traffic from 5G devices can reach application servers running in Wavelength Zones without leaving the telecommunications network.

[AWS Outposts](https://aws.amazon.com/outposts/) is a fully managed service that offers the same AWS infrastructure, AWS services, APIs, and tools to virtually any datacenter, co-location space, or on-premises facility for a truly consistent hybrid experience. AWS Outposts is ideal for workloads that require low latency access to on-premises systems, local data processing, data residency, and migration of applications with local system interdependencies.

[AWS Batch](https://aws.amazon.com/batch/) is atool that enables developers, scientists, and engineers to easily and efficiently run hundreds of thousands of batch computing jobs on AWS. AWS Batch dynamically provisions the optimal quantity and type of compute resources (e.g., CPU or memory optimized instances) based on the volume and specific resource requirements of the batch jobs submitted. AWS Batch plans, schedules, and executes your batch computing workloads across the full range of AWS compute services and features, such as [AWS Fargate](https://aws.amazon.com/fargate/), [Amazon EC2](https://aws.amazon.com/ec2/) and [Spot Instances](https://aws.amazon.com/ec2/spot/).

[Amazon Forecast](https://aws.amazon.com/forecast/) is a fully managed service that uses machine learning to deliver highly accurate forecasts.

[AWS Snow Family](https://aws.amazon.com/snow/) is a highly-secure, portable devices to collect and process data at the edge, and migrate data into and out of AWS.

[Amazon Neptune](https://aws.amazon.com/neptune/) is a fast, reliable, fully managed graph database service that makes it easy to build and run applications that work with highly connected datasets. The core of Amazon Neptune is a purpose-built, high-performance graph database engine optimized for storing billions of relationships and querying the graph with milliseconds latency.

[Amazon Timestream](https://aws.amazon.com/timestream/) is a fast, scalable, and serverless time series database service for IoT and operational applications that makes it easy to store and analyze trillions of events per day up to 1,000 times faster and at as little as 1/10th the cost of relational databases.

#  Google Cloud Platform (GCP)
[Back to the Top](#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/114321928-639d6080-9ad2-11eb-8297-5e6c10c1c792.png">
  <br />
</p>

### Google Cloud Learning Resources

[Google Cloud Platform] is a public cloud platform that lets you build, deploy, and scale applications, websites, and services on the same infrastructure as Google.

 * [Install the Google Cloud CLI](https://cloud.google.com/sdk/docs/install-sdk)

 * [Google Cloud Courses and Training](https://cloud.google.com/training/)

 * [Architecting with Google Compute Engine](https://google.qwiklabs.com/courses/1421?utm_source=gcp_training&utm_medium=website&utm_campaign=cgc)

 * [Get started with Cloud Storage on Web with Firebase](https://firebase.google.com/docs/storage/web/start)

 * [Getting started with BigQuery](https://cloud.google.com/bigquery/docs/quickstarts)

 * [Machine Learning Crash Course with Google Cloud](https://developers.google.com/machine-learning/crash-course/)

 * [Architecting with Google Kubernetes Engine in Google Cloud](https://google.qwiklabs.com/courses/1232?utm_source=gcp_training&utm_medium=website&utm_campaign=cgc)

 * [Google Cloud Internet of Things (IoT)](https://developers.google.com/iot/)

 * [Google Cloud Certified Professional Cloud Security Engineer](https://cloud.google.com/certification/cloud-security-engineer)


### Google Cloud Tools

[Cloud Code for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=GoogleCloudTools.cloudcode) is a an open source extension that brings the power and convenience of IDEs to cloud-native application development. Cloud Code integrates with Google Cloud services like Google Kubernetes Engine, Cloud Run, Cloud APIs and Secret Manager, and makes you feel like you are working with local code.

[Cloud SDK](https://cloud.google.com/sdk/) is a clietn tool used to to manage Google Cloud resources and applications with command-line tools and libraries. The Cloud SDK contains gcloud, gsutil, and bq command-line tools, which you can use to access [Compute Engine](https://cloud.google.com/compute), [Cloud Storage](https://cloud.google.com/storage), [BigQuery](https://cloud.google.com/bigquery), and more.

[Google Cloud Shell](https://cloud.google.com/shell/) is a free admin machine with browser-based command-line access for managing your infrastructure and applications on Google Cloud Platform.

[Cloud Code](https://cloud.google.com/code) is a client tool that writes, debugs, and run cloud-native applications, locally or in the cloud—quickly and easily. Extensions to IDEs such as Visual Studio Code and IntelliJ are provided to let you rapidly iterate, debug, and deploy code to Kubernetes.

[gcloud](https://cloud.google.com/sdk/gcloud/) is a CLI (command line interface) manages authentication, local configuration, developer workflow, interactions with Google Cloud APIs.

[gsutil](https://cloud.google.com/storage/docs/gsutil) is a Python application that lets you access Cloud Storage from the command line.

[Compute Engine](https://cloud.google.com/compute) is a secure and customizable compute service that lets you create and run virtual machines on Google’s infrastructure.

[App Engine](https://cloud.google.com/appengine/) is a client tool that lets you build and run applications on Google's infrastructure. It automatically scales to support sudden traffic spikes without provisioning, patching, or monitoring.

[Google Kubernetes Engine (GKE)](https://cloud.google.com/kubernetes-engine/) is a managed, production-ready environment for deploying containerized applications.

[Cloud Storage](https://cloud.google.com/storage) is a Object storage for companies of all sizes. Where store any amount of data and retrieve it as often as you would like to.

[BigQuery](https://cloud.google.com/bigquery) is a serverless, highly scalable, and cost-effective multi-cloud data warehouse designed for business agility.

[Cloud Bigtable](https://cloud.google.com/bigtable/) is Google's fully managed NoSQL Big Data database service. It's the same database that powers many core Google services, including Search, Analytics, Maps, and Gmail.

[Cloud SQL](https://cloud.google.com/sql/) is a tool that makes it easy to set up, manage, and administer your Postgres databases on Google Cloud.

[Cloud Datastore](https://cloud.google.com/datastore/) is a schemaless database, which allows you to worry less about making changes to your underlying data structure as your application evolves.

[Cloud Pub/Sub](https://cloud.google.com/pubsub/) is a messaging middleware for traditional service integration or a simple communication medium for modern microservices.

[Cloud Dataflow](https://cloud.google.com/dataflow/) is a tool that brings streaming events to Google Cloud's AI Platform and TensorFlow Extended (TFX) to enable predictive analytics, fraud detection, real-time personalization, and other advanced analytics.

[Cloud Dataproc](https://cloud.google.com/dataproc/) is a fully managed and highly scalable service for running Apache Spark, Apache Flink, Presto, and 30+ open source tools and frameworks.

[Cloud Datalab](https://cloud.google.com/datalab/docs/) is a tool that provides a productive, interactive, and integrated tool to explore, visualize, analyze and transform data, bringing together the power of Python, SQL, JavaScript, and the Google Cloud Platform with services such as BigQuery and Storage.

[Cloud Vision API](https://cloud.google.com/vision/) is a library that offers powerful pre-trained machine learning models through REST and RPC APIs.

[Cloud Speech API](https://cloud.google.com/speech-to-text/) is a library that enables developers to convert audio to text by applying powerful neural network models. The API recognizes over 80 languages and variants, to support your global user base.

[Cloud Build](https://cloud.google.com/cloud-build) is a continuous build, test, and deploy software across all languages and in multiple environments—including VMs, serverless, Kubernetes, and Firebase.

[Anthos](https://cloud.google.com/anthos/docs/concepts/overview) is a modern application management platform that provides a consistent development and operations experience for cloud and on-premises environments.

[Jenkins on Google Cloud](https://cloud.google.com/jenkins) is a client tool that helps speed up, scale, and security from your Jenkins pipeline.

[Tekton on Google Cloud](https://cloud.google.com/tekton) is a client tool that standardizes CI/CD pipelines across languages, and tools on premises or in the cloud with a Kubernetes native open source framework.

[Artifact Registry](https://cloud.google.com/artifact-registry) is a client tool to manage container images and language packages such as Maven and npm all in one place, fully integrated with Google Cloud’s tooling and runtimes.

[Cloud Deployment Manager](https://cloud.google.com/deployment-manager) is a client that creates and manages cloud resources with simple templates. Specify all the resources needed for applications in a declarative format using yaml.

[Red Hat OpenShift on Google Cloud](https://cloud.google.com/solutions/partners/openshift-on-gcp) is a fully-managed and jointly supported Red Hat OpenShift offering that enables you to deploy stateful and stateless apps with nearly any language, framework, database, or service. It gives you a hosted environment entirely on Google Cloud. A hybrid environment where you maintain part of your workload on-premises or in a private hosting environment and migrate the rest to Google Cloud.

## Kubernetes

[Back to the Top](#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/95383873-a884d800-08a0-11eb-8eaf-57af5b119f56.png">
  <br />
</p>

[Kubernetes (K8s)](https://kubernetes.io/) is an open-source system for automating deployment, scaling, and management of containerized applications.

<img src="https://user-images.githubusercontent.com/45159366/105645195-db9ea780-5e4e-11eb-8357-fb38b2f06d74.png">

**Building Highly-Availability(HA) Clusters with kubeadm. Source: [Kubernetes.io](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/high-availability/)**

### Red Hat CodeReady Containers (CRC)

[Back to the Top](#table-of-contents)

[Red Hat CodeReady Containers (CRC)](https://developers.redhat.com/content-gateway/rest/mirror/pub/openshift-v4/clients/crc/2.9.0) is a tool that provides a minimal, preconfigured OpenShift 4 cluster on a laptop or desktop machine for development and testing purposes. CRC is delivered as a platform inside of the VM.

 * **odo (OpenShift Do)**, a CLI tool for developers, to manage application components on the OpenShift Container Platform.
 
 <p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/193531714-a4316c2c-4ba9-4d31-86cf-7a3edc6f6375.png">
  <br />
</p>

**System Requirements:**

   * **OS:** CentOS Stream 8/RHEL 8/Fedora or later (the latest 2 releases).
   * **Download:** [pull-secret](https://cloud.redhat.com/openshift/install/crc/installer-provisioned?intcmp=701f20000012ngPAAQ)
   * **Login:** [Red Hat account](https://access.redhat.com/login)

**Other physical requirements include:**

   * Four virtual CPUs (**4 vCPUs**)
   * 10GB of memory (**RAM**)
   * 40GB of storage space

**To set up CodeReady Containers, start by creating the ```crc``` directory, and then download and extract the ```crc``` package:**

```mkdir /home/<user>/crc```

```wget https://mirror.openshift.com/pub/openshift-v4/clients/crc/latest/crc-linux-amd64.tar.xz```

```tar -xvf crc-linux-amd64.tar.xz```

**Next, move the files to the crc directory and remove the downloaded package(s):**

```mv /home/<user>/crc-linux-<version>-amd64/* /home/<user>/crc```

```rm /home/<user>/crc-linux-amd64.tar.xz```

```rm -r /home/<user>/crc-linux-<version>-amd64```

**Change to the ```crc``` directory, make ```crc``` executable, and export your ```PATH``` like this:**

```cd /home/<user>/crc```

```chmod +x crc```

```export PATH=$PATH:/home/<user>/crc```

**Set up and start the cluster:**

```crc setup```

```crc start -p /<path-to-the-pull-secret-file>/pull-secret.txt```

**Set up the OC environment:**

```crc oc-env```

```eval $(crc oc-env)```

**Log in as the developer user:**

```oc login -u developer -p developer https://api.crc.testing:6443```

```oc logout```

**And then, log in as the platform’s admin:**

```oc login -u kubeadmin -p password https://api.crc.testing:6443```

```oc logout```

#### Interacting with the cluster. The most common ways include:

**Starting the graphical web console:**

```crc console```

**Display the cluster’s status:**

 ```crc status```

**Shut down the OpenShift cluster:**

```crc stop```

**Delete or kill the OpenShift cluster:**

```crc delete```

 <p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/193534587-c86d546f-814b-420d-ac45-15d5c2ca6ede.png">
  <br />
</p>

### Setting up Podman

[Back to the Top](#table-of-contents)

[Podman (the POD manager)](https://podman.io/) is an open source tool for developing, managing, and running containers on your Linux® systems. It also manages the entire container ecosystem using the libpod library.  Podman’s daemonless and inclusive architecture makes it a more secure and accessible option for container management, and its accompanying tools and features, such as [Buildah](https://www.redhat.com/en/topics/containers/what-is-buildah) and [Skopeo](https://www.redhat.com/en/topics/containers/what-is-skopeo), allow developers to customize their container environments to best suit their needs. 

 * Fedora: ```sudo dnf install podman```
 * CentOS: ```sudo yum --enablerepo=extras install podman```
 * Ubuntu 20.04 or later: ```sudo apt install podman```
 * Debian 11 (bullseye) or later, or sid/unstable: ```sudo apt install podman```
 * ArchLinux: ```sudo pacman -S podman``` and then tweaks for rootless 

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/193426691-d47b65df-bd28-4c60-82f8-282005392556.png">
  <br />
 Podman
</p>

### Setting up Buildah

[Back to the Top](#table-of-contents) 

[Buildah](https://buildah.io/) is an open source, Linux-based tool that can build Docker- and Kubernetes-compatible images, and is easy to incorporate into scripts and build pipelines. In addition, Buildah has overlap functionality with [Podman](https://podman.io/), [Skopeo](https://github.com/containers/skopeo), and [CRI-O](https://cri-o.io/).

 * Fedora: ```sudo dnf -y install buildah```
 * CentOS: ```sudo yum --enablerepo=extras install buildah```
 * Ubuntu 20.04 or later: ```sudo apt install buildah```
 * Debian 11 (bullseye) or later, or sid/unstable: ```sudo apt install -y buildah```
 * ArchLinux: ```sudo pacman -S buildah``` and then tweaks for rootless 
 
 <p align="center">
<img src="https://user-images.githubusercontent.com/45159366/193426954-22a0dcd8-5911-448b-b538-f1569ec20b35.png">
  <br />
 Buildah
</p>

### Installing Kubernetes on WSL with Rancher Desktop

[Back to the Top](#table-of-contents)

[Rancher Desktop](https://www.rancher.com/products/rancher-desktop) is an open-source desktop application for Mac, Windows and Linux. Rancher Desktop runs Kubernetes and container management on your desktop letting you choose the version of Kubernetes you want to run. It can also build, push, pull, and run container images using either the Docker CLI (with Moby/dockerd) or nerdctl (with containerd).

 **Features:**

  * Installs a new Linux VM in WSL2 that has a Kubernetes cluster based on [k3s](https://k3s.io/) as well as installs various components in it such as KIM (for building docker images on the cluster) and the [Traefik Ingress Controller](https://traefik.io/solutions/kubernetes-ingress/).
 
  * It installs the kubectl and Helm CLIs on the Windows side linked to them.
 
  * A nice Windows app to manage its settings and help facilitate its upgrades.
  
<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/193426266-8fa9222c-f1b1-4bd4-ad1f-3a9e99e6fb06.png">
  <br />
 Rancher Desktop 
</p>

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/193426337-263c38d0-d875-49ef-931d-693a018c4805.png">
  <br />
 Rancher Desktop Kubernetes Settings
</p>

#### .deb Dev Repository

```curl -s https://download.opensuse.org/repositories/isv:/Rancher:/dev/deb/Release.key | gpg --dearmor | sudo dd status=none of=/usr/share/keyrings/isv-rancher-dev-archive-keyring.gpg```

```echo 'deb [signed-by=/usr/share/keyrings/isv-rancher-dev-archive-keyring.gpg] https://download.opensuse.org/repositories/isv:/Rancher:/dev/deb/ ./' | sudo dd status=none of=/etc/apt/sources.list.d/isv-rancher-dev.list```

```sudo apt update```

**See available versions**

```apt list -a rancher-desktop```

```sudo apt install rancher-desktop=<version>```

#### .rpm Dev Repository

```sudo zypper addrepo https://download.opensuse.org/repositories/isv:/Rancher:/dev/rpm/isv:Rancher:dev.repo```

```sudo zypper refresh```

**See available versions**

```zypper search -s rancher-desktop```

```zypper install --oldpackage rancher-desktop=<version>```

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/193425700-2b184434-a2c8-4bd6-9e17-9eb278fa490c.png">
  <br />
 Rancher Desktop Architecture Overview
</p>

### Installing Kubernetes on WSL with Docker Desktop

[Back to the Top](#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/192118415-36372619-de7a-4f5a-84c7-d20477765233.png">
  <br />
  Enable the WSL 2 base engine in Docker Desktop
</p>

We also need to set in Resources which WSL2 distribution we want to access Docker from, as shown below using Ubuntu 20.04. Then remember to restart Docker for Windows, and once the restart is complete we can use the docker command from within WSL:

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/192118416-fafb78c5-9222-4e6e-bc9c-47b3fe45a6bf.png">
  <br />
</p>


Make sure to use kind as a simple way to run Kubernetes in a container. Here we will install the instructions from the official [Kind website](https://kind.sigs.k8s.io/docs/user/quick-start/).

```curl -Lo ./kind https://github.com/kubernetes-sigs/kind/releases/download/v0.16.0/kind-$(uname)-amd64```

```chmod +x ./kind```

```mv ./kind /usr/local/bin/```

Now that kind is installed, we can create the Kubernetes cluster

```echo $KUBECONFIG```

```ls $HOME/.kube```

```kind create cluster --name wslkube```

```ls $HOME/.kube```

We have successfully created a single-node Kubernetes cluster.

```kubectl get nodes```

```kubectl get all --all-namespaces```

### Installing Kubernetes on WSL with Microk8s

[Back to the Top](#table-of-contents)

* **Note:** This install option requires systemd to be running on WSL

* **WSL Systemd requirements:** Windows 11 and a version of WSL 0.67.6 or above. 

[MicroK8s](https://microk8s.io/) is the simplest production-grade upstream Kubernets setup to get up and running.

Installing Microk8s 

```sudo snap install microk8s --classic```

Checking the status while Kubernetes starts

```microk8s status --wait-ready```

Turning on the services you want

```microk8s enable dashboard dns registry istio```

Try **microk8s enable --help** for a list of available services and optional features. **microk8s disable <name>** turns off a service.

Start using Kubernetes

```microk8s kubectl get all --all-namespaces```

If you mainly use MicroK8s you can make our kubectl the default one on your command-line with **alias mkctl="microk8s kubectl"**.


Access the Kubernetes dashboard

```microk8s dashboard-proxy```
 

### Kubernetes Tools, Frameworks, and Projects 

[Open Container Initiative](https://opencontainers.org/about/overview/) is an open governance structure for the express purpose of creating open industry standards around container formats and runtimes.

[Buildah](https://buildah.io/) is a command line tool to build Open Container Initiative (OCI) images. It can be used with Docker, Podman, Kubernetes.

[Podman](https://podman.io/) is a daemonless, open source, Linux native tool designed to make it easy to find, run, build, share and deploy applications using Open Containers Initiative (OCI) Containers and Container Images. Podman provides a command line interface (CLI) familiar to anyone who has used the Docker Container Engine.

[Containerd](https://containerd.io) is a daemon that manages the complete container lifecycle of its host system, from image transfer and storage to container execution and supervision to low-level storage to network attachments and beyond. It is available for Linux and Windows. 

[Azure Kubernetes Service (AKS)](https://azure.microsoft.com/en-us/services/kubernetes-service/) is serverless Kubernetes, with a integrated continuous integration and continuous delivery (CI/CD) experience, and enterprise-grade security and governance. Unite your development and operations teams on a single platform to rapidly build, deliver, and scale applications with confidence.

[Google Kubernetes Engine (GKE)](https://cloud.google.com/kubernetes-engine/) is a managed, production-ready environment for running containerized applications.

[Amazon EKS](https://docs.aws.amazon.com/eks/latest/userguide/what-is-eks.html) is a tool that runs Kubernetes control plane instances across multiple Availability Zones to ensure high availability.

[AWS Controllers for Kubernetes (ACK)](https://aws.amazon.com/blogs/containers/aws-controllers-for-kubernetes-ack/) is a new tool that lets you directly manage AWS services from Kubernetes. ACK makes it simple to build scalable and highly-available Kubernetes applications that utilize AWS services.

[Container Engine for Kubernetes (OKE)](https://www.oracle.com/cloud-native/container-engine-kubernetes/) is an Oracle-managed container orchestration service that can reduce the time and cost to build modern cloud native applications. Unlike most other vendors, Oracle Cloud Infrastructure provides Container Engine for Kubernetes as a free service that runs on higher-performance, lower-cost compute.

[Anthos](https://cloud.google.com/anthos/docs/concepts/overview) is a modern application management platform that provides a consistent development and operations experience for cloud and on-premises environments.

[Red Hat Openshift](https://www.openshift.com/) is a fully managed Kubernetes platform that provides a foundation for on-premises, hybrid, and multicloud deployments. 

[OKD](https://okd.io/) is a community distribution of Kubernetes optimized for continuous application development and multi-tenant deployment. OKD adds developer and operations-centric tools on top of Kubernetes to enable rapid application development, easy deployment and scaling, and long-term lifecycle maintenance for small and large teams.

[Odo](https://odo.dev/) is a fast, iterative, and straightforward CLI tool for developers who write, build, and deploy applications on Kubernetes and OpenShift.

[Kata Operator](https://github.com/openshift/kata-operator) is an operator to perform lifecycle management (install/upgrade/uninstall) of [Kata Runtime](https://katacontainers.io/) on Openshift as well as Kubernetes cluster.

[Thanos](https://thanos.io/) is a set of components that can be composed into a highly available metric system with unlimited storage capacity, which can be added seamlessly on top of existing Prometheus deployments.

[OpenShift Hive](https://github.com/openshift/hive) is an operator which runs as a service on top of Kubernetes/OpenShift. The Hive service can be used to provision and perform initial configuration of OpenShift 4 clusters.

[Rook](https://rook.io/) is a tool that turns distributed storage systems into self-managing, self-scaling, self-healing storage services. It automates the tasks of a storage administrator: deployment, bootstrapping, configuration, provisioning, scaling, upgrading, migration, disaster recovery, monitoring, and resource management.

[VMware Tanzu](https://tanzu.vmware.com/tanzu) is a centralized management platform for consistently operating and securing your Kubernetes infrastructure and modern applications across multiple teams and private/public clouds.

[Kubespray](https://kubespray.io/) is a tool that combines Kubernetes and Ansible to easily install Kubernetes clusters that can be deployed on [AWS](https://github.com/kubernetes-sigs/kubespray/blob/master/docs/aws.md), GCE, [Azure](https://github.com/kubernetes-sigs/kubespray/blob/master/docs/azure.md), [OpenStack](https://github.com/kubernetes-sigs/kubespray/blob/master/docs/openstack.md), [vSphere](https://github.com/kubernetes-sigs/kubespray/blob/master/docs/vsphere.md), [Packet](https://github.com/kubernetes-sigs/kubespray/blob/master/docs/packet.md) (bare metal), Oracle Cloud Infrastructure (Experimental), or Baremetal.

[KubeInit](https://github.com/kubeinit/kubeinit) provides Ansible playbooks and roles for the deployment and configuration of multiple Kubernetes distributions.

[Rancher](https://rancher.com/) is a complete software stack for teams adopting containers. It addresses the operational and security challenges of managing multiple Kubernetes clusters, while providing DevOps teams with integrated tools for running containerized workloads.

[K3s](https://github.com/rancher/k3s) is a highly available, certified Kubernetes distribution designed for production workloads in unattended, resource-constrained, remote locations or inside IoT appliances. 

[Helm](https://helm.sh/) is a Kubernetes Package Manager tool that makes it easier to install and manage Kubernetes applications.

[Knative](https://knative.dev/) is a Kubernetes-based platform to build, deploy, and manage modern serverless workloads. Knative takes care of the operational overhead details of networking, autoscaling (even to zero), and revision tracking. 

[KubeFlow](https://www.kubeflow.org/) is a tool dedicated to making deployments of machine learning (ML) workflows on Kubernetes simple, portable and scalable.

[Kubebox](https://github.com/astefanutti/kubebox) is a Terminal and Web console for Kubernetes.

[Kubsec](https://github.com/controlplaneio/kubesec) is a Security risk analysis for Kubernetes resources.

[Replex](https://www.replex.io/) is a Kubernetes Governance and Cost Management for the Cloud-Native Enterprise.

[Virtual Kubelet](https://virtual-kubelet.io/) is an open-source [Kubernetes kubelet](https://kubernetes.io/docs/reference/generated/kubelet/) implementation that masquerades as a kubelet.

[Telepresence](https://www.telepresence.io/) is a fast, local development for Kubernetes and OpenShift microservices.

[Weave Scope](https://www.weave.works/oss/scope/) is a tool that automatically detects processes, containers, hosts. No kernel modules, no agents, no special libraries, no coding. It seamless integration with Docker, Kubernetes, DCOS and AWS ECS.

[Nuclio](https://nuclio.io/) is a high-performance "serverless" framework focused on data, I/O, and compute intensive workloads. It is well integrated with popular data science tools, such as [Jupyter](https://jupyter.org/) and [Kubeflow](https://www.kubeflow.org/); supports a variety of data and streaming sources; and supports execution over CPUs and GPUs. 

[Supergiant Control](https://github.com/supergiant/control) is a tool that manages the lifecycle of clusters on your infrastructure and allows deployment of applications via HELM. Its deployment and configuration workflows will help you to get up and running with Kubernetes faster.

[Supergiant Capacity - Beta](https://github.com/supergiant/capacity) is a tool that ensures that the right hardware is available for the required resource load of your Kubernetes cluster at any given time. This helps prevent over-provisioning of your container environment and overspending on your hardware budget. 

[Test suite for Kubernetes](https://github.com/mrahbar/k8s-testsuite) is a test suite consists of two Helm charts for network bandwith testing and load testing a Kuberntes cluster. 

[Keel](https://github.com/keel-hq/keel) is a Kubernetes Operator to automate Helm, DaemonSet, StatefulSet & Deployment updates.

[Kube Monkey](https://github.com/asobti/kube-monkey) is an implementation of Netflix's Chaos Monkey for Kubernetes clusters. It randomly deletes Kubernetes (k8s) pods in the cluster encouraging and validating the development of failure-resilient services.

[Kube State Metrics (KSM)](https://github.com/kubernetes/kube-state-metrics) is a simple service that listens to the Kubernetes API server and generates metrics about the state of the objects. It's not focused on the health of the individual Kubernetes components, but rather on the health of the various objects inside, such as deployments, nodes and pods.

[Sonobuoy](https://sonobuoy.io/) is a diagnostic tool that makes it easier to understand the state of a Kubernetes cluster by running a choice of configuration tests in an accessible and non-destructive manner.

[PowerfulSeal](https://github.com/powerfulseal/powerfulseal) is a powerful testing tool for your Kubernetes clusters, so that you can detect problems as early as possible.

[Test Infra](https://github.com/kubernetes/test-infra) is a repository contains tools and configuration files for the testing and automation needs of the Kubernetes project.

[cAdvisor (Container Advisor)](https://github.com/google/cadvisor) is a tool that provides container users an understanding of the resource usage and performance characteristics of their running containers. It's a running daemon that collects, aggregates, processes, and exports information about running containers. Specifically, for each container it keeps resource isolation parameters, historical resource usage, histograms of complete historical resource usage and network statistics. 

[Etcd](https://etcd.io/) is a distributed key-value store that provides a reliable way to store data that needs to be accessed by a distributed system or cluster of machines. Etcd is used as the backend for service discovery and stores cluster state and configuration for Kubernetes.

[OpenEBS](https://openebs.io/) is a Kubernetes-based tool to create stateful applications using Container Attached Storage.

[Container Storage Interface (CSI)](https://www.architecting.it/blog/container-storage-interface/) is an API that lets container orchestration platforms like Kubernetes seamlessly communicate with stored data via a plug-in.

[MicroK8s](https://microk8s.io/) is a tool that delivers the full Kubernetes experience. In a Fully containerized deployment with compressed over-the-air updates for ultra-reliable operations. It is supported on Linux, Windows, and MacOS.

[Charmed Kubernetes](https://ubuntu.com/kubernetes/features) is a well integrated, turn-key, conformant Kubernetes platform, optimized for your multi-cloud environments developed by Canonical.

[Grafana Kubernetes App](https://grafana.com/grafana/plugins/grafana-kubernetes-app) is a toll that allows you to monitor your Kubernetes cluster's performance. It includes 4 dashboards, Cluster, Node, Pod/Container and Deployment. It allows for the automatic deployment of the required Prometheus exporters and a default scrape config to use with your in cluster Prometheus deployment.

[KubeEdge](https://kubeedge.io/en/) is an open source system for extending native containerized application orchestration capabilities to hosts at Edge.It is built upon kubernetes and provides fundamental infrastructure support for network, app. deployment and metadata synchronization between cloud and edge.

[Lens](https://k8slens.dev/)  is the most powerful IDE for people who need to deal with Kubernetes clusters on a daily basis. It has support for MacOS, Windows and Linux operating systems.

[kind](https://kind.sigs.k8s.io/) is a tool for running local Kubernetes clusters using Docker container “nodes”. It was primarily designed for testing Kubernetes itself, but may be used for local development or CI.

[Flux CD](https://fluxcd.io/) is a tool that automatically ensures that the state of your Kubernetes cluster matches the configuration you've supplied in Git. It uses an operator in the cluster to trigger deployments inside Kubernetes, which means that you don't need a separate continuous delivery tool.


# PowerShell Development

[Back to the Top](https://github.com/mikeroyal/WSL-Guide#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/117888551-19410800-b267-11eb-949b-2c98014da76f.png">
  <br />
</p>

## PowerShell Learning Resources

[Introduction to Bash Shell Scripting by Coursera](https://www.coursera.org/projects/introduction-to-bash-shell-scripting)

[Bash: Shell Script Basics by Pluralsight](https://www.pluralsight.com/courses/bash-shell-scripting)

[Bash/Shell by Codecademy](https://www.codecademy.com/catalog/language/bash)

[Windows Remote Management in Ansible using PowerShell](https://docs.ansible.com/ansible/latest/user_guide/windows_winrm.html)

[Getting Started with PowerShell](https://docs.microsoft.com/en-us/powershell/)

[PowerShell in Azure Cloud Shell](https://aka.ms/cloudshell/powershell-docs)

[Azure Functions using PowerShell](https://docs.microsoft.com/en-us/azure/azure-functions/functions-create-first-function-vs-code?pivots=programming-language-powershell)

[Azure Automation runbooks](https://docs.microsoft.com/en-us/azure/automation/automation-runbook-types)

[Using Visual Studio Code for PowerShell Development](https://docs.microsoft.com/en-us/powershell/scripting/dev-cross-plat/vscode/using-vscode?view=powershell-7)

[Integrated Terminal in Visual Studio Code](https://code.visualstudio.com/docs/editor/integrated-terminal)

[AWS Tools for Windows PowerShell](https://aws.amazon.com/powershell/)

[PowerShell Best Practices and Style Guide](https://poshcode.gitbooks.io/powershell-practice-and-style)

[AWS Command Line Interface and aws-shell Sample for AWS Cloud9](https://docs.aws.amazon.com/cloud9/latest/user-guide/sample-aws-cli.html)

[Configuring Cloud Shell on Google Cloud](https://cloud.google.com/shell/docs/configuring-cloud-shell)

[Google's Shell Style Guide](https://google.github.io/styleguide/shellguide.html)

## PowerShell Tools

[Bash](https://www.gnu.org/software/bash/) is the GNU Project's shell(Bourne Again SHell), which is an sh-compatible shell that integrates together useful features from the Korn shell (ksh) and the C shell (csh).

[PowerShell Core](https://microsoft.com/PowerShell) is a cross-platform (Windows, Linux, and macOS) automation and configuration tool/framework that works well with your existing tools and is optimized for dealing with structured data (JSON, CSV, XML, etc.), REST APIs, and object models. It also includes a command-line shell, an associated scripting language and a framework for processing cmdlets.

[Azure PowerShell](https://docs.microsoft.com/en-us/powershell/azure/overview) is a set of cmdlets for managing Microsoft Azure resources directly from the PowerShell command line. 

[Windows Subsystem for Linux (WSL)](https://docs.microsoft.com/en-us/learn/modules/get-started-with-windows-subsystem-for-linux/) is a compatibility layer developed by Microsoft for running Linux binary executables in a Executable/Linkable Format natively on Windows 10 and Windows Server.

[AWS Shell](https://aws.amazon.com/cli/) is a command-line shell program that provides convenience and productivity features to help both new and advanced users of the AWS Command Line Interface.

[Google Cloud Shell](https://cloud.google.com/shell/) is a free admin machine with browser-based command-line access for managing your infrastructure and applications on Google Cloud Platform.

[VS Code Bash Debug](https://marketplace.visualstudio.com/items?itemName=rogalmic.bash-debug) is a bash debugger GUI frontend based on awesome bashdb scripts (bashdb now included in package).

[VS Code Bash IDE](https://marketplace.visualstudio.com/items?itemName=mads-hartmann.bash-ide-vscode) is a Visual Studio Code extension utilizing the [bash language server](https://github.com/bash-lsp/bash-language-server/blob/master/bash-lsp), that is based on [Tree Sitter](https://github.com/tree-sitter/tree-sitter) and its [grammar for Bash](https://github.com/tree-sitter/tree-sitter-bash) and supports [explainshell](https://explainshell.com/) integration.


#  Wayland Development
[Back to the Top](https://github.com/mikeroyal/WSL-Guide#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/104235197-79cf4e00-5409-11eb-97a6-a12f7bd8ad2a.png">
  <br />

</p>

 ## Wayland Learning Resources

[Wayland](https://wayland.freedesktop.org) is a protocol for a compositor to talk to its clients as well as a C library implementation of that protocol. The compositor can be a standalone display server running on Linux kernel modesetting and evdev input devices, an [X application](https://www.x.org/wiki/XServer/), or a wayland client itself.

[Wayland Architecture](https://wayland.freedesktop.org/architecture.html)

[Wayland Documentation](https://wayland.freedesktop.org/docs/html/)

[Sotfware Toolkits that have Wayland support right now](https://wayland.freedesktop.org/toolkits.html)

[Contribution instructions for Wayland](https://gitlab.freedesktop.org/wayland/wayland/blob/master/CONTRIBUTING.md)

[Contribution instructions for Weston](https://gitlab.freedesktop.org/wayland/weston/blob/master/CONTRIBUTING.md)

[Reporting Wayland bugs](https://gitlab.freedesktop.org/wayland/wayland/issues)

[Reporting Weston bugs](https://gitlab.freedesktop.org/wayland/weston/issues)

[WSLG: X11 and Wayland Applications in Windows Subsystem for Linux(WSL2)](https://linuxplumbersconf.org/event/9/contributions/611/attachments/702/1298/XDC2020_-_X11_and_Wayland_applications_in_WSL.pdf)

[Qt Wayland Compositor](https://doc.qt.io/qt-5/qtwaylandcompositor-index.html)

[Qt Wayland Compositor Examples](https://doc.qt.io/qt-5/qtwaylandcompositor-examples.html)

[Wayland on ArchWiki](https://wiki.archlinux.org/index.php/Wayland)

[Sway on ArchWiki](https://wiki.archlinux.org/index.php/Sway)

[Wayland on Ubuntu Wiki](https://wiki.ubuntu.com/Wayland)

[Wayland on Debian Wiki](https://wiki.debian.org/Wayland)

[The Wayland Display Server on Fedora Docs](https://docs.fedoraproject.org/en-US/fedora/rawhide/system-administrators-guide/Wayland/)

[Wayland features on Fedora Project Wiki](https://fedoraproject.org/wiki/Wayland_features)

[Wayland on GNOME Wiki](https://wiki.gnome.org/Initiatives/Wayland)

[KWin/Wayland on KDE Community Wiki](https://community.kde.org/index.php?title=KWin/Wayland)

[Wayland Desktop Landscape on Gentoo Wiki](https://wiki.gentoo.org/wiki/Wayland_Desktop_Landscape)

[Wayland in Void Linux Handbook](https://docs.voidlinux.org/config/graphical-session/wayland.html)

[Wayland on Enlightenment DE](https://www.enlightenment.org/about-wayland)

## Tools

[Weston](https://gitlab.freedesktop.org/wayland/weston) is a lightweight and functional Wayland compositor.

[XWayland](https://wayland.freedesktop.org/xserver.html) is an X Server running as a Wayland client(for backwards compatibility), allowing the [Xorg server](https://www.x.org/wiki/XServer/) can be modified to use wayland input devices for input and forward either the root window or individual top-level windows as wayland surfaces.

[KWin](https://community.kde.org/KWin/Wayland) is the window manager for the KDE Plasma Desktop. It gives you complete control over your windows, making sure they're not in the way but aid you in your task. It paints the window decoration, the bar on top of every window with (configurable) buttons like close, maximize and minimize.

[Qt](https://www.qt.io/) is the faster, smarter way to create innovative devices, modern UIs & applications for multiple screens. It is one of the most popular toolkits for the Wayland and X11 windowing.

[GTK](https://www.gtk.org/) is a free and open source cross-platform widget toolkit for creating graphical user interfaces developed by [GNOME Project](https://www.gnome.org/). It is one of the most popular toolkits for the Wayland and X11 windowing.

[NVIDIA Wayland EGL External Platform library](https://github.com/NVIDIA/egl-wayland) is a work-in-progress implementation of a EGL External Platform library to add client-side Wayland support to EGL on top of EGLDevice and EGLStream families of extensions.

[NVIDIA EGL External Platform Interface](https://github.com/NVIDIA/eglexternalplatform) is a work-in-progress specification of the EGL External Platform interface for writing EGL platforms and their interactions with modern window systems on top of existing low-level EGL platform implementations. This keeps window system implementation specifics out of EGL drivers by using application-facing EGL functions.

[Sway](https://swaywm.org/) is an [i3](https://i3wm.org/)-compatible Wayland compositor. 

[wlroots](https://github.com/swaywm/wlroots) is a modular Wayland compositor library.

[WayfireWM](https://github.com/WayfireWM/wayfire) is a 3D Wayland compositor, inspired by [Compiz](https://launchpad.net/compiz) and based on [wlroots](https://github.com/swaywm/wlroots).

[SDDM](https://github.com/sddm/sddm) is a modern display manager for X11 and Wayland aiming to be fast, simple and beautiful. It uses modern technologies like QtQuick, which in turn gives the designer the ability to create smooth, animated user interfaces.

[x11docker](https://github.com/mviereck/x11docker) is an application that you allows to run graphical desktop applications (and entire desktops) in Docker Linux containers.

[Mako](https://github.com/emersion/mako) is alightweight notification daemon for Wayland. It also works on [Sway](https://swaywm.org/).

[Wayland-rs](https://github.com/Smithay/wayland-rs) is a Rust implementation of the wayland protocol (client and server).

[Wine-wayland](https://github.com/varmd/wine-wayland) is an application that allows you to running DX9/DX11 and Vulkan games using pure Wayland and Wine/DXVK.

# Networking

[Back to the Top](https://github.com/mikeroyal/WSL-Guide#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/82833053-d1687b80-9e71-11ea-8c6d-074100f2f54b.png">
  <br />
</p>

## Networking Learning Resources
  
[AWS Certified Security - Specialty Certification](https://aws.amazon.com/certification/certified-security-specialty/)

[Microsoft Certified: Azure Security Engineer Associate](https://docs.microsoft.com/en-us/learn/certifications/azure-security-engineer)

[Google Cloud Certified Professional Cloud Security Engineer](https://cloud.google.com/certification/cloud-security-engineer)

[Cisco Security Certifications](https://www.cisco.com/c/en/us/training-events/training-certifications/certifications/security.html)

[The Red Hat Certified Specialist in Security: Linux](https://www.redhat.com/en/services/training/ex415-red-hat-certified-specialist-security-linux-exam)

[Linux Professional Institute LPIC-3 Enterprise Security Certification](https://www.lpi.org/our-certifications/lpic-3-303-overview)

[Cybersecurity Training and Courses from IBM Skills](https://www.ibm.com/skills/topics/cybersecurity/)

[Cybersecurity Courses and Certifications by Offensive Security](https://www.offensive-security.com/courses-and-certifications/)  
  
[Citrix Certified Associate – Networking(CCA-N)](http://training.citrix.com/cms/index.php/certification/networking/)

[Citrix Certified Professional – Virtualization(CCP-V)](https://www.globalknowledge.com/us-en/training/certification-prep/brands/citrix/section/virtualization/citrix-certified-professional-virtualization-ccp-v/)

[CCNP Routing and Switching](https://learningnetwork.cisco.com/s/ccnp-enterprise)

[Certified Information Security Manager(CISM)](https://www.isaca.org/credentialing/cism)

[Wireshark Certified Network Analyst (WCNA)](https://www.wiresharktraining.com/certification.html)

[Juniper Networks Certification Program Enterprise (JNCP)](https://www.juniper.net/us/en/training/certification/)

[Networking courses and specializations from Coursera](https://www.coursera.org/browse/information-technology/networking)

[Network & Security Courses from Udemy](https://www.udemy.com/courses/it-and-software/network-and-security/)

[Network & Security Courses from edX](https://www.edx.org/learn/cybersecurity)

## Networking Tools & Concepts

[Qt Network Authorization](https://doc.qt.io/qt-6/qtnetworkauth-index.html) is a tool that provides a set of APIs that enable Qt applications to obtain limited access to online accounts and HTTP services without exposing users' passwords.

[cURL](https://curl.se/) is a computer software project providing a library and command-line tool for transferring data using various network protocols(HTTP, HTTPS, FTP, FTPS, SCP, SFTP, TFTP, DICT, TELNET, LDAP LDAPS, MQTT, POP3, POP3S, RTMP, RTMPS, RTSP, SCP, SFTP, SMB, SMBS, SMTP or SMTPS). cURL is also used in cars, television sets, routers, printers, audio equipment, mobile phones, tablets, settop boxes, media players and is the Internet transfer engine for thousands of software applications in over ten billion installations.

[cURL Fuzzer](https://github.com/curl/curl-fuzzer) is a quality assurance testing for the curl project.

[DoH](https://github.com/curl/doh) is a stand-alone application for DoH (DNS-over-HTTPS) name resolves and lookups.

[Authelia](https://www.authelia.com/) is an open-source highly-available authentication server providing single sign-on capability and two-factor authentication to applications running behind [NGINX](https://nginx.org/en/).

[nginx(engine x)](https://nginx.org/en/) is an HTTP and reverse proxy server, a mail proxy server, and a generic TCP/UDP proxy server, originally written by Igor Sysoev.

[Proxmox Virtual Environment(VE)](https://www.proxmox.com/en/) is a complete open-source platform for enterprise virtualization. It inlcudes a built-in web interface that you can easily manage VMs and containers, software-defined storage and networking, high-availability clustering, and multiple out-of-the-box tools on a single solution.

[Wireshark](https://www.wireshark.org/) is a very popular network protocol analyzer that is commonly used for network troubleshooting, analysis, and communications protocol development. Learn more about the other useful [Wireshark Tools](https://wiki.wireshark.org/Tools) available.

[HTTPie](https://github.com/httpie/httpie) is a command-line HTTP client. Its goal is to make CLI interaction with web services as human-friendly as possible. HTTPie is designed for testing, debugging, and generally interacting with APIs & HTTP servers.

[HTTPStat](https://github.com/reorx/httpstat) is a tool that visualizes curl statistics in a simple layout.

[Wuzz](https://github.com/asciimoo/wuzz) is an interactive cli tool for HTTP inspection. It can be used to inspect/modify requests copied from the browser's network inspector with the "copy as cURL" feature.

[Websocat](https://github.com/vi/websocat) is a ommand-line client for WebSockets, like netcat (or curl) for ws:// with advanced socat-like functions.

   - Connection: In networking, a connection refers to pieces of related information that are transferred through a network. This generally infers that a connection is built before the data transfer (by following the procedures laid out in a protocol) and then is deconstructed at the at the end of the data transfer.

   - Packet: A packet is, generally speaking, the most basic unit that is transferred over a network. When communicating over a network, packets are the envelopes that carry your data (in pieces) from one end point to the other.

Packets have a header portion that contains information about the packet including the source and destination, timestamps, network hops. The main portion of a packet contains the actual data being transferred. It is sometimes called the body or the payload.

   - Network Interface: A network interface can refer to any kind of software interface to networking hardware. For instance, if you have two network cards in your computer, you can control and configure each network interface associated with them individually.

A network interface may be associated with a physical device, or it may be a representation of a virtual interface. The "loop-back" device, which is a virtual interface to the local machine, is an example of this.

   - LAN: LAN stands for "local area network". It refers to a network or a portion of a network that is not publicly accessible to the greater internet. A home or office network is an example of a LAN.

   - WAN: WAN stands for "wide area network". It means a network that is much more extensive than a LAN. While WAN is the relevant term to use to describe large, dispersed networks in general, it is usually meant to mean the internet, as a whole.
If an interface is connected to the WAN, it is generally assumed that it is reachable through the internet.

   - Protocol: A protocol is a set of rules and standards that basically define a language that devices can use to communicate. There are a great number of protocols in use extensively in networking, and they are often implemented in different layers.

Some low level protocols are TCP, UDP, IP, and ICMP. Some familiar examples of application layer protocols, built on these lower protocols, are HTTP (for accessing web content), SSH, TLS/SSL, and FTP.

   - Port: A port is an address on a single machine that can be tied to a specific piece of software. It is not a physical interface or location, but it allows your server to be able to communicate using more than one application.

   - Firewall: A firewall is a program that decides whether traffic coming into a server or going out should be allowed. A firewall usually works by creating rules for which type of traffic is acceptable on which ports. Generally, firewalls block ports that are not used by a specific application on a server.

   - NAT: Network address translation is a way to translate requests that are incoming into a routing server to the relevant devices or servers that it knows about in the LAN. This is usually implemented in physical LANs as a way to route requests through one IP address to the necessary backend servers.

   - VPN: Virtual private network is a means of connecting separate LANs through the internet, while maintaining privacy. This is used as a means of connecting remote systems as if they were on a local network, often for security reasons.

## Network Layers

While networking is often discussed in terms of topology in a horizontal way, between hosts, its implementation is layered in a vertical fashion throughout a computer or network. This means is that there are multiple technologies and protocols that are built on top of each other in order for communication to function more easily. Each successive, higher layer abstracts the raw data a little bit more, and makes it simpler to use for applications and users. It also allows you to leverage lower layers in new ways without having to invest the time and energy to develop the protocols and applications that handle those types of traffic.

As data is sent out of one machine, it begins at the top of the stack and filters downwards. At the lowest level, actual transmission to another machine takes place. At this point, the data travels back up through the layers of the other computer. Each layer has the ability to add its own "wrapper" around the data that it receives from the adjacent layer, which will help the layers that come after decide what to do with the data when it is passed off.

One method of talking about the different layers of network communication is the OSI model. OSI stands for [Open Systems Interconnect](https://en.wikipedia.org/wiki/OSI_model).This model defines seven separate layers. The layers in this model are:

   - Application: The application layer is the layer that the users and user-applications most often interact with. Network communication is discussed in terms of availability of resources, partners to communicate with, and data synchronization.

   - Presentation: The presentation layer is responsible for mapping resources and creating context. It is used to translate lower level networking data into data that applications expect to see.

   - Session: The session layer is a connection handler. It creates, maintains, and destroys connections between nodes in a persistent way.

   - Transport: The transport layer is responsible for handing the layers above it a reliable connection. In this context, reliable refers to the ability to verify that a piece of data was received intact at the other end of the connection. This layer can resend information that has been dropped or corrupted and can acknowledge the receipt of data to remote computers.

   - Network: The network layer is used to route data between different nodes on the network. It uses addresses to be able to tell which computer to send information to. This layer can also break apart larger messages into smaller chunks to be reassembled on the opposite end.

   - Data Link: This layer is implemented as a method of establishing and maintaining reliable links between different nodes or devices on a network using existing physical connections.

   - Physical: The physical layer is responsible for handling the actual physical devices that are used to make a connection. This layer involves the bare software that manages physical connections as well as the hardware itself (like Ethernet).

The TCP/IP model, more commonly known as the Internet protocol suite, is another layering model that is simpler and has been widely adopted.It defines the four separate layers, some of which overlap with the OSI model:

   - Application: In this model, the application layer is responsible for creating and transmitting user data between applications. The applications can be on remote systems, and should appear to operate as if locally to the end user.
The communication takes place between peers network.

   - Transport: The transport layer is responsible for communication between processes. This level of networking utilizes ports to address different services. It can build up unreliable or reliable connections depending on the type of protocol used.

   - Internet: The internet layer is used to transport data from node to node in a network. This layer is aware of the endpoints of the connections, but does not worry about the actual connection needed to get from one place to another. IP addresses are defined in this layer as a way of reaching remote systems in an addressable manner.

   - Link: The link layer implements the actual topology of the local network that allows the internet layer to present an addressable interface. It establishes connections between neighboring nodes to send data.

### Interfaces
**Interfaces** are networking communication points for your computer. Each interface is associated with a physical or virtual networking device. Typically, your server will have one configurable network interface for each Ethernet or wireless internet card you have. In addition, it will define a virtual network interface called the "loopback" or localhost interface. This is used as an interface to connect applications and processes on a single computer to other applications and processes. You can see this referenced as the "lo" interface in many tools.

## Network Protocols

Networking works by piggybacks on a number of different protocols on top of each other. In this way, one piece of data can be transmitted using multiple protocols encapsulated within one another.

**Media Access Control(MAC)** is a communications protocol that is used to distinguish specific devices. Each device is supposed to get a unique MAC address during the manufacturing process that differentiates it from every other device on the internet. Addressing hardware by the MAC address allows you to reference a device by a unique value even when the software on top may change the name for that specific device during operation. Media access control is one of the only protocols from the link layer that you are likely to interact with on a regular basis.

**The IP protocol** is one of the fundamental protocols that allow the internet to work. IP addresses are unique on each network and they allow machines to address each other across a network. It is implemented on the internet layer in the IP/TCP model. Networks can be linked together, but traffic must be routed when crossing network boundaries. This protocol assumes an unreliable network and multiple paths to the same destination that it can dynamically change between. There are a number of different implementations of the protocol. The most common implementation today is IPv4, although IPv6 is growing in popularity as an alternative due to the scarcity of IPv4 addresses available and improvements in the protocols capabilities.

**ICMP: internet control message protocol** is used to send messages between devices to indicate the availability or error conditions. These packets are used in a variety of network diagnostic tools, such as ping and traceroute. Usually ICMP packets are transmitted when a packet of a different kind meets some kind of a problem. Basically, they are used as a feedback mechanism for network communications.

**TCP: Transmission control protocol** is implemented in the transport layer of the IP/TCP model and is used to establish reliable connections. TCP is one of the protocols that encapsulates data into packets. It then transfers these to the remote end of the connection using the methods available on the lower layers. On the other end, it can check for errors, request certain pieces to be resent, and reassemble the information into one logical piece to send to the application layer. The protocol builds up a connection prior to data transfer using a system called a three-way handshake. This is a way for the two ends of the communication to acknowledge the request and agree upon a method of ensuring data reliability. After the data has been sent, the connection is torn down using a similar four-way handshake. TCP is the protocol of choice for many of the most popular uses for the internet, including WWW, FTP, SSH, and email. It is safe to say that the internet we know today would not be here without TCP.

**UDP: User datagram protocol** is a popular companion protocol to TCP and is also implemented in the transport layer. The fundamental difference between UDP and TCP is that UDP offers unreliable data transfer. It does not verify that data has been received on the other end of the connection. This might sound like a bad thing, and for many purposes, it is. However, it is also extremely important for some functions. It’s not required to wait for confirmation that the data was received and forced to resend data, UDP is much faster than TCP. It does not establish a connection with the remote host, it simply fires off the data to that host and doesn't care if it is accepted or not. Since UDP is a simple transaction, it is useful for simple communications like querying for network resources. It also doesn't maintain a state, which makes it great for transmitting data from one machine to many real-time clients. This makes it ideal for VOIP, games, and other applications that cannot afford delays.

**HTTP: Hypertext transfer protocol** is a protocol defined in the application layer that forms the basis for communication on the web. HTTP defines a number of functions that tell the remote system what you are requesting. For instance, GET, POST, and DELETE all interact with the requested data in a different way.

**FTP: File transfer protocol** is in the application layer and provides a way of transferring complete files from one host to another. It is inherently insecure, so it is not recommended for any externally facing network unless it is implemented as a public, download-only resource.

**DNS: Domain name system** is an application layer protocol used to provide a human-friendly naming mechanism for internet resources. It is what ties a domain name to an IP address and allows you to access sites by name in your browser.

**SSH: Secure shell** is an encrypted protocol implemented in the application layer that can be used to communicate with a remote server in a secure way. Many additional technologies are built around this protocol because of its end-to-end encryption and ubiquity. There are many other protocols that we haven't covered that are equally important. However, this should give you a good overview of some of the fundamental technologies that make the internet and networking possible.

[REST(REpresentational State Transfer)](https://www.codecademy.com/articles/what-is-rest) is an architectural style for providing standards between computer systems on the web, making it easier for systems to communicate with each other.

[JSON Web Token (JWT)](https://jwt.io) is a compact URL-safe means of representing claims to be transferred between two parties. The claims in a JWT are encoded as a JSON object that is digitally signed using JSON Web Signature (JWS).

[OAuth 2.0](https://oauth.net/2/) is an open source authorization framework that enables applications to obtain limited access to user accounts on an HTTP service, such as Amazon, Google, Facebook, Microsoft, Twitter GitHub, and DigitalOcean. It works by delegating user authentication to the service that hosts the user account, and authorizing third-party applications to access the user account.

## Virtualization

[KVM (for Kernel-based Virtual Machine)](https://www.linux-kvm.org/page/Main_Page) is a full virtualization solution for Linux on x86 hardware containing virtualization extensions (Intel VT or AMD-V). It consists of a loadable kernel module, kvm.ko, that provides the core virtualization infrastructure and a processor specific module, kvm-intel.ko or kvm-amd.ko.

[QEMU](https://www.qemu.org) is a fast processor emulator using a portable dynamic translator. QEMU emulates a full system, including a processor and various peripherals. It can be used to launch a different Operating System without rebooting the PC or to debug system code.

[Hyper-V](https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/) enables running virtualized computer systems on top of a physical host. These virtualized systems can be used and managed just as if they were physical computer systems, however they exist in virtualized and isolated environment. Special software called a hypervisor manages access between the virtual systems and the physical hardware resources. Virtualization enables quick deployment of computer systems, a way to quickly restore systems to a previously known good state, and the ability to migrate systems between physical hosts.

[VirtManager](https://github.com/virt-manager/virt-manager) is a graphical tool for managing virtual machines via libvirt. Most usage is with QEMU/KVM virtual machines, but Xen and libvirt LXC containers are well supported. Common operations for any libvirt driver should work.

[oVirt](https://www.ovirt.org) is an open-source distributed virtualization solution, designed to manage your entire enterprise infrastructure. oVirt uses the trusted KVM hypervisor and is built upon several other community projects, including libvirt, Gluster, PatternFly, and Ansible.Founded by Red Hat as a community project on which Red Hat Enterprise Virtualization is based allowing for centralized management of virtual machines, compute, storage and networking resources, from an easy-to-use web-based front-end with platform independent access.

[Xen](https://github.com/xen-project/xen) is focused on advancing virtualization in a number of different commercial and open source applications, including server virtualization, Infrastructure as a Services (IaaS), desktop virtualization, security applications, embedded and hardware appliances, and automotive/aviation.

[Ganeti](https://github.com/ganeti/ganeti) is a virtual machine cluster management tool built on top of existing virtualization technologies such as Xen or KVM and other open source software. Once installed, the tool assumes management of the virtual instances (Xen DomU).

[Packer](https://www.packer.io/) is an open source tool for creating identical machine images for multiple platforms from a single source configuration. Packer is lightweight, runs on every major operating system, and is highly performant, creating machine images for multiple platforms in parallel. Packer does not replace configuration management like Chef or Puppet. In fact, when building images, Packer is able to use tools like Chef or Puppet to install software onto the image.

[Vagrant](https://www.vagrantup.com/) is a tool for building and managing virtual machine environments in a single workflow. With an easy-to-use workflow and focus on automation, Vagrant lowers development environment setup time, increases production parity, and makes the "works on my machine" excuse a relic of the past. It provides easy to configure, reproducible, and portable work environments built on top of industry-standard technology and controlled by a single consistent workflow to help maximize the productivity and flexibility of you and your team.

[VMware Workstation](https://www.vmware.com/products/workstation-pro.html) is a hosted hypervisor that runs on x64 versions of Windows and Linux operating systems; it enables users to set up virtual machines on a single physical machine, and use them simultaneously along with the actual machine.

# Databases

[Back to the Top](https://github.com/mikeroyal/WSL-Guide#table-of-contents)

## Database Learning Resources

[SQL](https://en.wikipedia.org/wiki/SQL) is a standard language for storing, manipulating and retrieving data in relational databases.

[SQL Tutorial by W3Schools](https://www.w3schools.com/sql/)

[Learn SQL Skills Online from Coursera](https://www.coursera.org/courses?query=sql)

[SQL Courses Online from Udemy](https://www.udemy.com/topic/sql/) 

[SQL Online Training Courses from LinkedIn Learning](https://www.linkedin.com/learning/topics/sql)

[Learn SQL For Free from Codecademy](https://www.codecademy.com/learn/learn-sql)

[GitLab's SQL Style Guide](https://about.gitlab.com/handbook/business-ops/data-team/platform/sql-style-guide/)

[OracleDB SQL Style Guide Basics](https://oracle.readthedocs.io/en/latest/sql/basics/style-guide.html)

[Tableau CRM: BI Software and Tools](https://www.salesforce.com/products/crm-analytics/overview/)

[Databases on AWS](https://aws.amazon.com/products/databases/)

[Best Practices and Recommendations for SQL Server Clustering in AWS EC2.](https://docs.aws.amazon.com/AWSEC2/latest/WindowsGuide/aws-sql-clustering.html)

[Connecting from Google Kubernetes Engine to a Cloud SQL instance.](https://cloud.google.com/sql/docs/mysql/connect-kubernetes-engine)

[Educational Microsoft Azure SQL resources](https://docs.microsoft.com/en-us/sql/sql-server/educational-sql-resources?view=sql-server-ver15)

[MySQL Certifications](https://www.mysql.com/certification/)

[SQL vs. NoSQL Databases: What's the Difference?](https://www.ibm.com/cloud/blog/sql-vs-nosql)

[What is NoSQL?](https://aws.amazon.com/nosql/)

## Databases and Tools

[Azure Data Studio](https://github.com/Microsoft/azuredatastudio) is an open source data management tool that enables working with SQL Server, Azure SQL DB and SQL DW from Windows, macOS and Linux.

[Azure SQL Database](https://azure.microsoft.com/en-us/services/sql-database/)  is the intelligent, scalable, relational database service built for the cloud. It’s evergreen and always up to date, with AI-powered and automated features that optimize performance and durability for you. Serverless compute and Hyperscale storage options automatically scale resources on demand, so you can focus on building new applications without worrying about storage size or resource management.

[Azure SQL Managed Instance](https://azure.microsoft.com/en-us/services/azure-sql/sql-managed-instance/) is a fully managed SQL Server Database engine instance that's hosted in Azure and placed in your network. This deployment model makes it easy to lift and shift your on-premises applications to the cloud with very few application and database changes. Managed instance has split compute and storage components.

[Azure Synapse Analytics](https://azure.microsoft.com/en-us/services/synapse-analytics/) is a limitless analytics service that brings together enterprise data warehousing and Big Data analytics. It gives you the freedom to query data on your terms, using either serverless or provisioned resources at scale. It brings together the best of the SQL technologies used in enterprise data warehousing, Spark technologies used in big data analytics, and Pipelines for data integration and ETL/ELT.

[MSSQL for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=ms-mssql.mssql) is an extension for developing Microsoft SQL Server, Azure SQL Database and SQL Data Warehouse everywhere with a rich set of functionalities.

[SQL Server Data Tools (SSDT)](https://docs.microsoft.com/en-us/sql/ssdt/download-sql-server-data-tools-ssdt) is a development tool for building SQL Server relational databases, Azure SQL Databases, Analysis Services (AS) data models, Integration Services (IS) packages, and Reporting Services (RS) reports. With SSDT, a developer can design and deploy any SQL Server content type with the same ease as they would develop an application in Visual Studio or Visual Studio Code.

[Bulk Copy Program](https://docs.microsoft.com/en-us/sql/tools/bcp-utility) is a command-line tool that comes with Microsoft SQL Server. BCP, allows you to import and export large amounts of data in and out of SQL Server databases quickly snd efficeiently.

[SQL Server Migration Assistant](https://www.microsoft.com/en-us/download/details.aspx?id=54258) is a tool from Microsoft that simplifies database migration process from Oracle to SQL Server, Azure SQL Database, Azure SQL Database Managed Instance and Azure SQL Data Warehouse.

[SQL Server Integration Services](https://docs.microsoft.com/en-us/sql/integration-services/sql-server-integration-services?view=sql-server-ver15) is a development platform for building enterprise-level data integration and data transformations solutions. Use Integration Services to solve complex business problems by copying or downloading files, loading data warehouses, cleansing and mining data, and managing SQL Server objects and data.

[SQL Server Business Intelligence(BI)](https://www.microsoft.com/en-us/sql-server/sql-business-intelligence) is a collection of tools in Microsoft's SQL Server for transforming raw data into information businesses can use to make decisions.

[Tableau](https://www.tableau.com/) is a Data Visualization software used in relational databases, cloud databases, and spreadsheets. Tableau was acquired by [Salesforce in August 2019](https://investor.salesforce.com/press-releases/press-release-details/2019/Salesforce-Completes-Acquisition-of-Tableau/default.aspx).

[DataGrip](https://www.jetbrains.com/datagrip/) is a professional DataBase IDE developed by Jet Brains that provides context-sensitive code completion, helping you to write SQL code faster. Completion is aware of the tables structure, foreign keys, and even database objects created in code you're editing.

[RStudio](https://rstudio.com/) is an integrated development environment for R and Python, with a console, syntax-highlighting editor that supports direct code execution, and tools for plotting, history, debugging and workspace management.

[MySQL](https://www.mysql.com/) is a fully managed database service to deploy cloud-native applications using the world's most popular open source database. 

[PostgreSQL](https://www.postgresql.org/) is a powerful, open source object-relational database system with over 30 years of active development that has earned it a strong reputation for reliability, feature robustness, and performance.

[Amazon DynamoDB](https://aws.amazon.com/dynamodb/) is a key-value and document database that delivers single-digit millisecond performance at any scale. It is a fully managed, multiregion, multimaster, durable database with built-in security, backup and restore, and in-memory caching for internet-scale applications.

[FoundationDB](https://www.foundationdb.org/) is an open source distributed database designed to handle large volumes of structured data across clusters of commodity servers. It organizes data as an ordered key-value store and employs ACID transactions for all operations. It is especially well-suited for read/write workloads but also has excellent performance for write-intensive workloads. FoundationDB was acquired by [Apple in 2015](https://techcrunch.com/2015/03/24/apple-acquires-durable-database-company-foundationdb/).

[CouchbaseDB](https://www.couchbase.com/) is an open source distributed [multi-model NoSQL document-oriented database](https://en.wikipedia.org/wiki/Multi-model_database). It creates a key-value store with managed cache for sub-millisecond data operations, with purpose-built indexers for efficient queries and a powerful query engine for executing SQL queries.

[IBM DB2](https://www.ibm.com/analytics/db2) is a collection of hybrid data management products offering a complete suite of AI-empowered capabilities designed to help you manage both structured and unstructured data on premises as well as in private and public cloud environments. Db2 is built on an intelligent common SQL engine designed for scalability and flexibility.

[MongoDB](https://www.mongodb.com/) is a document database meaning it stores data in JSON-like documents. 

[OracleDB](https://www.oracle.com/database/) is a powerful fully managed database helps developers manage business-critical data with the highest availability, reliability, and security.

[MariaDB](https://mariadb.com/) is an enterprise open source database solution for modern, mission-critical applications.

[SQLite](https://sqlite.org/index.html) is a C-language library that implements a small, fast, self-contained, high-reliability, full-featured, SQL database engine.SQLite is the most used database engine in the world. SQLite is built into all mobile phones and most computers and comes bundled inside countless other applications that people use every day.

[SQLite Database Browser](https://sqlitebrowser.org/) is an open source SQL tool that allows users to create, design and edits SQLite database files. It lets users show a log of all the SQL commands that have been issued by them and by the application itself. 

[dbWatch](https://www.dbwatch.com/) is a complete database monitoring/management solution for SQL Server, Oracle, PostgreSQL, Sybase, MySQL and Azure. Designed for proactive management and automation of routine maintenance in large scale on-premise, hybrid/cloud database environments.

[Cosmos DB Profiler](https://hibernatingrhinos.com/products/cosmosdbprof) is a real-time visual debugger allowing a development team to gain valuable insight and perspective into their usage of Cosmos DB database. It identifies over a dozen suspicious behaviors from your application’s interaction with Cosmos DB.

[Adminer](https://www.adminer.org/) is an SQL management client tool for managing databases, tables, relations, indexes, users. Adminer has support for all the popular database management systems such as MySQL, MariaDB, PostgreSQL, SQLite, MS SQL, Oracle, Firebird, SimpleDB, Elasticsearch and MongoDB.

[DBeaver](https://dbeaver.io/) is an open source database tool for developers and database administrators. It offers supports for JDBC compliant databases such as MySQL, Oracle, IBM DB2, SQL Server, Firebird, SQLite, Sybase, Teradata, Firebird, Apache Hive, Phoenix, and Presto.

[DbVisualizer](https://dbvis.com/) is a SQL management tool that allows users to manage a wide range of databases such as Oracle, Sybase, SQL Server, MySQL, H3, and SQLite.

[AppDynamics Database](https://www.appdynamics.com/supported-technologies/database) is a management product for Microsoft SQL Server. With AppDynamics you can monitor and trend key performance metrics such as resource consumption, database objects, schema statistics and more, allowing you to proactively tune and fix issues in a High-Volume Production Environment.

[Toad](https://www.quest.com/toad/) is a SQL Server DBMS toolset developed by Quest. It increases productivity by using extensive automation, intuitive workflows, and built-in expertise. This SQL management tool resolve issues, manage change and promote the highest levels of code quality for both relational and non-relational databases.

[Lepide SQL Server](https://www.lepide.com/sql-storage-manager/) is an open source storage manager utility to analyse the performance of SQL Servers. It provides a complete overview of all configuration and permission changes being made to your SQL Server environment through an easy-to-use, graphical user interface.

[Sequel Pro](https://sequelpro.com/) is a fast MacOS database management tool for working with MySQL. This SQL management tool helpful for interacting with your database by easily to adding new databases, new tables, and new rows.

# Setting up a macOS Workspace VM

[Back to the Top](https://github.com/mikeroyal/WSL-Guide#table-of-contents)

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

## Contribute

- [x] If would you like to contribute to this guide simply make a [Pull Request](https://github.com/mikeroyal/WSL-Guide/pulls).


## License
[Back to the Top](https://github.com/mikeroyal/WSL-Guide/blob/master/README.md#table-of-contents)

Distributed under the [Creative Commons Attribution 4.0 International (CC BY 4.0) Public License](https://creativecommons.org/licenses/by/4.0/).
