{
  reineke =
    { config, pkgs, ... }:
    {
      deployment = {
        targetHost = "reineke.nodes.braun-odw.eu";
        keys.root.text = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQCpUT/y8rl08ZxWUVIAj2LHh1dapmG8S6DqZLDaqKlBEhEj3w/DC3991+NA/3I8O9ITvwGeox3EC/WMNb0NYq1jgLACvIc+ig14b69U86HbVMcTJqyCkc0Bf/zgbEnH+HxzKsPGFBLjlISHIInwwquoxDCa3sR8LVuhCUc2YYiRcgIbXxUcwxlMrSrJuKmsfMDBdGACTK4AvgR7q7SXVjypCvU+joPmX9d8IKZRg59AQkWnZAdulNPF/xk53wSZlkNynh6JhWjU28x/1XUSkK+JHVKUoaQgRFmf9OdqmT7YCi9KfP6/ipAJcB41N1/zDwahIy6sGxtx+TjEPGKsGu2RJMKdjwXioMcQNgoHhuQhJZTiimnJJz5Y6DzUdgNsZkRHFmoinbZ71TFGGppLijMC173sOioMSToNuyHEJKu91bDDxJfaZE9DQCh4nGxEJYNxUwlO2YIMzFVHWyYQ5IpSi8CfWmnWIJTLltVQOzPnFMt5N1ZCIN/O0NYMHzPmjE8= root@reineke.nodes.braun-odw.eu";
      };

      imports = [
        ./reineke-hardware.nix
      ];

      boot.loader.grub = {
        enable = true;
        version = 2;
        device = "/dev/sda";
      };

      time.timeZone = "UTC";

      networking = {
        hostName = "reineke";

        interfaces.ens3 = {
          useDHCP = true;

          ipv6.addresses = [{
            address = "2a01:4f8:1c1c:3385::2";
            prefixLength = 64;
          }];
        };

        defaultGateway6 = {
          address = "fe80::1";
          interface = "ens3";
        };
      };

      services.openssh.enable = true;

      # This value determines the NixOS release from which the default
      # settings for stateful data, like file locations and database versions
      # on your system were taken. It‘s perfectly fine and recommended to leave
      # this value at the release version of the first install of this system.
      # Before changing this value read the documentation for this option
      # (e.g. man configuration.nix or on https://nixos.org/nixos/options.html).
      system.stateVersion = "20.09"; # Did you read the comment?
    };
}

# TASK: Set `users.mutableUsers` to `false`:
#       https://nixos.org/manual/nixos/stable/index.html#sec-user-management
# TASK: Set up automatic upgrades:
#       https://nixos.org/manual/nixos/stable/index.html#sec-upgrading-automatic
