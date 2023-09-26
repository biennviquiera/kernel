savedcmd_samples/rust/rust_semaphore.mod := printf '%s\n'   rust_semaphore.o | awk '!x[$$0]++ { print("samples/rust/"$$0) }' > samples/rust/rust_semaphore.mod
