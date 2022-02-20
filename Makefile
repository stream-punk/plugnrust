PROJECT=plugnrust
LDFLAGS += -bundle -exported_symbols_list symbols.list

SRC = \
    Makefile \
    Cargo.toml \
    Cargo.lock \
    src/lib.rs

fast: target/debug/lib$(PROJECT).bin
debug: target/universal/debug/lib$(PROJECT).bin
release: target/universal/release/lib$(PROJECT).bin

target/debug/lib$(PROJECT).a: $(SRC)
	cargo build

target/x86_64-apple-darwin/debug/lib$(PROJECT).a: $(SRC)
	cargo build --target x86_64-apple-darwin

target/debug/lib$(PROJECT).bin: target/debug/lib$(PROJECT).a symbols.list
	clang $(LDFLAGS) -o $@ target/debug/lib$(PROJECT).a

target/x86_64-apple-darwin/debug/lib$(PROJECT).bin: \
		target/x86_64-apple-darwin/debug/lib$(PROJECT).a symbols.list
	clang $(LDFLAGS) -arch x86_64 -o $@ target/x86_64-apple-darwin/debug/lib$(PROJECT).a
	
target/universal/debug/lib$(PROJECT).bin: \
		target/debug/lib$(PROJECT).bin \
		target/x86_64-apple-darwin/debug/lib$(PROJECT).bin
	mkdir -p target/universal/debug
	lipo -create $^ -output $@

target/release/lib$(PROJECT).a: $(SRC)
	cargo build --release
	
target/x86_64-apple-darwin/release/lib$(PROJECT).a: $(SRC)
	cargo build --release --target x86_64-apple-darwin

target/release/lib$(PROJECT).bin: target/release/lib$(PROJECT).a symbols.list
	clang $(LDFLAGS) -o $@ target/release/lib$(PROJECT).a
	
target/x86_64-apple-darwin/release/lib$(PROJECT).bin: \
		target/x86_64-apple-darwin/release/lib$(PROJECT).a symbols.list
	clang $(LDFLAGS) -arch x86_64 -o $@ target/x86_64-apple-darwin/release/lib$(PROJECT).a

target/universal/release/lib$(PROJECT).bin: \
		target/release/lib$(PROJECT).bin \
		target/x86_64-apple-darwin/release/lib$(PROJECT).bin
	mkdir -p target/universal/release
	lipo -create $^ -output $@
	strip -S $@
	
clean:
	cargo clean
