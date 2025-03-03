build:
	wasm-pack build --target web --out-dir pkg --scope mathlikeanim-rs
copy-wasm:
	cp -af ./pkg/. ./examples/hello-world/node_modules/@mathlikeanim-rs/mathlikeanim-rs
pack:
	wasm-pack pack --pkg-dir pkg
publish:
	wasm-pack publish --pkg-dir pkg --access public
publish-cargo:
	cargo publish
build-ts:
	cd typescript && npm run build
pack-ts:
	cd typescript && npm pack
publish-ts:
	cd typescript && npm publish
copy-ts:
	cp -af ./typescript/dist/. ./examples/hello-world/node_modules/@mathlikeanim-rs/renderer/dist
gen-docs:
	cd docs && jsdoc -c jsdoc.conf.js
serve-docs:
	cd docs && http-server
serve-example:
	cd examples/hello-world && http-server
copy-readme-ts:
	cp -af ./README.md ./typescript/README.md