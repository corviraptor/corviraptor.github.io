with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    cargo wasm-pack 
    trunk

    llvmPackages_9.lld
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