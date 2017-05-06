# tjson-rust [![Crate Version][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Build Status][build-image]][build-link] ![MIT/Apache 2 licensed][license-image]

A Rust implementation of [Tagged JSON (TJSON)][TJSON] based on [serde].

[TJSON] is a microformat which supplements JSON with an extended set of
data types by supplying a type "tag" embedded in object member names:

```json
{
  "array-example:A<O>": [
    {
      "string-example:s": "foobar",
      "binary-example:d": "QklOQVJZ",
      "float-example:f": 0.42,
      "int-example:i": "42",
      "timestamp-example:t": "2016-11-06T22:27:34Z",
      "boolean-example:b": true
    }
  ],
  "set-example:S<i>": [1, 2, 3]
}
```

[crate-image]: https://img.shields.io/crates/v/tjson.svg
[crate-link]: https://crates.io/crates/tjson
[docs-image]: https://docs.rs/tjson/badge.svg
[docs-link]: https://docs.rs/tjson/
[build-image]: https://travis-ci.org/tjson/tjson-rust.svg?branch=master
[build-link]: https://travis-ci.org/tjson/tjson-rust
[license-image]: https://img.shields.io/badge/license-MIT%2FApache2-blue.svg
[TJSON]: https://www.tjson.org/
[serde]: https://github.com/serde-rs/serde/

## Documentation

Documentation can be found on docs.rs:

[Documentation](https://docs.rs/tjson/)

## Help and Discussion

Have questions? Want to suggest a feature or change?

* [TJSON Gitter]: web-based chat
* [TJSON Google Group]: join via web or email ([tjson+subscribe@googlegroups.com])

[TJSON Gitter]: https://gitter.im/tjson/Lobby
[TJSON Google Group]: https://groups.google.com/forum/#!forum/tjson
[tjson+subscribe@googlegroups.com]: mailto:tjson+subscribe@googlegroups.com

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/tjson/tjson-rust

## License

Copyright (c) 2017 Tony Arcieri. Incorporations portions from the [Serde JSON]
project: Copyright 2017 Serde Developers. Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT] or https://opensource.org/licenses/MIT)

at your option.

[Serde JSON]: https://github.com/serde-rs/json
[LICENSE-APACHE]: https://github.com/tjson/tjson-rust/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/tjson/tjson-rust/blob/master/LICENSE-MIT
