# simpleflake-rs

Travis Status: [![Build Status](https://secure.travis-ci.org/michaelcontento/simpleflake-rs.png?branch=master)](http://travis-ci.org/michaelcontento/simpleflake-rs)


Distributed ID generation in rust for the lazy. Based on the awesome [python implementation][simpleflake-py] from [SawdustSoftware][].

You can read an overview of what this does and why it came into being at the [Sawdust Software Blog][desc].

# Installation

Just add this crate as a dependency to your `Cargo.toml`:

```
[dependencies.simpleflake]
git = "https://github.com/michaelcontento/simpleflake-rs.git"
```

# Usage

```rust
extern crate simpleflake;

let new_id = simpleflake::new();
println!("generated id: {}", new_id);

let parts = simpleflake::parse(new_id);
println!("timestamp: {}", parts.timestamp);
println!("random bits: {}", parts.random_bits);
```

[desc]: http://engineering.custommade.com/simpleflake-distributed-id-generation-for-the-lazy/
[simpleflake-py]: https://github.com/SawdustSoftware/simpleflake
[SawdustSoftware]: http://sawdustsoftware.com/

