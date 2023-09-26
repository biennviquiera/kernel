savedcmd_samples/rust/rust_minimal.mod := printf '%s\n'   rust_minimal.o | awk '!x[$$0]++ { print("samples/rust/"$$0) }' > samples/rust/rust_minimal.mod
