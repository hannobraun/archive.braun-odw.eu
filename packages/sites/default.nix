# TASK: Make this work. Relevant documentation:
#       - https://nixos.org/manual/nixpkgs/stable/#compiling-rust-applications-with-cargo
#       - https://nixos.wiki/wiki/Packaging/Tutorial

with import <nixpkgs> {};

rustPlatform.buildRustPackage rec {
    name = "sites";

    # Specify source directory.
    #
    # TASK: This is an absolute path, which works because we happen to run in a
    #       Docker container and know where we copied the source to. But it is a
    #       bad hack. There must be a way to refer to the directory that
    #       default.nix is located in?
    src = "/sites";

    # Nix expects `src` to be an archive file, but we're pointing to a
    # directory. To make this work, we need to override the unpack phase.
    #
    # In addition to copying the files from the source directory, this also
    # makes sure `Cargo.lock` exists. Nix checks for this file's existence and
    # will fail otherwise.
    unpackPhase = "
        cp -r $src/* .

        HOME=. # Nix sets this to a non-existing directory
        cargo update
    ";

    # TASK: Figure out what to do with this. Ideally, I'd want Nix to ignore the
    #       checksum. I'm building a local project, after all.
    #
    #       The current value is copied from somewhere, because Nix expects a
    #       string with a valid length for a SHA-256 checksum, but I haven't
    #       gotten to the point where it would complain about the wrong value
    #       (and give me the right one) yet.
    cargoSha256 = "1487n0vwm73i2sjz4cqpls1191lglk9wravmjc2nlzqwc96lqh7q";

    # TASK: I'm not quite sure what's happening, but from the build output it
    #       seems that Nix is building the package twice for some reason? The
    #       second time around, it runs `cargo build` with `--frozen`, which
    #       fails because a dependency needs to be downloaded. Previoulsy, there
    #       are messages about dependencies being vendored.
}
