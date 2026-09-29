{ pkgs ? import <nixpkgs> {} }:

let
  libs = with pkgs; [
    wayland
    libxkbcommon
    libGL
    vulkan-loader   # needed by pixels (wgpu)
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];
in
pkgs.mkShell {
  nativeBuildInputs = [ pkgs.pkg-config ];
  buildInputs = libs;
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
}
