# scoped_alloca
Requires `clang` and `ar` to be installed on your system

## Drawbacks
Only works in release mode (`--release`) at the moment. Debug doesn't seem to enable the features necessary for inlining the alloca wrapper (but maybe we can have them enabled in debug?)
Only confirmed to work in rustc 1.26; older versions without LTO optimizations may not work at all (even in release mode)
