# Orbic RC400L Tools
A collection of utilities I found useful for poking around the Orbic RC400L.

Built using info collected from the [rayhunter project](https://github.com/EFForg/rayhunter) and [this xda forums thread](https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash.4334899/), along with a couple blogposts from Matthew Garrett<sup>[[1]](https://mjg59.dreamwidth.org/61725.html)[[2]](https://mjg59.dreamwidth.org/62419.html)</sup>

Use these utilities **at your own risk!** Everything in this repo is probably a bad idea!\
It's generally not a good idea to send random USB control messages down the wire using a random program you found on the internet!\
It's definitely not a good idea to build/flash/run random setuid binaries you found on the internet, especially on your potentially sensitive network hardware!

## Enabling ADB
Plug in your device to USB and run `cargo enable-adb` to build and run `enable_adb`. This will send the command to your device to enable adb. Your device will reboot and adb should be enabled. This will also disable RNDIS, regardless of your USB Tethering setting.

This is also just equivalent to running `cargo run --bin enable_adb`

## Shell as UID/GID 1000
I ran in to permissions issues on my device, even after obtaining rootshell (id/gid 0), when trying to perfrorm actions on files located in directories owned by uid 1000 (for example the `/etc/init.d/` and `/bin/` folders). For reference this was on firmware version `ORB400L_V1.3.0_BVZRT`

This is probably a terrible solution, but it worked for quickly deleting a few files from these protected folders. You should almost certainly delete this binary once you're done.

1. Build the binary by running `cargo build-rootshell`\
   This is just an alias for `cargo build --bin 1kshell --target armv7-unknown-linux-musleabihf --profile firmware`
2. Push the binary to the device with `adb push target/armv7-unknown-linux-musleabihf/firmware/1kshell /tmp/`
3. If you try to run `1kshell` now it will panic because you don't have the necessary permissions. If you already have rayhunter's rootshell installed you can run that first to get the necessary permissions, otherwise you'll have to send AT commands over serial, either by following the [instructions on the xda forums](https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash.4334899/post-87855183) or by using `./installer util serial "<desired command>"` provided by rayhunter.
4. You should now be able to run `1kshell`, congratulations, you now have shell as uid/gid 1000!
5. You should probably delete this binary as soon as you're done with it. The security implications of keeping around a setuid binary like this are not great, especially if you want to use this device for its intended purpose as an actual router.

### Util serial commands
For convenience, these are the AT commands we need to send to create our 1kshell setuid binary
```
AT+SYSCMD=cp /tmp/1kshell /bin/
AT+SYSCMD=chown root /bin/1kshell
AT+SYSCMD=chmod 4755 /bin/1kshell
```

### Uninstall
After you're done using `1kshell` you should delete it. You'll need to use an instance of `1kshell` to delete the setuid binary in `/bin/` and a normal user shell to delete the version in `/tmp/` because they'll be owned by different users.

It will look something like this:
```
/ $ 1kshell 
bash-4.3$ rm -f /bin/1kshell 
bash-4.3$ exit
/ $ rm -f /tmp/1kshell
```