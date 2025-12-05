{ pkgs, lib, ... }:

let
  libs = with pkgs; [
    wayland
    udev
    libxkbcommon
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
  ];
in
{
  packages =
    with pkgs;
    [
      pkg-config
      alsa-lib-with-plugins
    ]
    ++ libs;

  env.LD_LIBRARY_PATH = lib.makeLibraryPath libs;
}
