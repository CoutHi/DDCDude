{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
  };

  outputs = { self, nixpkgs, ...}: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs { inherit system; };
    in pkgs.mkShell {
      packages = with pkgs; [
        gtk4.dev
        glib.dev
        glibc.dev
        cairo.dev
        pango.dev
        graphene.dev
        gdk-pixbuf.dev
        pkg-config
      ];

      shellHook = ''
        echo "Welcome To Gtk Shell"
      '';
    };
  };
}
