{
  description = "Devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        #overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system; };
        commonPkgs = [
        ];

        # shared ShellHook Elements.
        sharedHook = ''
        '';
      in
      with pkgs; {
        devShells.default = mkShell
          {
            buildInputs = [
              kubernetes-helm
            ];

            shellHook = sharedHook;
          };
      });
}
