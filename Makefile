
build:
	cd contract && cargo build --target wasm32-unknown-unknown --release && cd ../ && \
	cp contract/target/wasm32-unknown-unknown/release/abs_wallet.wasm ./out/main.wasm


deploy-prod:
	make build && \
	near dev-deploy --wasmFile ./out/main.wasm
