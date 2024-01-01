# chess

(Eventually) an efficient chess bot. For now, me attempting to recreate chess in Rust using
bitboards.

## Might not work on Windows

I use the `random` function from C (ffi). This exists on Linux. Might not on windows. I'm not sure,
I don't use it.

## Could have been `const`

Fun fact: magic number and lookup table generation *could* have been done at compile time
(except for seeded random numbers).
However, Rust doesn't yet allow `const_fn_in_trait` or const closures.
Rust should get better `const` support.
