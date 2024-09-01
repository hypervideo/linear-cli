{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        lr = pkgs.rustPlatform.buildRustPackage {
          name = "lr";
          src = ./.;
          cargoHash = "sha256-RCNjt24Egs5FQkJO4sODe9alycL2wVSyIABGMGb03FU=";
        };
      in
      {
        packages = { inherit lr; };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            clippy
            pkg-config
            graphql-client
          ];

          buildInputs = with pkgs; [
            openssl
            clang
          ] ++ (if pkgs.stdenv.isDarwin then [ libiconv ] else [ ]);

          packages = with pkgs; [
            rust-analyzer
            rustfmt
          ];

          RUST_BACKTRACE = "1";
          RUST_LOG = "info,lr=debug";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        };
      }
    );
}
