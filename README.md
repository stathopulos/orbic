# Orbic RC400L Tools
A collection of utilities I found useful for poking around the Orbic RC400L.

Built using info collected from the [rayhunter project](https://github.com/EFForg/rayhunter) and [this xda forums thread](https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash.4334899/), along with a couple blogposts from Matthew Garrett<sup>[[1]](https://mjg59.dreamwidth.org/61725.html)[[2]](https://mjg59.dreamwidth.org/62419.html)</sup>

Use these utilities **at your own risk!** Everything in this repo is probably a bad idea!\
It's generally not a good idea to send random USB control messages down the wire using a random program you found on the internet!\
It's definitely not a good idea to build/flash/run random setuid binaries you found on the internet, especially on your potentially sensitive network hardware!

## Quick instructions

Quick instructions to build and push a 1kshell binary to your device and set the necessary permissions. More detailed instructions can be found in the following sections.

This guide assumes you have [adb](https://developer.android.com/tools/adb) installed. You can check if it's installed with `adb --version`. If it is, power on your device and connect it to USB.

1. Enable adb. Check if your device is connectable via adb using `adb devices`. If you haven't already enabled adb mode your device likely won't show up. To enable adb, run `cargo enable-adb`
2. Build 1kshell with `cargo build-rootshell`
3. Push the 1kshell binary to your device with `adb push target/armv7-unknown-linux-musleabihf/firmware/1kshell /tmp/`
4. Copy the binary to `/bin/` and configure the permissions by running `cargo at-cmds`. You should now have a 1kshell binary in `/bin/` with the correct permissions, alongside the one already in `/tmp/`
5. Get a shell on your device with `adb shell` and run `1kshell`. You should now have shell as uid/gid 1000!
6. Delete both the binary in `/bin/` and `/tmp/` once you're done by following the [uninstall instructions](#uninstall)

```sh
cargo enable-adb
cargo build-rootshell
adb push target/armv7-unknown-linux-musleabihf/firmware/1kshell /tmp/
cargo at-cmds
adb shell
```

## Enabling ADB
Plug in your device to USB and run `cargo enable-adb` to build and run `enable_adb`. This will send the command to your device to enable adb. Your device will reboot and adb should be enabled. This will also disable RNDIS, regardless of your USB Tethering setting.

This is also just equivalent to running `cargo run --bin enable_adb`

## Shell as UID/GID 1000
I ran in to permissions issues on my device, even after obtaining rootshell (id/gid 0), when trying to perfrorm actions on files located in directories owned by uid 1000 (for example the `/etc/init.d/` and `/bin/` folders). For reference this was on firmware version `ORB400L_V1.3.0_BVZRT`

This is probably a terrible solution, but it worked for quickly deleting a few files from these protected folders. You should almost certainly delete this binary once you're done.

1. Build the binary by running `cargo build-rootshell`\
   This is just an alias for `cargo build --bin 1kshell --target armv7-unknown-linux-musleabihf --profile firmware`
2. Push the binary to the device with `adb push target/armv7-unknown-linux-musleabihf/firmware/1kshell /tmp/`\
   You will need adb to be enabled, so if this fails and your device does not show up when you run `adb devices` you will need to follow the instructions in [Enabling ADB](#enabling-adb)
3. If you try to run `1kshell` now it will panic because you don't have the necessary permissions. If you already have rayhunter's rootshell installed you can run that first to get the necessary permissions, otherwise you'll have to send AT commands over serial: either by using the [AT Commands](#at-commands) binary provided in this repo, by following the [instructions on the xda forums](https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash.4334899/post-87855183) or by using the `./installer util serial "<desired command>"` provided by rayhunter.
4. You should now be able to run `1kshell`, congratulations, you now have shell as uid/gid 1000!
5. You should probably delete this binary as soon as you're done with it. The security implications of keeping around a setuid binary like this are not great, especially if you want to use this device for its intended purpose as an actual router.

### Uninstall
After you're done using 1kshell you should delete it. You'll need to use an instance of 1kshell to delete the setuid binary in `/bin/` and a normal user shell to delete the version in `/tmp/` because they'll be owned by different users.

It will look something like this:
```
/ $ 1kshell 
bash-4.3$ rm -f /bin/1kshell 
bash-4.3$ exit
/ $ rm -f /tmp/1kshell
```

## AT Commands

"AT Commands" let us execute some shell commands we otherwise wouldn't have permission to run by sending messages over a USB connection. We'll use this to copy our 1kshell binary to `/bin/` and set the into setuid and executable flags so we can run it to inherit the `1000` uid/gid.

After building the 1kshell binary and pushing it to the device's `/tmp/` directory with `adb`, you can run `cargo at-cmds` to copy 1kshell to `/bin/` and set the `4755` permission flags. This is just an alias for `cargo run --bin at_commands`. You should now be able to run `1kshell` and inherit uid/gid 1000.



### Util serial commands
For convenience, these are the AT commands we need to send to create our 1kshell setuid binary. These are the commands sent when you run `cargo at-cmds`
```
AT+SYSCMD=cp /tmp/1kshell /bin/
AT+SYSCMD=chown root /bin/1kshell
AT+SYSCMD=chmod 4755 /bin/1kshell
```

