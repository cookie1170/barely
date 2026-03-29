target-dir := shell("cargo metadata --format-version 1 --no-deps | jq -r '.target_directory'")
out-dir := 'target/web_pkg'

build-web example profile='dev':
    rm -rf '{{ out-dir }}'
    cargo build --example '{{ example }}' --target wasm32-unknown-unknown --profile '{{ profile }}'
    wasm-bindgen '{{ target-dir }}/wasm32-unknown-unknown/{{ profile }}/examples/{{ example }}.wasm' --out-dir {{ out-dir }} --no-typescript --target web
    mv '{{ out-dir }}/{{ example }}.js' '{{ out-dir }}/index.js'
    cp 'web/index.html' '{{ out-dir }}'

run-web example profile='dev': (build-web example profile)
    xdg-open 'http://localhost:4000'
    basic-http-server '{{ out-dir }}'
