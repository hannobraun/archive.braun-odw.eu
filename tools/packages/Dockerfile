FROM nixos/nix

COPY * sites/

# TASK: This is not the correct command, it seems. Seems I need to write more of
#       the documentation instead of trying to take shortcuts.
ENTRYPOINT nix-build sites
