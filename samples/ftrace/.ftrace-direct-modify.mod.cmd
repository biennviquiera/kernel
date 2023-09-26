savedcmd_samples/ftrace/ftrace-direct-modify.mod := printf '%s\n'   ftrace-direct-modify.o | awk '!x[$$0]++ { print("samples/ftrace/"$$0) }' > samples/ftrace/ftrace-direct-modify.mod
