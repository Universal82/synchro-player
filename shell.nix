{ pkgs ? import <nixpkgs> { } }:

let
  dlopenLibraries = with pkgs; [
    libxkbcommon

    # GPU backend
    vulkan-loader
    # libGL

    # Window system
    wayland
    # xorg.libX11
    # xorg.libXcursor
    # xorg.libXi
  ];
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    pkg-config
  ];

  buildInputs = with pkgs; [
    glib
    
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-bad
    gst_all_1.gst-plugins-ugly
    gst_all_1.gst-libav

    fontconfig
    # libvaxis     # Required if using newer terminal backends
    
    # Wayland dependencies
    wayland
    libxkbcommon
    
    # X11 dependencies
    # xorg.libX11
    # xorg.libXcursor
    # xorg.libXrandr
    # xorg.libXi
    
    # Graphics drivers / OpenGL
    libGL
    vulkan-loader
  ];

  env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${pkgs.lib.makeLibraryPath dlopenLibraries}";
  env.RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
}
