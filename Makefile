PROJECT=plugnrust
LDFLAGS += -bundle -exported_symbols_list symbols.list

SRC = src/lib.rs

debug: target/debug/lib$(PROJECT).bin
release: target/release/lib$(PROJECT).bin
	
target/debug/lib$(PROJECT).a: $(SRC)
	cargo build

target/debug/lib$(PROJECT).bin: target/debug/lib$(PROJECT).a symbols.list
	clang $(LDFLAGS) -o $@ target/debug/lib$(PROJECT).a

target/release/lib$(PROJECT).a: $(SRC)
	cargo build --release

target/release/lib$(PROJECT).bin: target/release/lib$(PROJECT).a symbols.list
	clang $(LDFLAGS) -o $@ target/release/lib$(PROJECT).a
	
clean:
	cargo clean
	
