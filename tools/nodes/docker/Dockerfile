# TASK: Define an ARG for the private ssh key, pass it via `--build-arg`:
#       https://docs.docker.com/engine/reference/builder/#arg

FROM nixos/nix

RUN nix-env -i nixops

ENV NIXOPS_STATE=deployments.nixops
ENV NIXOPS_DEPLOYMENT=reineke

COPY * nodes/
RUN nixops create nodes/*.nix

ENTRYPOINT nixops deploy
