# Installing DFTB+ for `dftbplus-sys`

Currently, `dftbplus-sys` only supports a system installation of `libdftbplus`,
which it expects to locate through pkg-config. In other words, it expects one of the following commands:

```
# if dftb+ was built shared
pkg-config --cflags --libs libdftb+

# if dftb+ was built static
pkg-config --cflags --libs --static libdftb+
```

to print all of `-I`, `-L`, `-l`, and `-D` flags necessary to build against the library and preprocess its header file. To ensure everything goes smoothly, there are a couple of things you need to keep in mind (and also environment variables you may need to set to describe your setup to `dftbplus-sys`).

**This guide was written for v20.2.1 of dftbplus.**

## Building DFTB+

In these examples I will be using `$HOME/.local` as the install prefix.

If you're following along, make sure that `$PREFIX/lib/pkgconfig` is in your **`PKG_CONFIG_PATH`**
and that the variable is exported to the environment.

e.g. **`.bashrc`**:

```bash
PREFIX=$HOME/.local

export PKG_CONFIG_PATH=$PREFIX/lib/pkgconfig:$PKG_CONFIG_PATH
```

### Get the DFTB+ Source

You can get the DFTB+ source from the [DFTB+ website](http://www.dftbplus.org/) or from their github.  As a simple matter of preference I will show the latter here.

```bash
git clone https://github.com/dftbplus/dftbplus
cd dftbplus
git pull --tags
git checkout 20.2.1
```

### Building with cmake

DFTB+ has a cmake build system now! Huzzah! This saves you from most of the work you used to have to do...

```bash
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=$PREFIX  \
    -DCMAKE_BUILD_TYPE=Release \
    -DPKGCONFIG_LANGUAGE=C \
    -DWITH_API=TRUE \
    -DBUILD_SHARED_LIBS=ON \
    -DLAPACK_LIBRARY=/usr/lib/liblapack.so
make -j4
make install
```

Some things to note here:

* `WITH_API=TRUE`:  In the current version this is no longer necessary as it defaults to TRUE, but I've included it here just for the sake of discussion.  This setting tells it to build the library API, which is the thing we ultimately want to link to from rust.
* `-DPKGCONFIG_LANGUAGE=C`:  **Very important!**  Without this, the pkgconfig file will have things for linking Fortran instead of C.  `dftbplus-sys` specifically needs the C API!
* `-DLAPACK_LIBRARY=/usr/lib/liblapack.so`:  I found it necessary to include an explicit path to lapack or else the executable wouldn't link properly (even though the `cmake` output claimed to detect this library). YMMV. This accepts other syntaxes (like an entire group of compile args); please see [dftbplus's own instructions](https://github.com/dftbplus/dftbplus/blob/master/INSTALL.rst).
* `-DBUILD_SHARED_LIBS=ON`:  You can choose to build a shared library or a static library.  **If you build a shared library as shown here, you will also need to set `RUST_DFTBPLUS_LINK_TYPE` when using `cargo`!** (described below)
* **MPI is currently unsupported by `dftbplus-sys`**. Try it if you like, but you can almost certainly expect to run into trouble. [Let me know how it works out for you](https://github.com/ExpHP/dftbplus-sys/issues/new)!

### Testing the build

Don't forget to run `utils/get_opt_externals` if you want to test your build!

```bash
cd ..  # leave the  build/ directory
utils/get_opt_externals
cd build
make test  # run the test suite
```

### A note about shared libraries

If you build DFTB+ as a shared library, you will need to let `dftbplus-sys` know by setting an environment variable:

```bash
# Do this before using cargo
export RUST_DFTBPLUS_LINK_TYPE=shared
```

The valid values of this environment variable are `shared` and `static`, and it defaults to `static`.

## Migrating from older versions of `dftbplus-sys`

v0.0.1 of `dftbplus-sys` used to query pkgconfig for `libdftb+` instead of `dftbplus` (it was written back before dftbplus had a cmake build system to generate this file).  You may be able to rename your existing `.pc` file.

v0.0.1 of `dftbplus-sys` expected a static library, because DFTB+ couldn't be built as shared at that time.  The default setting of `RUST_DFTBPLUS_LINK_TYPE` is `static`, so you should not have to adjust this to use your old DFTB+ installation.

