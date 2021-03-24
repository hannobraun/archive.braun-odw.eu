# TASK: Make this work. Relevant documentation:
#       - https://nixos.org/manual/nixpkgs/stable/#compiling-rust-applications-with-cargo
#       - https://nixos.wiki/wiki/Packaging/Tutorial

with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "sites";

    # Specify source directory.
    #
    # TASK: This is an absolute path, which works because we happen to run in a
    #       Docker container and know where we copied the source to. But it is a
    #       bad hack. There must be a way to refer to the directory that
    #       default.nix is located in?
    src = "/sites";

    buildInputs = [ cargo ];

    buildPhase = "
        export CARGO_HOME=$(mktemp -d cargo-home.XXXXXXXX)
        cargo build --release
    ";

    installPhase = "
        mkdir -p $out/bin
        cp target/release/sites $out/bin
    ";
}
