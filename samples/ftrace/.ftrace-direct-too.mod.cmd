savedcmd_samples/ftrace/ftrace-direct-too.mod := printf '%s\n'   ftrace-direct-too.o | awk '!x[$$0]++ { print("samples/ftrace/"$$0) }' > samples/ftrace/ftrace-direct-too.mod
