.PHONY: buildjs

PKG_OUT_DIR:=pkg

TARGET_WASM=target/wasm32-unknown-unknown/release/webtransport_wit.wasm
COMPONENT_WASM=$(PKG_OUT_DIR)/component.wasm

$(TARGET_WASM):
	cargo build --release

$(COMPONENT_WASM): $(TARGET_WASM) | $(PKG_OUT_DIR)
	wasm-tools component new \
        $(TARGET_WASM) \
        -o $(COMPONENT_WASM) 

buildjs: $(COMPONENT_WASM)
	deno run -A npm:@bytecodealliance/jco transpile \
		$(COMPONENT_WASM) \
		--no-nodejs-compat \
		--base64-cutoff 5000000 \
		--tla-compat \
		--map 'tombind:webtransport/webtransport=../js/webtransport.js' \
		-o pkg

$(PKG_OUT_DIR):
	mkdir -p $@

