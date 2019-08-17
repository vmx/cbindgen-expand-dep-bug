cbindgen-expand-dep-bug
=======================

This is a minimal example for reproducing Bug #384.

If you run `cargo build`, then the `bindings.h` will look like this:

```c
/* Generated with cbindgen:0.9.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

uint32_t get_x(const dep_struct *dep_struct);
```

It is missing the `dep_struct` type definition:

```c
typedef struct {
  uint32_t x;
  double y;
} dep_struct;
```

That is there, when you comment out the macro expansion in the `cbindgen.toml`:

```toml
[parse.expand]
crates = ["cbindgen-dep"]
```

This code is based on the minimal example created for
https://github.com/eqrion/cbindgen/issues/292, thanks @acfoltzer.
