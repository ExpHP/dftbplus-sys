# `dftbplus-sys` release notes
## v0.0.2 (2020-12-22)
- Update to support versions of DFTB+ that come with a cmake build system.  This means pkg-config is now queried for the name `dftbplus` instead of the name `libdftb+`.
- Added enviroment variable `RUST_DFTBPLUS_LINK_TYPE=shared` to link to a shared library.
## v0.0.1 (2018-04-30)
- Initial release.
- Supports static linking to a system library only. Some assembly required!
