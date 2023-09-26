savedcmd_samples/ftrace/ftrace-direct.mod := printf '%s\n'   ftrace-direct.o | awk '!x[$$0]++ { print("samples/ftrace/"$$0) }' > samples/ftrace/ftrace-direct.mod
