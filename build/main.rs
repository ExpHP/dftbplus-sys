/* *************************************************** **
** This file is licensed under EITHER the MIT license  **
** or the Apache 2.0 license, at your option.          **
**                                                     **
**     http://www.apache.org/licenses/LICENSE-2.0      **
**     http://opensource.org/licenses/MIT              **
**                                                     **
** *************************************************** */

mod probe;

const LIB_NAME: &'static str = "dftb+";

use path_abs::{PathArc, PathDir};
use walkdir::WalkDir;

use std::path::Path;
use std::fmt::{self, Display};
use std::borrow::Borrow;

// ----------------------------------------------------

fn main() -> PanicResult<()> {
    _main_print_reruns()?;

    let meta = _main_link_library()?;

    _main_gen_bindings(meta)?;

    Ok(())
}

fn _main_link_library() -> PanicResult<BuildMeta> {
    Ok(probe::probe_and_link()?)
}

// ----------------------------------------------------

// Information discovered during the build that is needed during bindgen.
struct BuildMeta {
    // Path for an #include directive.
    header: &'static str,
    // A bunch of -I arguments
    include_dirs: CcFlags,
    // A bunch of -D arguments
    defines: CcFlags,
}

// ----------------------------------------------------

fn _main_gen_bindings(meta: BuildMeta) -> PanicResult<()> {
    let BuildMeta { header, include_dirs, defines } = meta;

    let out_path = PathDir::new(env::expect("OUT_DIR"))?;

    let _ = ::std::fs::create_dir(out_path.join("codegen"));

    let mut gen = ::bindgen::Builder::default();
    // use #include to let the C preprocessor resolve the true filepath
    gen = gen.header_contents(
        "include_lib.h",
        &format!(r##"#include <{}>"##, header),
    );

    gen = gen.clang_args(defines.to_args());
    gen = gen.clang_args(include_dirs.to_args());

    // support older versions of libclang, which will mangle even
    // the names of C functions unless we disable this.
    gen = gen.trust_clang_mangling(false);

    gen.generate()
        .unwrap_or_else(|_| panic!("Unable to generate bindings for '{}'!", LIB_NAME))
        .write_to_file(out_path.join("codegen/dftbplus.rs"))
        .unwrap_or_else(|_| panic!("Couldn't write bindings for '{}'!", LIB_NAME));

    Ok(())
}

// ----------------------------------------------------

fn _main_print_reruns() -> PanicResult<()> {
    rerun_if_changed("Cargo.toml");
    rerun_if_changed_recursive("src".as_ref())?;

    Ok(())
}

fn rerun_if_changed_recursive(root: &Path) -> PanicResult<()> {
    for entry in WalkDir::new(root) {
        let entry = entry?;
        rerun_if_changed(entry.path().display());
    }
    Ok(())
}

fn rerun_if_changed<T: Display>(path: T) { println!("cargo:rerun-if-changed={}", path); }
#[allow(unused)]
fn rerun_if_env_changed<T: Display>(var: T) { println!("cargo:rerun-if-env-changed={}", var); }

// ----------------------------------------------------

/// A result type that is always Ok because it panics otherwise.
///
/// Used whenever I'm too lazy to do any better.
pub type PanicResult<T> = Result<T, Never>;

#[derive(Debug, Clone)]
pub enum Never {}
impl<T: Display> From<T> for Never {
    fn from(e: T) -> Never { panic!("{}", e); }
}

// ----------------------------------------------------

mod env {
    use ::std::env;

    // For vars that cargo provides, like OUT_DIR.
    // This doesn't do "rerun-if-env-changed".
    pub fn expect(var: &str) -> String {
        env::var(var).unwrap_or_else(|e| panic!("error reading {}: {}", var, e))
    }

    // (there's currently no env vars designed specifically for this crate)
}

// ----------------------------------------------------

// A flag for the C compiler (or preprocessor or linker).
#[derive(PartialEq, Eq)]
pub enum CcFlag {
    // a "-DNAME" flag (or "-DNAME=VALUE", we don't care)
    Define(String),
    // an "-Ipath/to/include" flag (or "-I" "path/to/include").
    IncludeDir(PathArc),
    // an "-Lpath/to/include" flag (or "-L" "path/to/include").
    LibDir(PathArc),
    // an "-llibrary" flag
    Lib(String),
    // an unknown argument.  We will assume it is not something
    // that would prevent the next argument from being parsed as
    // an option, because there's no reliable way to tell.
    //
    // This will only cause trouble if an unrecognized option is given
    // an option argument beginning with -I/-l/-L/-D or similar, and
    // they are separated by a space.  This seems unlikely.
    Other(String),
}

pub struct CcFlags(Vec<CcFlag>);

impl CcFlag {
    fn fmt_with_space(&self, space: &str, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CcFlag::IncludeDir(ref path) => write!(f, "-I{}{}", space, path.display()),
            CcFlag::LibDir(ref path) => write!(f, "-L{}{}", space, path.display()),
            CcFlag::Lib(ref s) => write!(f, "-l{}{}", space, s),
            CcFlag::Define(ref s) => write!(f, "-D{}{}", space, s),
            CcFlag::Other(ref s) => write!(f, "{}", s),
        }
    }
}

// Displays as "-l iberty"
//
// This format is required for `cargo:rustc-flags`.
#[allow(unused)]
struct WithSpace<C>(C);
impl<C> fmt::Display for WithSpace<C> where C: Borrow<CcFlag> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    { self.0.borrow().fmt_with_space(" ", f) }
}

// Displays as "-liberty"
//
// This format is convenient for producing atomic arguments without fear
// of quoting issues.
struct WithoutSpace<C>(C);
impl<C> fmt::Display for WithoutSpace<C> where C: Borrow<CcFlag> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    { self.0.borrow().fmt_with_space("", f) }
}

impl CcFlags {
    fn to_args(&self) -> Vec<String> {
        self.0.iter().map(|x| WithoutSpace(x).to_string()).collect()
    }
}
