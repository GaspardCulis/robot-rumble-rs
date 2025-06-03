let
  rust_overlay = import (builtins.fetchGit {
    url = "https://github.com/oxalica/rust-overlay";
    rev = "9c8ea175cf9af29edbcff121512e44092a8f37e4";
  });
  pkgs = import <nixpkgs> {overlays = [rust_overlay];};
  rustVersion = "latest";
  rustToolchain = (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain.channel;
  rust = pkgs.rust-bin.${rustToolchain}.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "rust-analyzer"
      "rustc-codegen-cranelift" # Fast compile times
    ];
    targets = ["wasm32-unknown-unknown"];
  };
in
  pkgs.mkShell rec {
    packages = with pkgs; [
      just
      tracy
    ];
    nativeBuildInputs = with pkgs; [
      rust
      pkg-config
      wasm-bindgen-cli
    ];
    buildInputs = with pkgs; [
      mold
      clang
      llvmPackages.bintools
      # fixes libstdc++ issues and libgl.so issues
      stdenv.cc.cc.lib
      udev
      alsa-lib
      vulkan-loader
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr # To use the x11 feature
      libxkbcommon
      wayland # To use the wayland feature
    ];
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

    shellHook = ''
      export PATH="$HOME/.cargo/bin:$PATH"
    '';
  }
