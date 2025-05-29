{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          pkgs.pkg-config
          pkgs.glib
          pkgs.openssl
          pkgs.openssl.dev
          pkgs.python3
          pkgs.alsa-lib
          pkgs.atk
          pkgs.gtk3
          pkgs.pango
          pkgs.systemd.dev
          pkgs.xdotool
          pkgs.gcc.cc
        ];
        shellHook = ''
          export LD_LIBRARY_PATH="${pkgs.openssl.out}/lib:${pkgs.gcc.cc.lib}/lib:$LD_LIBRARY_PATH"
        '';
      };
    };
}
