with import <nixpkgs/nixos> {};
pkgs.mkShell {
  buildInputs = with pkgs; [
    (pkgs.latest.rustChannels.stable.rust.override { extensions = [ "rust-src" "rust-std" "rustfmt-preview" "clippy-preview" ]; })
    gnome3.gobject-introspection
    gnome3.gtk
    gnome3.glib
    gdk_pixbuf
    at-spi2-core
    git
  ];

  nativeBuildInputs = with pkgs; [
    openssl
    pkgconfig
  ];
}
