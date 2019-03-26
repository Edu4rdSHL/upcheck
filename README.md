# upcheck

Check ArchLinux available updates and send it via libnotify.

# How to install and get it working?

First install [pacman-contrib](https://www.archlinux.org/packages/community/x86_64/pacman-contrib/) and [libnotify](https://www.archlinux.org/packages/extra/x86_64/libnotify/) from ArchLinux repos:

```
# pacman -Syu --needed pacman-contrib libnotify
```

Installation is easy, put the `upcheck` script in $HOME/.local/bin/, then put the `upcheck.service` and `upcheck.timer` into $HOME/.config/systemd/user/ and enable/start `upcheck.timer` with the command `systemctl --user enable upcheck.timer && systemctl --user start upcheck.timer`

**Important note**: you need a [notifications server](https://wiki.archlinux.org/index.php/Desktop_notifications#Notification_servers) if you aren't using Cinnamon, Deepin, Enlightenment, GNOME, GNOME Flashback or KDE Plasma.

# How it works?

It use the `checkupdates` bash script provided by the `pacman-contrib` package and the `notify-send` binary provided by the `libnotify` package. First, it exec the command `checkupdates` and save the output in a variable, if the variable is not an empty string, mean that there are available updates and then send the variable content using `libnotify`.

# What is the purpose?

It doesn't have any special purpose, it was created only trying to avoid the user manual interaction of checking available updates everytime and show it when are available.

# How can I checge the execution frequency time?

Modify the `upcheck.timer` unit and change the OnUnitActiveSec= option to what you want. See `man 7 systemd.time` for the time specifications.

That is all.