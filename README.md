A ping command for ports

# Why

Knowing when a server is back up via ICMP (L2) is great. 

Knowing when specific listening services are back up via TCP/IP (L5) is great too!

This is a really simple tool that I use in my day-to-day sysadmin duties. Specifically, when I need to know when a system is back up and listening on port 22.

Note: This does not *validate* that a proper service is up, only that a port is open and accepting connections.

I have zero intention of doing any work to get this to work on Windows. 

# Install

## Simple

`cargo install --git https://github.com/koepnick/port_ping`

## Simplerer

`wget ...`

## From Source

```shell
git clone https://github.com/koepnick/port_ping
cd port_ping
make release
```

## TODO: 

- [ ] Add sanity check for name resolution
- [ ] Support for Apple M1/M2 devices
- [ ] Support for Termux on Android
- [ ] Add a `manpage`
