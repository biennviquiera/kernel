savedcmd_samples/rust/rust_minimal.o := RUST_MODFILE=samples/rust/rust_minimal rustc --edition=2021 -Zbinary_dep_depinfo=y -Dunsafe_op_in_unsafe_fn -Drust_2018_idioms -Dunreachable_pub -Dnon_ascii_idents -Wmissing_docs -Drustdoc::missing_crate_level_docs -Dclippy::correctness -Dclippy::style -Dclippy::suspicious -Dclippy::complexity -Dclippy::perf -Dclippy::let_unit_value -Dclippy::mut_mut -Dclippy::needless_bitwise_bool -Dclippy::needless_continue -Dclippy::no_mangle_with_rust_abi -Wclippy::dbg_macro --target=./scripts/target.json -Cpanic=abort -Cembed-bitcode=n -Clto=n -Cforce-unwind-tables=n -Ccodegen-units=1 -Csymbol-mangling-version=v0 -Crelocation-model=static -Zfunction-sections=n -Dclippy::float_arithmetic -Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2 -Ztune-cpu=generic -Cno-redzone=y -Ccode-model=kernel -Copt-level=2 -Cdebug-assertions=n -Coverflow-checks=y -Cforce-frame-pointers=y -Cdebuginfo=2  --cfg MODULE  @./include/generated/rustc_cfg -Zallow-features=allocator_api,const_refs_to_cell -Zcrate-attr=no_std -Zcrate-attr='feature(allocator_api,const_refs_to_cell)' --extern alloc --extern kernel --crate-type rlib -L ./rust/ --crate-name rust_minimal --emit=dep-info=samples/rust/.rust_minimal.o.d --emit=obj=samples/rust/rust_minimal.o samples/rust/rust_minimal.rs

source_samples/rust/rust_minimal.o := samples/rust/rust_minimal.rs

deps_samples/rust/rust_minimal.o := \
  /home/bienn/linux-cs429-fall-2023/rust/libcore.rmeta \
  /home/bienn/linux-cs429-fall-2023/rust/libcompiler_builtins.rmeta \
  /home/bienn/linux-cs429-fall-2023/rust/libkernel.rmeta \
  /home/bienn/linux-cs429-fall-2023/rust/liballoc.rmeta \
  /home/bienn/linux-cs429-fall-2023/rust/libmacros.so \
  /home/bienn/linux-cs429-fall-2023/rust/libbindings.rmeta \
  /home/bienn/linux-cs429-fall-2023/rust/libbuild_error.rmeta \

samples/rust/rust_minimal.o: $(deps_samples/rust/rust_minimal.o)

$(deps_samples/rust/rust_minimal.o):
