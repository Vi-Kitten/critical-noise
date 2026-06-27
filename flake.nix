{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs = { self, nixpkgs }: let
        pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
        devShells."x86_64-linux".default = pkgs.mkShell {
            buildInputs = with pkgs; [
                lld
                mold
                rustc
                libclang
                cargo
                rust-analyzer
                clippy
                rustfmt
                bacon
            ];
            nativeBuildInputs = [ pkgs.rustPlatform.bindgenHook pkgs.pkg-config pkgs.libclang ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            env.LIBCLANG_PATH = "${pkgs.libclang}/lib";
        };
    };
}
