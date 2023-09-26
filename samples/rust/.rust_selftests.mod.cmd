savedcmd_samples/rust/rust_selftests.mod := printf '%s\n'   rust_selftests.o | awk '!x[$$0]++ { print("samples/rust/"$$0) }' > samples/rust/rust_selftests.mod
