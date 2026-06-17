# P8OS

**Your computer, anywhere.**

P8OS is an experimental web-native operating system focused on portability, simplicity, and user ownership. The project aims to provide a lightweight computing environment that can boot from removable media, run on modest hardware, and deliver a consistent personal workspace across different machines.

Unlike traditional desktop operating systems that are centered around native applications, P8OS is built around a web application runtime. Applications are primarily delivered as web-based experiences while retaining the ability to access native operating system capabilities through secure runtime APIs.

The long-term vision is to make the user's environment portable rather than the hardware itself. A user should be able to boot P8OS from a USB drive on any compatible computer and immediately access their applications, files, settings, and services.

## Vision

P8OS is designed around a few core principles:

* Portable computing
* Web-first application platform
* Lightweight system requirements
* Privacy-focused architecture
* Self-hosted friendly ecosystem
* Cloud-optional operation
* Offline-capable applications

The operating system should remain usable even when disconnected from the internet. Core utilities such as settings, files, notes, and calculators are intended to function locally while cloud-connected applications can synchronize when connectivity is available.

## Architecture

The planned architecture consists of several major layers:

```text
Hardware
    ↓
Linux Kernel
    ↓
System Services
    ↓
P8 Shell
    ↓
P8 Runtime
    ↓
Applications
```

### Linux Kernel

P8OS uses a custom-configured Linux kernel optimized for fast boot times, low resource usage, and removable media deployment.

### P8 Shell

The shell is the primary user interface presented after boot.

Responsibilities include:

* Application launcher
* Workspace management
* Search
* Lock screen
* Notifications
* System integration

### P8 Runtime

The runtime serves as the application execution environment.

Responsibilities include:

* Application discovery
* Manifest loading
* Web application rendering
* Runtime APIs
* Permission enforcement
* Storage management
* Application lifecycle handling

### Applications

Applications are distributed as self-contained packages.

Example structure:

```text
calculator/
├── manifest.json
├── index.html
├── app.js
└── icon.png
```

## Long-Term Goals

### Portable Workspace

A user should be able to carry a complete computing environment on removable media and boot it on compatible hardware.

### Self-Hosted Infrastructure

Future versions of P8OS will integrate with a companion self-hosted platform that provides:

* Authentication
* Synchronization
* File storage
* Backups
* Application distribution
* Personal cloud services

### Security

Planned security features include:

* Encrypted user data
* Application signing
* Runtime permissions
* Secure update mechanisms
* Automatic session locking

## Current Status

P8OS is currently in active development and should be considered highly experimental.

The project is focused on establishing:

1. Build system infrastructure
2. Runtime architecture
3. Shell implementation
4. Application platform
5. Bootable image generation

## License

License to be determined.
