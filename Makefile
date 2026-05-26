WASM_GMP_TARGET := wasm32-unknown-emscripten
WASM_PROFILE := release
WASM_GMP_JS := target/$(WASM_GMP_TARGET)/$(WASM_PROFILE)/vdf-wasm-gmp.js
WASM_GMP_WASM := target/$(WASM_GMP_TARGET)/$(WASM_PROFILE)/vdf_wasm_gmp.wasm
NODE_GMP_WASM_DIR := examples/node-wasm-gmp
NODE_GMP_PKG_DIR := $(NODE_GMP_WASM_DIR)/pkg
GMP_VERSION := 6.3.0
GMP_URL := https://ftpmirror.gnu.org/gnu/gmp/gmp-$(GMP_VERSION).tar.xz
GMP_SRC_ROOT := target/wasm-gmp-src
GMP_TARBALL := $(GMP_SRC_ROOT)/gmp-$(GMP_VERSION).tar.xz
GMP_SRC_DIR := $(GMP_SRC_ROOT)/gmp-$(GMP_VERSION)
GMP_WASM_PREFIX := $(CURDIR)/target/wasm-gmp-build
GMP_WASM_LIB := $(GMP_WASM_PREFIX)/lib/libgmp.a
JOBS ?= $(shell sysctl -n hw.ncpu 2>/dev/null || nproc 2>/dev/null || echo 4)
WASM_GMP_RUSTFLAGS := -L native=$(GMP_WASM_PREFIX)/lib \
	-C link-arg=-sMODULARIZE=1 \
	-C link-arg=-sEXPORT_NAME=createVdfGmpModule \
	-C link-arg=-sENVIRONMENT=node \
	-C link-arg=-sALLOW_MEMORY_GROWTH=1 \
	-C link-arg=-sWASM_BIGINT \
	-C link-arg=-sEXPORTED_RUNTIME_METHODS=["HEAPU8"] \
	-C link-arg=-sEXPORTED_FUNCTIONS=["_main","_vdf_alloc","_vdf_dealloc","_vdf_last_result_ptr","_vdf_last_result_len","_vdf_last_error_ptr","_vdf_last_error_len","_vdf_solve_wesolowski","_vdf_verify_wesolowski"]

.PHONY: wasm node-bindings node-test node-bench gmp-wasm-lib node-gmp-bindings node-gmp-test node-gmp-bench clean-node-wasm-gmp

wasm: node-gmp-bindings

node-bindings: node-gmp-bindings

node-test: node-gmp-test

node-bench: node-gmp-bench

$(GMP_TARBALL):
	mkdir -p $(GMP_SRC_ROOT)
	curl -L $(GMP_URL) -o $(GMP_TARBALL)

$(GMP_SRC_DIR): $(GMP_TARBALL)
	rm -rf $(GMP_SRC_DIR)
	tar -xf $(GMP_TARBALL) -C $(GMP_SRC_ROOT)

$(GMP_WASM_LIB): $(GMP_SRC_DIR)
	cd $(GMP_SRC_DIR) && CC_FOR_BUILD=/usr/bin/clang emconfigure ./configure --host=$(WASM_GMP_TARGET) --disable-assembly --disable-shared --enable-static --prefix="$(GMP_WASM_PREFIX)" CFLAGS="-O3" ABI=standard
	cd $(GMP_SRC_DIR) && emmake make -j$(JOBS)
	cd $(GMP_SRC_DIR) && emmake make install

gmp-wasm-lib: $(GMP_WASM_LIB)

node-gmp-bindings: $(GMP_WASM_LIB)
	rustup target add $(WASM_GMP_TARGET)
	RUSTFLAGS='$(WASM_GMP_RUSTFLAGS)' cargo build -p vdf-wasm-gmp --target $(WASM_GMP_TARGET) --release
	mkdir -p $(NODE_GMP_PKG_DIR)
	cp $(WASM_GMP_JS) $(NODE_GMP_PKG_DIR)/vdf-wasm-gmp.js
	cp $(WASM_GMP_WASM) $(NODE_GMP_PKG_DIR)/vdf_wasm_gmp.wasm

node-gmp-test: node-gmp-bindings
	cd $(NODE_GMP_WASM_DIR) && node test.js

node-gmp-bench: node-gmp-bindings
	cd $(NODE_GMP_WASM_DIR) && node bench.js

clean-node-wasm-gmp:
	rm -rf $(NODE_GMP_PKG_DIR)
