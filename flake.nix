{
  description = "rust shell dev";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
          nativeBuildInputs = [
            makeWrapper
            pkg-config
          ];
          buildInputs = [
            openssl
            libxkbcommon
            libGL
            just
            sd
            cargo-watch
            cargo-release
            wasm-pack
            udev
            alsa-lib
            vulkan-loader

            libxkbcommon
            wayland
            watchexec

            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11

            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-unknown-unknown" ];
            })
          ];
          env = {
            ZSTD_SYS_USE_PKG_CONFIG = true;
          };
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          shellHook = ''
            alias run="cargo run"
            alias ls=exa
            alias find=fd
            alias build="cargo build"
            alias web="cargo run --release --target wasm32-unknown-unknown"
          '';
        };
      }
    );
}
