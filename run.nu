
def build_wasm [--release] {
  if $release {
    # TODO: wasm optimizations
    cargo build --target wasm32-unknown-unknown --manifest-path wasm/Cargo.toml --target-dir wasm/target --release
    wasm-bindgen --no-typescript --target bundler --out-dir app/wasm wasm/target/wasm32-unknown-unknown/release/*.wasm
  } else {
    cargo build --target wasm32-unknown-unknown --manifest-path wasm/Cargo.toml --target-dir wasm/target
    wasm-bindgen --no-typescript --target bundler --out-dir app/wasm wasm/target/wasm32-unknown-unknown/debug/*.wasm
  }
}

def clean_wasm [] {
  try { rm --verbose -r app/wasm }
  cargo clean --manifest-path wasm/Cargo.toml
}

def build_app [] {
  cd app
  npm install
  node esbuild.js build
  cd ..
}

def dev_app [] {
  cd app
  npm install
  node esbuild.js dev
  cd ..
}

def clean_app [] {
  try { rm -r app/node_modules --verbose }
  for i in (glob app/www/* | where { ($in | path basename) not-in [index.html .gitignore] }) {
    rm $i --verbose
  }
}

# utility for invoking build commands. Runs subcommand 'watch --both' by default
def main [] { main dev --target both }

def 'main build' [] {
  build_wasm --release
  build_app
}

def 'main dev' [
  --target(-t): string # one of [app wasm both]
] {
  match $target {
    app => { dev_app },
    wasm => {
      build_wasm
      watch wasm/src { build_wasm }
    },
    both => {
      build_wasm
      [{ dev_app } { watch wasm/src { build_wasm } }] | par-each { do $in }
    },
    _ => { error make { msg: $'target ($target) is not part of [app wasm both]' } } 
  }
}

def 'main clean' [] {
  clean_wasm
  clean_app
}
