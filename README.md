[![Build Status](https://travis-ci.org/dpc/hex2d-rs.svg?branch=master)](https://travis-ci.org/dpc/rclist-rs)

# rclist-rs

## Introduction

`RcList` is read-only, append-only list (log), that can share common tail (history) with other `RcList`.

Example:

    HEAD-Y-\
            1 -> 2 -> 3 -weak-> 4 -> 5
    HEAD-X------/                   /
    HEAD-Z--------------------------

Lists Y, X, Z are sharing history. Link between 3 and 4 is weak, which means
after HEAD-Z is destroyed nodes 4 and 5 will be deallocated.

Read [Documentation](//dpc.github.io/rclist-rs/) for details.

See [issues](//github.com/dpc/rclist-rs/issues/) for TODO and BUGs.

## Building

    cargo build

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
