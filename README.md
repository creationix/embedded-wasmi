Test building tiny static Linux binary using rust with embedded webassambly runtime.

```sh
# Build with musl and release config
cargo build --release --target x86_64-unknown-linux-musl
# Strip to reduce main binary size
strip target/x86_64-unknown-linux-musl/release/wasmi-runtime

# Check size
ls -lh target/x86_64-unknown-linux-musl/release/wasmi-runtime
# -rwxr-xr-x 2 tim tim 438K Jan  4 22:17 target/x86_64-unknown-linux-musl/release/wasmi-runtime

# Check it's static
file target/x86_64-unknown-linux-musl/release/wasmi-runtime
# target/x86_64-unknown-linux-musl/release/wasmi-runtime: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, BuildID[sha1]=99a91268e95717429c1341e4e77b3aed11f66940, stripped

# Run it
target/x86_64-unknown-linux-musl/release/wasmi-runtime
```