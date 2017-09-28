/*!
Find the `DW_AT_producer` for all compilation units within a shared library or
executable.

[![](https://docs.rs/dwprod/badge.svg)](https://docs.rs/dwprod/) [![](http://meritbadge.herokuapp.com/dwprod)](https://crates.io/crates/dwprod) [![](https://img.shields.io/crates/d/dwprod.png)](https://crates.io/crates/dwprod) [![Unix Build Status](https://travis-ci.org/fitzgen/dwprod.png?branch=master)](https://travis-ci.org/fitzgen/dwprod)

### What is `DW_AT_producer`?

The `DW_AT_producer` is an attribute within DWARF debug info that says what
compiler was used to create each compilation unit that ended up within a given
shared library or executable.

### Usage

#### As a Library

First, add this to your `Cargo.toml`:

```toml
[dependencies.dwprod]
version = "0.1.0"
# Do not build the command line `dwprod` executable.
default-features = false
```

Then, import the `dwprod` crate and use it to iterate over `DW_AT_producer`
values:

```rust,no_run
extern crate dwprod;

fn try_main() -> dwprod::Result<()> {
    let opts = dwprod::Options::new("path/to/some/executable");

    opts.producers(|producers| {
        while let Some(producer) = producers.next()? {
            println!("Found DW_AT_producer = {}", producer);
        }

        Ok(())
    })?
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Uh oh! {}", e);
        ::std::process::exit(1);
    }
}
```

The [`fallible-iterator`](https://crates.io/crates/fallible-iterator) crate can
also be used to leverage iterator combinators like `map` and `filter`:

```rust
extern crate dwprod;
extern crate fallible_iterator;

use fallible_iterator::FallibleIterator;
use std::path::Path;

fn each_rustc_producer<F>(
    shared_lib_or_exe: &Path,
    mut callback: F
) -> dwprod::Result<()>
where
    F: FnMut(String)
{
    let opts = dwprod::Options::new(shared_lib_or_exe);
    opts.producers(|producers| {
        producers
            // Filter down to only the producers with "rustc" in their name.
            .filter(|p| p.contains("rustc"))
            // Then map the given callback over each producer.
            .map(&mut callback)
            // Finally, use `count` to force iteration.
            .count()?;

        Ok(())
    })?
}
# fn main() {}
```

#### As a Command Line Tool

First, install via `cargo`:

```commands
$ cargo install dwprod
```

Then, run `dwprod path/to/shared/library/or/executable` to get a dump of all of
the `DW_AT_producer` values for each compilation unit within the given shared
library or executable.

Here is the result of running `dwprod` on itself:

```commands
$ dwprod $(which dwprod)
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
<truncated>
```

For more details about the `dwprod` command line tool, run `dwprod --help`.
 */

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

extern crate fallible_iterator;
extern crate gimli;
extern crate object;

use fallible_iterator::FallibleIterator;
use gimli::{CompilationUnitHeadersIter, DebugAbbrev, DebugInfo, DebugStr, EndianBuf, NativeEndian,
            Reader};
use std::error;
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::path;

/// Errors that `dwprod` can encounter.
#[derive(Debug)]
pub enum Error {
    /// Some other error.
    Msg(String),

    /// An IO error.
    Io(io::Error),

    /// A DWARF parsing error.
    Dwarf(gimli::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Msg(ref s) => write!(f, "{}", s),
            Error::Io(ref e) => write!(f, "{}", e),
            Error::Dwarf(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Msg(ref s) => s.as_str(),
            Error::Io(ref e) => e.description(),
            Error::Dwarf(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Msg(_) => None,
            Error::Io(ref e) => Some(e),
            Error::Dwarf(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<gimli::Error> for Error {
    fn from(e: gimli::Error) -> Error {
        Error::Dwarf(e)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::Msg(s)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Error {
        s.to_string().into()
    }
}

/// Either an `Ok(T)` or an `Err(dwprod::Error)`.
pub type Result<T> = ::std::result::Result<T, Error>;

/// A builder for configuring `dwprod`.
#[derive(Default, Debug)]
pub struct Options {
    file: path::PathBuf,
}

impl Options {
    /// Construct a new `Options` for the shared library or executable at the
    /// given path.
    pub fn new<P: AsRef<path::Path>>(path: P) -> Options {
        Options {
            file: path.as_ref().into(),
        }
    }

    /// Finish configuring and get an iterator over the `DW_AT_producer`s in the
    /// compilation units of the configured files.
    pub fn producers<F, T>(self, mut f: F) -> Result<T>
    where
        F: FnMut(&mut Producers) -> T,
    {
        let mut contents = vec![];
        {
            let mut file = fs::File::open(self.file)?;
            file.read_to_end(&mut contents)?;
        }

        let file = object::File::parse(&contents[..])?;

        let debug_info = file.get_section(".debug_info")
            .ok_or_else(|| Error::from("missing .debug_info section"))?;
        let debug_info = DebugInfo::new(debug_info, gimli::NativeEndian);

        let debug_abbrev = file.get_section(".debug_abbrev")
            .ok_or_else(|| Error::from("missing .debug_abbrev section"))?;
        let debug_abbrev = DebugAbbrev::new(debug_abbrev, gimli::NativeEndian);

        let debug_str = file.get_section(".debug_str")
            .ok_or_else(|| Error::from("missing .debug_str section"))?;
        let debug_str = DebugStr::new(debug_str, gimli::NativeEndian);

        let headers = debug_info.units();

        let mut producers = Producers {
            debug_str,
            debug_abbrev,
            headers,
        };

        Ok(f(&mut producers))
    }
}

/// A `FallibleIterator` yielding `String` values for the `DW_AT_producer` for
/// each compilation unit in the configured file.
#[derive(Debug)]
pub struct Producers<'a> {
    debug_str: DebugStr<EndianBuf<'a, NativeEndian>>,
    debug_abbrev: DebugAbbrev<EndianBuf<'a, NativeEndian>>,
    headers: CompilationUnitHeadersIter<EndianBuf<'a, NativeEndian>>,
}

impl<'a> Producers<'a> {
    /// Get the next `DW_AT_producer`, if any.
    ///
    /// It is usually more ergonomic to use `FallibleIterator` combinators, but
    /// this method exists as an escape hatch.
    pub fn next(&mut self) -> Result<Option<String>> {
        loop {
            let unit_header = match self.headers.next()? {
                None => return Ok(None),
                Some(h) => h,
            };

            let abbrevs = unit_header.abbreviations(&self.debug_abbrev)?;
            let mut tree = unit_header.entries_tree(&abbrevs, None)?;
            let root = tree.root()?;
            let mut attrs = root.entry().attrs();

            while let Some(attr) = attrs.next()? {
                if let gimli::DW_AT_producer = attr.name() {
                    match attr.value() {
                        gimli::AttributeValue::DebugStrRef(offset) => {
                            return Ok(Some(self.debug_str.get_str(offset)?.to_string()?.into()));
                        }
                        gimli::AttributeValue::Block(data) => {
                            return Ok(Some(data.to_string()?.into()));
                        }
                        // Unknown kind of `DW_AT_producer` value; skip it.
                        _ => continue,
                    }
                }
            }

            // No DW_AT_producer for this compilation unit, so just continue to
            // the next unit header.
        }
    }
}

impl<'a> FallibleIterator for Producers<'a> {
    type Error = Error;
    type Item = String;

    fn next(&mut self) -> Result<Option<Self::Item>> {
        Producers::next(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
