# read-cell

[![crates](https://img.shields.io/crates/v/read-cell.svg?style=for-the-badge&label=read-cell)](https://crates.io/crates/read-cell)
[![docs](https://img.shields.io/badge/docs.rs-read--cell-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white)](https://docs.rs/read-cell)
[![actions](https://img.shields.io/github/workflow/status/zakarumych/read-cell/badge/master?style=for-the-badge)](https://github.com/zakarumych/read-cell/actions?query=workflow%3ARust)
[![MIT/Apache](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](COPYING)
![loc](https://img.shields.io/tokei/lines/github/zakarumych/read-cell?style=for-the-badge)


Provides read-only counterpart to standard [`Cell`] type.
Unlike [`Cell`], [`ReadCell`] cannot be used to mutate inner value, just like [`&T`],
but similar to [`Cell`] it cannot be used to get [`&T`] to the inner value.

While [`&Cell<T>`] references and [`&T`] references to the same value cannot coexist,
[`&ReadCell<T>`] reference and [`&Cell<T>`] reference to the same value can coexist.
As well as [`&ReadCell<T>`] reference and [`&T`] reference to the same value can coexist.


[`Cell`]: https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html
[`&Cell<T>`]: https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html
[`&ReadCell<T>`]: https://docs.rs/read-cell/latest/read_cell/struct.ReadCell.html
[`&T`]: https://doc.rust-lang.org/nightly/core/primitive.reference.html

## License

Licensed under either of

* Apache License, Version 2.0, ([license/APACHE](license/APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([license/MIT](license/MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
