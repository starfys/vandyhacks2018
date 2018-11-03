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
      };
      # virtualHosts."server.example.com" = {
      #   forceSSL = true;
      #   useACMEHost = "example.com";
      #   locations."/" = {
      #     proxyPass = "http://localhost:8888";
      #   };
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
    ];
    users.mutableUsers = false;
    users.users.api = {
      createHome = true;
      description = "Used for API daemon";
      hashedPassword = "$6$rounds=1000000$HDzFLWCTA$knAEZzKKpAh0HBwFjA9bm/dolrOGibyJxeykHCtnKzvR1qKoQdSdnY6AzZYABinPP9WHNi2ApcsucDbTM7cV41";
    };
    # Deployment details
    deployment.targetEnv = "digitalOcean";
    deployment.digitalOcean.enableIpv6 = true;
    deployment.digitalOcean.region = "nyc3";
    deployment.digitalOcean.size = "s-4vcpu-8gb";

  };
}
