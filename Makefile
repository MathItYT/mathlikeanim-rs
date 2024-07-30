all:
	wasm-pack build --target web --out-dir javascript/browser --features browser --no-default-features
	wasm-pack build --target nodejs --out-dir javascript/node --features node --no-default-features
	cp javascript/browser/mathlikeanim_rs_bg.wasm javascript/mathlikeanim_rs_bg.wasm