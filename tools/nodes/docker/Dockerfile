# TASK: Define an ARG for the private ssh key, passt it via `--build-arg`:
#       https://docs.docker.com/engine/reference/builder/#arg

FROM nixos/nix

RUN nix-env -i nixops

ENTRYPOINT which nixops
