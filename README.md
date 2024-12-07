# minecraft-net
This work is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/?ref=chooser-v1)

**This package is a work in progress!**

A package implementing the 1.21.1 minecraft protocol.

## Features / Goals
- [ ] Structs for all Packets (~74/221 ~33%)
  - [X] Handshake (1 packet)
  - [X] Status (4 packets)
  - [X] Login (11 packets)
  - [X] Configuration (25 packets)
  - [ ] Play (180 packets) (33 somewhat done, 7 missing fields)
- [ ] improved usability
- [ ] checking validity
- [X] Encoding/Decoding packets
- [X] Helper for encrypted connections
- [ ] Documentation :(

## Note:
This project is not in any way affiliated with Mojang and/or Microsoft. 
Use at your own risk.

It is also still in development.

Based on [wiki.vg](https://wiki.vg) which has been shutdown is migrating to [minecraft.wiki](https://minecraft.wiki).