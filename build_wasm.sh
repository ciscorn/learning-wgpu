for i in {2..8} ; do
    cargo build --target wasm32-unknown-unknown --package tutorial$i --release --lib
    wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/release/tutorial$i.wasm
    cp index-template.html ./pkg/tutorial$i.html
    sed -i "s/tutorialX/tutorial$i/g" ./pkg/tutorial$i.html
done

cp index.html pkg/index.html

# static-web-server --root pkg --http2 --http2-tls-cert localhost.pem --http2-tls-key localhost-key.pem