{ pkgs, ... }:

{
  packages = with pkgs; [
    pkg-config
    wayland
    alsa-lib-with-plugins
    udev
  ];
}
