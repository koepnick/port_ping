A ping command for ports

# Why

Knowing when a server is back up via ICMP (L2) is great. 

Knowing when specific listening services are back up via TCP/IP (L5) is great too!

This is a really simple tool that I use in my day-to-day sysadmin duties. Specifically, when I need to know when a system is back up and listening on port 22.

Note: This does not *validate* that a proper service is up, only that a port is open and accepting connections.
