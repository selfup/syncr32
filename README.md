# syncr32

### Status

In Development! :tada:

Not ready for use! :joy:

### Purpose

To sync files to another storage medium for backup purposes.

Primarily built for Windows, because why not, but the API should work with *nix unless some weird discrepencies occur with Windows vs. *nix.

### Reasoning Behind the Project

To make a safe, and open source solution to sync directories.

### Educational

This project is primarily for Educational purposes :smile:

### Usage

**Windows**

Set an Environment Variable for the directory you want to backup

*BACKUP_ROOT* is the name of the ENV VAR

```bat
cargo run --release
```

_Currently this just prints all the dirs_

**macOs/Linux**

To list what is contained in the projects fixture directory for example:

```bash
BACKUP_ROOT=$PWD/src/fixtures cargo run --release
```

_Currently this just prints all the dirs_

### Binaries

You will have to build your own binaries so that the program can grab your ENV var.

Otherwise it will just contain my own BACKUP_DIR path and not be very useful :joy:

In the `windows_build.bat` or `unix_build.bat` change the path that `BACKUP_ROOT` points to!

Then run the respective script!

Use at your own risk :P
