# Chatroom Server
Simple **Async** Chatroom Server made with [Rust](https://www.rust-lang.org) and [Tokio](https://tokio.rs)

## How to use
Simply build and run the proyect and connect to PORT `9090` with a Telnet client.

```cmd
telnet localhost 9090
```

NOTE: The port number is arbitrary and can be changed in [config.rs](src/config.rs).

## TODO List:
- [x] Multiple messages
- [x] Multiple clients
- [ ] Fix Telnet bugginess
- [ ] Write client program