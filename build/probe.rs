/* *************************************************** **
** This file is licensed under EITHER the MIT license  **
** or the Apache 2.0 license, at your option.          **
**                                                     **
**     http://www.apache.org/licenses/LICENSE-2.0      **
**     http://opensource.org/licenses/MIT              **
**                                                     **
** *************************************************** */

use crate::{BuildMeta, CcFlag, CcFlags};
use std::fmt;

pub(crate) fn probe_and_link() -> Result<BuildMeta, ProbeError> {
    probe_and_link_via_pkgconfig()
}

pub(crate) enum ProbeError {
    PkgConfig(::pkg_config::Error),
}

impl From<::pkg_config::Error> for ProbeError {
    fn from(e: ::pkg_config::Error) -> Self {
        ProbeError::PkgConfig(e)
    }
}

impl fmt::Display for ProbeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProbeError::PkgConfig(e) => fmt::Display::fmt(e, f),
        }
    }
}

fn probe_and_link_via_pkgconfig() -> Result<BuildMeta, ProbeError> {
    let library = {
        ::pkg_config::Config::new()
            .statik(true)
            .probe("libdftb+")?
    };
    let include_dirs = CcFlags({
        library.include_paths.into_iter()
            .map(Into::into).map(CcFlag::IncludeDir)
            .collect()
    });
    let defines = CcFlags({
        library.defines.into_iter()
            .map(|(key, value)| match value {
                Some(value) => CcFlag::Define(format!("{}={}", key, value)),
                None => CcFlag::Define(format!("{}", key)),
            })
            .collect()
    });

    Ok(BuildMeta {
        header: "dftbplus.h",
        include_dirs,
        defines,
    })
}
