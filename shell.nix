let
  rust_overlay = import (builtins.fetchGit {
    url = "https://github.com/oxalica/rust-overlay";
    rev = "9c8ea175cf9af29edbcff121512e44092a8f37e4";
  });
  pkgs = import <nixpkgs> {
    config.allowUnfree = true;
    config.android_sdk.accept_license = true;
    overlays = [rust_overlay];
  };
  rustVersion = "latest";
  rustToolchain = (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain.channel;
  rust = pkgs.rust-bin.${rustToolchain}.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "rust-analyzer"
      "rustc-codegen-cranelift" # Fast compile times
    ];
    targets = [
      "wasm32-unknown-unknown"
      "aarch64-linux-android"
      "armv7-linux-androideabi"
    ];
  };

  androidComposition = pkgs.androidenv.composeAndroidPackages {
    platformVersions = [
      "35"
    ];
    systemImageTypes = ["google_apis_playstore"];
    abiVersions = [
      "armeabi-v7a"
      "arm64-v8a"
    ];
    includeNDK = true;
    includeExtras = [
      "extras;google;auto"
    ];
  };
in
  pkgs.mkShell rec {
    packages = with pkgs; [
      just
      tracy
      cargo-apk
      cargo-xbuild
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
    JAVA_HOME = pkgs.openjdk17-bootstrap;
    JRE_HOME = pkgs.openjdk17-bootstrap;
    ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
    ANDROID_NDK_ROOT = "${ANDROID_HOME}/ndk-bundle";

    shellHook = ''
      export PATH="$HOME/.cargo/bin:$PATH"
    '';
  }
