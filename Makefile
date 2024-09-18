build:
	wasm-pack build --target web --out-dir javascript/pkg
pack:
	wasm-pack pack javascript/pkg
publish:
	wasm-pack publish