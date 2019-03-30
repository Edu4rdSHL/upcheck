# upcheck

Checker for ArchLinux available updates and send it via notify-rust.

# How to install and get it working?

First install [pacman-contrib](https://www.archlinux.org/packages/community/x86_64/pacman-contrib/) from ArchLinux repos:

```
# pacman -Syu --needed pacman-contrib
```
**Withouth systemd service:**

Put the bin/upcheck binary in $HOME/.local/bin/ or /usr/bin and then execute upcheck with these parameters:
``````
upcheck <N time in minutes> # Check for updates every N minutes.
``````
**Using AUR package:**

Install the [upcheck](https://aur.archlinux.org/packages/upcheck/) package.

**Manually for user only:**

Installation is easy, put the `bin/upcheck` binary in $HOME/.local/bin/, put the `upcheck.service` into $HOME/.config/systemd/user/. **Edit** the `upcheck.service` file in the *ExecStart=* section pointing to the `upcheck` executable path and finally enable/start `upcheck.service` with the command `systemctl --user enable upcheck.service && systemctl --user start upcheck.service`

**Important note**: you need a [notifications server](https://wiki.archlinux.org/index.php/Desktop_notifications#Notification_servers) if you aren't using Cinnamon, Deepin, Enlightenment, GNOME, GNOME Flashback or KDE Plasma.

# Building yourself

If you don't want to use the provided binary, you can compile it following the next commands:
```
# pacman -S rust
$ git clone https://gitlab.com/edu4rdshl/upcheck.git
$ cd upcheck
$ cargo build --release
$ cp target/release/upcheck $HOME/.local/bin/
```
Then continue with "How to install and get it working" skipping the binary section.

# How it works?

It use the `checkupdates` bash script provided by the `pacman-contrib` package. First, it exec the command `checkupdates` and save the output in a variable, if the variable is not an empty string, mean that there are available updates and then send the variable content using the  [notify-rust](https://crates.io/crates/notify-rust) crate.

# What is the purpose?

It doesn't have any special purpose, it was created only trying to avoid the user manual interaction of checking available updates everytime and show it when are available.

# How can I change the execution frequency time?

Modify the `upcheck.service` unit and change the ExecStart= option to the time that what you want in minutes before the executable.

Example:
```u
```ExecStart=/usr/bin/upcheck 90 # Execute upcheck every 90 minutes.```


That is all.
