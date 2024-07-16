{
  description = "Delta Agent Oxide Devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        # Join our system package list with the rust overlay so that the stuff from
        # the overlay is under pkgs.
        pkgs = import nixpkgs { inherit system overlays; };

        sharedHook = ''
          export PROTOBUF_LOCATION=${pkgs.protobuf}
          export PROTOC_INCLUDE=$PROTOBUF_LOCATION/include
          export PROTOC=$PROTOBUF_LOCATION/bin/protoc
          export LLVM_TOOL_PATH=${pkgs.rustc.llvmPackages.llvm}/bin
          export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib/
          export RUST_LOG="info"
          export REPORT=true
        '';
      in
      with pkgs; {
        devShells.default = mkShell
          {
            buildInputs = [
              openssl
              pkg-config
              rustc.llvmPackages.llvm
              llvmPackages.bintools
              rust-analyzer-unwrapped
              alsa-lib
              systemd
            ];

            shellHook = sharedHook;
          };
      });

}

