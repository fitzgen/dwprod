# dwprod

Find the `DW_AT_producer` for all compilation units within an shared library or
executable.

Here is the result of running `dwprod` on itself:

```
$ cargo run ~/dwprod/target/debug/dwprod
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/dwprod /home/fitzgen/dwprod/target/debug/dwprod`
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
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
GNU C 4.8.5 -m64 -mtune=generic -march=x86-64 -g3 -O3 -O2 -O2 -std=gnu11 -fvisibility=hidden -funroll-loops -ffunction-sections -fdata-sections -fPIC
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
clang LLVM (rustc version 1.22.0-nightly (088216fb9 2017-09-04))
GNU C11 6.3.0 20170406 -mtune=generic -march=x86-64 -g -O2 -O3 -std=gnu11 -fgnu89-inline -fno-stack-protector -fmerge-all-constants -frounding-math -fPIC -ftls-model=initial-exec
```

License: Apache-2.0/MIT
