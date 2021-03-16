FROM nixos/nix

RUN nix-env --install nixops
RUN nix-env --install openssh

ENV NIXOPS_STATE=deployments.nixops
ENV NIXOPS_DEPLOYMENT=reineke

# TASK: This isn't the recommended way to handle secrets like SSH keys. See:
#       https://docs.docker.com/engine/reference/builder/#arg
#
#       I don't think it matters in this case, since this image is just used
#       locally and not distributed to anyone else, but it would be nicer to do
#       it the proper way, of course.
ARG SSH_KEY
RUN mkdir ~/.ssh
RUN echo "${SSH_KEY}" > ~/.ssh/id_rsa
RUN ssh-keyscan reineke.nodes.braun-odw.eu >> ~/.ssh/known_hosts

COPY * nodes/
RUN nixops create nodes/*.nix

ENTRYPOINT cat ~/.ssh/id_rsa
# TASK: Add something like `ENTRYPOINT nixops deploy`.
