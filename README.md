# Simple Rust Typing Test
## Multiplayer Branch

I envision this working with the same binary. The server should be able to started by specifying --server and an address to bind to using --bind and an optional port number --port.

When a client wants to play multiplayer they should be able to run `typing_test --mp` and then optionally specify `--server` and `--port`. These values should default to harrisowe.me and 1948. 

### Features
 - Watch live progress
 - Server chooses word string and sends information to both clients
 - Countdown to game start once all connected players are ready (must have way for players to ready/unready)
 - Communication done preferrable over TCP connection
 - Server times users as well to prevent cheating.
 - Server decides winner

