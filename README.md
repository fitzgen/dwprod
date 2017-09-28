# dwprod

Find the `DW_AT_producer` for all compilation units within a shared library or
executable.

[![](https://docs.rs/dwprod/badge.svg)](https://docs.rs/dwprod/) [![](http://meritbadge.herokuapp.com/dwprod)](https://crates.io/crates/dwprod) [![](https://img.shields.io/crates/d/dwprod.png)](https://crates.io/crates/dwprod) [![Unix Build Status](https://travis-ci.org/fitzgen/dwprod.png?branch=master)](https://travis-ci.org/fitzgen/dwprod)

#### What is `DW_AT_producer`?

The `DW_AT_producer` is an attribute within DWARF debug info that says what
compiler was used to create each compilation unit that ended up within a given
shared library or executable.

#### Usage

##### As a Library

First, add this to your `Cargo.toml`:

```toml
[dependencies.dwprod]
version = "0.1.0"
# Do not build the command line `dwprod` executable.
default-features = false
```

Then, import the `dwprod` crate and use it to iterate over `DW_AT_producer`
values:

```rust
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
```

##### As a Command Line Tool

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

License: Apache-2.0/MIT
