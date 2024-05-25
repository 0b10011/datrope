# Datrope

Strict types for Discord's not-so-strict API and Gateway.
`serde` is used for (de)serialization.

## How to use

[Get set up with Rust](https://www.rust-lang.org/learn/get-started) and add this library as a dependency to your project:

```bash
cargo add datrope
```

### Deserializing

For any JSON you receive from the API or Gateway (use whichever type applies for the endpoint you're receiving data from):

```rust
let payload: EventPayload = serde_json::from_str(&message)?;
```

### Serializing

When sending data to the API or Gateway, build the data and convert to JSON:

```rust
let message = EventPayload::Heartbeat(None);
let json = serde_json::to_string(&message);
```
