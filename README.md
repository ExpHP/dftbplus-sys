# `dtfbplus-sys`

Generates Rust bindings for the C interface of [DFTB+](http://www.dftbplus.org/),
a package implementing the Density Functional-based Tight Binding method.

## Usage

`dftbplus-sys` is **not yet published on crates.io**. Use a git dependency:

<!-- Please remember to update ALL TOML examples, not just this one! -->
```toml
[dependencies]
dftbplus-sys = { git = "https://github.com/ExpHP/dftbplus-sys", tag = "v0.0.1" }
```

`dftbplus-sys` currently always links to a system installation of `libdftb+`.
**Because `DFTB+` includes no installation mechanism, you will need to "install" it yourself.**

Please see [Installing DFTB+ for `dftbplus-sys`](doc/installing-dftbplus.md).

## Docs

See [`dftbplus.h`] in the DFTB+ source tree.
This is the file that bindings will be generated to.

If you just want to see the rust signatures for the bindings, you can also generate those yourself:

```
git clone https://github.com/ExpHP/dftbplus-sys
cd dftbplus-sys
cargo doc --open
```

## Configuration

There are currently no environment vars or cargo features for configuring `dftbplus-sys`.

## Does it work?

For an easier time diagnosing building/linking issues, you can clone this repo and try running the `link-test` example. It should create an empty file named `link-test.out`.

```sh
$ git clone https://github.com/ExpHP/dftbplus-sys
$ cd dftbplus-sys
$ cargo run --example=link-test || echo "failure!"
$
$ ls -l link-test.out
.rw-r--r-- 0 lampam 30 Apr 10:34 link-test.out
```

Be sure to try this using the environment variables and `--features` that you plan to enable in your own project.

## License

The Rust crate `dftbplus-sys` is licensed under either the MIT license or the Apache License (Version 2.0), at your option.

DFTB+ is free software licensed under the GNU Lesser GPL v3.0.
Its source code is available at https://github.com/dftbplus/dftbplus.

Please notice that the license of DFTB+ contains terms that apply to any
code that uses `dtfbplus-sys` to link to DFTB+.
Please see the file [`LICENSE-LGPL`](LICENSE-LGPL) for more details.

## Release notes

See [Release notes](relnotes.md).

## Citations

B. Aradi, B. Hourahine, and Th. Frauenheim. DFTB+, a sparse matrix-based
implementation of the DFTB method, J. Phys. Chem. A, 111 5678 (2007).

[`dftbplus.h`]: https://github.com/dftbplus/dftbplus/blob/master/api/mm/dftbplus.h
