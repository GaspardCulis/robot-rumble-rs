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
    systemImageTypes = ["default"];
    abiVersions = [
      "armeabi-v7a"
      "arm64-v8a"
    ];
    includeNDK = true;
    includeExtras = [
      "extras;google;auto"
    ];
  };
  androidsdk = androidComposition.androidsdk;
  sdk_root = "${androidsdk}/libexec/android-sdk";
  ndk_root = "${sdk_root}/ndk-bundle";
  ndk_path = "${ndk_root}/toolchains/llvm/prebuilt/linux-x86_64/bin";
  java = pkgs.openjdk17-bootstrap;
in
  pkgs.mkShell rec {
    packages = with pkgs; [
      just
      tracy
      cargo-apk
      cargo-ndk
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
      # Android
      java
      gradle
    ];
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    JAVA_HOME = java;
    JRE_HOME = java;

    # https://github.com/katyo/oboe-rs/blob/7ea2b9b3bc9cdfa9ed4cbfeafdcafb47b3fac4e7/default.nix#L4
    ANDROID_HOME = "${sdk_root}";
    ANDROID_NDK_ROOT = "${ndk_root}";
    NDK_HOME = "${ndk_root}";

    shellHook = ''
      export PATH="$HOME/.cargo/bin:$PATH"
      export PATH="${ndk_path}:${androidsdk}/bin:$PATH";
    '';
  }
