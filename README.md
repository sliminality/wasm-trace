# slack-rs

A library for reading and writing Slack messages, with an accompanying CLI (and, as a stretch goal, a lightweight ncurses-style TUI).

The first `slack` command will start a background server that authenticates with the Slack [Real Time Messaging API](https://api.slack.com/rtm) and opens a WebSocket connection using the [ws-rs](https://github.com/housleyjk/ws-rs) crate.

Subsequent calls to `slack read <channel> [-n #]` will pull the last `#` messages from `<channel>` from the server, similar to `tail`:

```sh
> slack read krishna-lunch -3
Sarah Lim [Tuesday, May 8 7:14 AM] in #krishna-lunch 
They changed it up, paneer is today :0
Sasha Weiss [Tuesday, May 18 8:19 AM] in #general 
I got today!
Armaan Shah [Friday, May 11 10:40 AM] in #general 
I overslept is anyone getting Krishna lunch today?
```

Sending a new message via `slack send <channel> <message>` will send `<message>` to `<channel>` via the server:

```sh
> slack send general "KL anyone?"
Sarah Lim [10:30 AM] in #krishna-lunch
KL anyone?
```

Sending a new DM:

```sh
> slack dm meggyg "we should meet to work on rust"
Sarah Lim [10:30 AM] in @meggyg
we should meet to work on rust
```

## Why we think it’s interesting and hard

1. **Asynchronous/event-driven I/O:** We'd like to explore different abstractions for async computation, such as futures and event emitters. We are separately familiar with event-driven programming and multithreading, but haven't worked with both together. We'd also like to understand the challenges of implementing a pub/sub pattern using multiple threads in a statically-typed language.
2. **State synchronization:** Ensuring our local server keeps up-to-date with message state and handles network failure.
3. **Resource management:** The Slack desktop client is frequently (and rightfully) slammed as an Electron disaster with a memory footprint linear in the number of workspaces. We'd like to understand why it is hard to engineer a good client, and how running a background server might affect this.

## Concrete functional requirements we intend to meet

### Must-have

- Connecting to one workspace, authenticating, and storing OAuth tokens on disk
- Reading the most recent `n` messages from a channel or DM in real-time (using background server)
- Sending a new message to a channel or a DM
	- Notify user when message successfully sends
	- Retry if network connection is poor

### Nice to have

-   Unicode 10 emoji reactions
-   Aesthetically pleasing client
-   Keyboard shortcuts for channel navigation and posting

## Library examples

### Listening for messages

```rust
let channel = slack::Channel::new('krishna-lunch').unwrap();
channel.on(slack::ChannelEvent::NewMessages, |messages| {
	println!('{:?}', messages);
});
```

### Sending messages

```rust
channel.send('krishna lunch anyone?');
```
