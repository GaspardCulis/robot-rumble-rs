{
  description = "Best game";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {self, nixpkgs} @ inputs: let
    system ="x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = with pkgs; mkShell rec {
    nativeBuildInputs = [
      pkg-config
      rustc
      cargo
      rustfmt
      rust-analyzer
      clippy
    ];
    buildInputs = [
      clang
      llvmPackages.bintools
      udev alsa-lib vulkan-loader
      xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
      libxkbcommon wayland # To use the wayland feature
    ];
    LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  };
}
