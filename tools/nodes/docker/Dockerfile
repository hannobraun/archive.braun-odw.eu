FROM nixos/nix

# TASK: This isn't the recommended way to handle secrets like SSH keys. See:
#       https://docs.docker.com/engine/reference/builder/#arg
#
#       I don't think it matters in this case, since this image is just used
#       locally and not distributed to anyone else, but it would be nicer to do
#       it the proper way, of course.
ARG SSH_KEY
ENV SSH_KEY=$SSH_KEY

RUN nix-env --install nixops

ENV NIXOPS_STATE=deployments.nixops
ENV NIXOPS_DEPLOYMENT=reineke

COPY * nodes/
RUN nixops create nodes/*.nix

ENTRYPOINT echo $SSH_KEY
# TASK: Add something like `ENTRYPOINT nixops deploy`.
