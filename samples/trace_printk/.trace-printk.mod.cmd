savedcmd_samples/trace_printk/trace-printk.mod := printf '%s\n'   trace-printk.o | awk '!x[$$0]++ { print("samples/trace_printk/"$$0) }' > samples/trace_printk/trace-printk.mod
