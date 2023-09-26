savedcmd_samples/ftrace/sample-trace-array.mod := printf '%s\n'   sample-trace-array.o | awk '!x[$$0]++ { print("samples/ftrace/"$$0) }' > samples/ftrace/sample-trace-array.mod
