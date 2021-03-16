{
  reineke =
    { config, pkgs, ... }:
    {
      deployment = {
        targetHost = "reineke.nodes.braun-odw.eu";
        # TASK: Set up private SSH key.
      };

      imports = [
        ./reineke-hardware.nix
      ];
    };
}

# TASK: Import basic configuration from server.
# TASK: Deploy an SSH key, set up login into root using that key:
#       - https://nixos.org/manual/nixos/stable/index.html#sec-ssh
#       - Search for "deployment.keys" in NixOps manual.
# TASK: Set `users.mutableUsers` to `false`:
#       https://nixos.org/manual/nixos/stable/index.html#sec-user-management
# TASK: Set up automatic upgrades:
#       https://nixos.org/manual/nixos/stable/index.html#sec-upgrading-automatic
