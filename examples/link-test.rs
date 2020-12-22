// link-test - The smallest possible crate using dftbplus-sys
//             that has an observable side-effect.

// Usage:
//
//     cargo run --example=link-test  [other cargo arguments...]
//
// If successful, the example will print nothing to stdout,
// create an empty file `link-test.out` in the current working directory,
// and exit successfully.
//
// The vast majority of possible problems will manifest during the linking
// of the final binary, before it is run.  I can't tell you what you'll see,
// exactly, but most likely cargo will report failure running a "cc" command,
// and exit with a nonzero status.

use std::os::raw::c_char;
use dftbplus_sys as c;

const OUT_PATH: &[u8] = b"link-test.out\0";
fn main() {
    unsafe {
        let mut dft = ::std::mem::MaybeUninit::<c::DftbPlus>::uninit();

        // NOTE: The output filepath can be a null pointer to suppress output,
        //       but here we want an observable side-effect.
        c::dftbp_init(dft.as_mut_ptr(), OUT_PATH.as_ptr() as *const c_char);
        c::dftbp_final(dft.as_mut_ptr());
    }
}
