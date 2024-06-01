# Datrope

Strict types for Discord's not-so-strict API and Gateway.
`serde` is used for (de)serialization.

## How to use

[Get set up with Rust](https://www.rust-lang.org/learn/get-started) and add this library as a dependency to your project:

```bash
cargo add datrope
```

### Features

The library size and compile time can be reduced by turning off the `default` feature and enabling just what you need.

Example:

```toml
# Cargo.toml

[dependencies]
datrope = { version = "*", default-features = false, features = ["all_objects", "serde"] }

```

#### `default`

_Enables: `api`, `gateway`, `clone`, `debug`, and `serde`_

The [default feature](https://doc.rust-lang.org/cargo/reference/features.html#the-default-feature) set aimed at Just Working™ for the majority of users, at the cost of slower compile times.

#### `api`

_Enables: `api_objects` and `serde`_

The API client to make HTTP requests to various endpoints.

#### `api_objects`

Objects returned from and sent to the Discord API. This feature is intended for folks wanting to implement their own API client. If `serde` is enabled, all objects will implement `Serialize` and `Deserialize`.

#### `gateway`

_Enables: `gateway_objects`, `serde`, and `api`_

The Gateway client to handle events sent to and received from the [Discord Gateway](https://discord.com/developers/docs/topics/gateway).

#### `gateway_objects`

_Enables: `api_objects`_

Objects returned from and sent to the Discord Gateway. This feature is intended for folks wanting to implement their own Gateway client. If `serde` is enabled, all objects will implement `Serialize` and `Deserialize`.

#### `all_objects`

_Enables: `api_objects` and `gateway_objects`_

Objects returned from and sent to the Discord API and Gateway. This feature is intended for folks wanting to implement their own API and Gateway clients.

#### `undocumented-fields`

Enables fields returned by the Discord API or Gateway that are not documented by Discord. These may change at any time.

#### `clone`

Will derive `Clone` for all objects.

#### `debug`

Will derive `Debug` for all objects.

#### `serde`

_Enables: `dep:serde`, `dep:serde_json`, `dep:serde_repr`, `enumset/serde`, `time/serde`, `time/formatting`, `time/parsing`, and `url/serde`_

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
