target/release/cbinary: src/cbinary.c target/release/librustlib.so
	cc -o target/release/cbinary src/cbinary.c -lrustlib -Ltarget/release

target/release/librustlib.so:
	cargo build --release
	(cd target/release/ && ln -nsf librustlib-*.so librustlib.so)

run: target/release/cbinary
	(cd target/release/ && LD_LIBRARY_PATH=./:$$LD_LIBRARY_PATH ./cbinary)
