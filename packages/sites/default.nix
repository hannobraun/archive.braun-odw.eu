# TASK: Make this work. Relevant documentation:
#       - https://nixos.org/manual/nixpkgs/stable/#compiling-rust-applications-with-cargo
#       - https://nixos.wiki/wiki/Packaging/Tutorial

with import <nixpkgs> {};

rustPlatform.buildRustPackage rec {
    name = "sites";

    # TASK: Figure out how to specify the local directory as the source.

    # TASK: Figure out what to do with this. Ideally, I'd want Nix to ignore the
    #       checksum. I'm building a local project, after all.
    #
    #       The current value is copied from somewhere, because Nix expects a
    #       string with a valid length for a SHA-256 checksum, but I haven't
    #       gotten to the point where it would complain about the wrong value
    #       (and give me the right one) yet.
    cargoSha256 = "0x2g1jqygyr5wiwg4ma1nd7w4ydpy82z9gkcv8vh2v8dn3y58v5m";
}
