let
  rust_overlay = import (builtins.fetchGit {
    url = "https://github.com/oxalica/rust-overlay";
    rev = "2af83121f9d2c5281796e60e2b048906a84b9fac";
  });
  pkgs = import <nixpkgs> {overlays = [rust_overlay];};
  rustVersion = "latest";
  rust = pkgs.rust-bin.nightly.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "rust-analyzer"
      "rustc-codegen-cranelift" # Fast compile times
    ];
    targets = ["wasm32-unknown-unknown"];
  };
in
  pkgs.mkShell rec {
    nativeBuildInputs = with pkgs; [
      rust
      pkg-config
      wasm-bindgen-cli
    ];
    buildInputs = with pkgs; [
      mold
      clang
      llvmPackages.bintools
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
