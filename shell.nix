with import <nixpkgs> {};

mkShell {
  name = "rust-env";
  nativeBuildInputs = [
    rustc
    cargo 
    wasm-pack 
    trunk
    lld_18
    cargo-binutils

    inotify-tools
    rustup
    rust-analyzer

    pkg-config
  ];
  buildInputs = [
    openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}