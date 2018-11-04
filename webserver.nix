{
  # Import config to deploy to digital ocean
  
  resources.sshKeyPairs.ssh-key = {};
  webserver = { config, pkgs, ... }: {
    # ACME config
    # security.acme.certs = {
      # "example.com" = {
      #   email = "@fooexample.com";
      #   extraDomains = {};
      # };
    # };
    # Configure services
    services.nginx = {
      enable = true;
      virtualHosts."example.com" = {
      #  forceSSL = true;
      #  enableACME = true;
        locations."/" = {
          root = "/var/www";
        };
        locations."/api" = {
          proxyPass = "http://localhost:8000";
        };
      };
    };
    # SSHD
    services.openssh = {
      enable = true;
    };
    # Postgresql database
    services.postgresql = {
      enable = true;
    };
    # Manually allow port for nginx
    networking.firewall.allowedTCPPorts = [ 80 443 ];
    # Add rxvt terminfo
    environment.systemPackages = with pkgs; [
      # Add urxvt terminfo so ssh doesn't have issues
      rxvt_unicode.terminfo
      # Persistence
      tmux
      # Python
      (pkgs.python3.withPackages (ps: with ps; [virtualenv pip]))
      # Build tools
      binutils.bintools
      gcc
      gnumake
      pkgconfig
    ];
    users.mutableUsers = false;
    users.users.api = {
      description = "Used for API daemon";
      isNormalUser = true;
      createHome = true;
      hashedPassword = "!";
    };
    # Deployment details
    deployment.targetEnv = "digitalOcean";
    deployment.digitalOcean.enableIpv6 = true;
    deployment.digitalOcean.region = "nyc3";
    deployment.digitalOcean.size = "s-4vcpu-8gb";

  };
}
