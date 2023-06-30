{
  description = "Tonic Hello World";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
      nixpkgs,
      flake-utils,
      ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      # Join our system package list with the rust overlay so that the stuff from
      # the overlay is under pkgs.
      pkgs = import nixpkgs {inherit system;};
      # Stuff that we always want.
      # shared ShellHook Elements.
    in
      with pkgs; {
        devShells.default =
          mkShell
            {
              buildInputs = [
                protobuf
              ];
            };
      });
}
