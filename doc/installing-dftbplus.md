# Installing DFTB+ for `dftbplus-sys`

_(**notice:** These instructions were written prior to the 19.1 release of DFTB+, which is the first to contain the library.  They are based off of the master branch of https://github.com/dftbplus/dftbplus at the time of writing.  Things may have changed since then!)_

## The bottom line

Currently, `dftbplus-sys` only supports a system installation of `libdftb+`,
which it expects to locate through pkg-config.
In other words, it expects the following command:

```
pkg-config --cflags --libs --static libdftb+
```

to print all of `-I`, `-L`, `-l`, and `-D` flags necessary to build against the library and preprocess its header file.

Notice from the above that `dftbplus-sys` assumes the library was built statically.
This is because, at the time of writing, DFTB+ itself does not appear to support being built as a shared library.

> **Q:** So that's just `make install_api`, right?

Nope.

At the time of writing, DFTB+'s `make install_api` has a number of problems:

* There's no way to discover static dependencies like `libgfortran`.  (Ideally we want a `pkg-config` file with a `Libs.private:` line).
* It doesn't install the C header file. (rather, it fills `include/` with over a hundred Fortran `.mod` files!)
* It doesn't install the external libraries that it builds (e.g. `xmlf90`), and I don't see any option to have it use preinstalled versions.

So basically you need to install it yourself.  Don't worry; this document is here to walk you through this step-by-step.

## Step-by-step Guide

### Choose your installation directory

In these examples I will be using `$HOME/.local` as the install prefix.
If you're following along, make sure that `$HOME/lib/pkgconfig` is in your **`PKG_CONFIG_PATH`**
and that the variable is exported to the environment.

e.g. **`.bashrc`**:

```bash
export PKG_CONFIG_PATH=/home/lampam/.local/lib/pkgconfig:$PKG_CONFIG_PATH
```

### Get the DFTB+ Source

Download the source for DFTB+ version 19.1 or later from the [DFTB+ website](http://www.dftbplus.org/).
**Versions prior to 19.1 do not include the C API!**

At the time of writing, version 19.1 has not yet been released, but an unreleased version of the API is available on the master branch at github.

```
$ git clone https://github.com/dftbplus/dftbplus
$ cd dftbplus
```

### Configure your build

Browse around `sys/` for a suitable makefile, and copy it to `./make.arch`. Edit it if necessary.

There are also some knobs you can turn in `make.config` to enable/disable dependencies.

On my system, I simply did the following:

```
$ cp sys/make.x86_64-linux-gnu make.arch
```

**MPI and OpenMP are currently unsupported by `dftbplus-sys`**. Try them if you like, but you can almost certainly expect to run into trouble. [Let me know how it works out for you](https://github.com/ExpHP/dftbplus-sys/issues/new)!

### Build and test the API

Download the Slater-Koster files used by some of the tests:

```
$ ./utils/get_opt_externals
```

Now build the API and run its test suite, **making sure to record the output from `make`.**

```
$ make test_api 2>&1 | tee make.log
```

The tests should succeed.

### Find the flags that were used to build the C API test

`make test_api` will have at some point generated a binary named `test_qdepextpotc`.  This tests the C API.  We want the flags from that build.

```
$ grep 'test_qdepextpotc ' make.log
gcc  -o test_qdepextpotc test_qdepextpotc.o testhelpers.o -L/home/lampam/asd/clo
ne/dftbplus/_build/api/mm -ldftb+ -L/home/lampam/asd/clone/dftbplus/_build/exter
nal/xmlf90 -lxmlf90 -llapack -lblas  -lgfortran -lm -lgomp
```

Reading this closely, we find that, in my case, there are two libraries that were built during the compilation of DFTB+:

* **`_build/api/mm/libdftb+.a`**
* **`_build/external/xmlf90/libxmlf90.a`**

...and the other 5 are system libraries:

```
-llapack -lblas -lgfortran -lm -lgomp
```

Also take note of any `-D` flags. (in this case, there weren't any)

### Install the files

Set up your install prefix if it doesn't exist.

```
PREFIX=/home/lampam/.local
mkdir -p $PREFIX/lib/pkgconfig
mkdir -p $PREFIX/include
```

Install the header file and any libraries that were built.

```
cp -a api/mm/dftbplus.h $PREFIX/include
cp -a _build/api/mm/libdftb+.a $PREFIX/lib
cp -a _build/external/xmlf90/libxmlf90.a $PREFIX/lib
```

Create the pkgconfig file at `$PREFIX/lib/pkgconfig/libdftb+.pc`.  (The name matters!)

* Put the correct version number.
* `Libs:` should always have `-L${libdir}`.
* `Libs:` should also contain `-l` flags for each library you just installed to `$PREFIX/lib`.
* `Libs.private:` should have `-l` flags for the other libraries.
* `Cflags:` should always have `-I${includedir}`.
* If you have any `-D` flags, put them in `Cflags:`.

**`/home/lampam/.local/lib/pkgconfig/libdftb+.pc`**
```
prefix=/home/lampam/.local
exec_prefix=${prefix}
libdir=${exec_prefix}/lib
includedir=${prefix}/include

Name: DFTB+
Description: DFTB+ C Binding
Version: 19.1
Requires.private:
Libs: -L${libdir} -ldftb+ -lxmlf90
Libs.private: -llapack -lblas  -lgfortran -lm -lgomp
Cflags: -I${includedir}
```

That's it! (...hopefully!)

### Test your install

Try linking that test manually now, using `pkg-config` to procure the flags.

Use `cc` as your linker, because this is what Rust uses.

```
$ cd _build/test/api/mm/testers/
$ cc -o test_qdepextpotc test_qdepextpotc.o testhelpers.o $(pkg-config --cflags --libs --static libdftb+)
```

If this succeeds, rejoice!  Your installion of DFTB+ should be compatible with `dftbplus-sys`.

If you want to make sure, clone this repository and run the `link-test` example. It should create an empty file named `link-test.out`:

```
$ git clone https://github.com/ExpHP/dftbplus-sys
$ cd dftbplus-sys
$ cargo run --example=link-test || echo "failure!"
$ ls -l link-test.out
.rw-r--r-- 0 lampam 30 Apr 10:34 link-test.out
```

