{ pkgs, lib, config, inputs, ... }:
let
  variables = import ./config/variables.nix;

  numberOfHosts = builtins.length variables.domains;
  certificateName = if numberOfHosts < 2 then variables.mainDomain else "${variables.mainDomain}+${builtins.toString (numberOfHosts - 1)}";
	serverName = lib.strings.concatMapStringsSep " " (domain: domain) variables.domains;
in
{
  certificates = variables.domains;
  hosts = builtins.listToAttrs (map(domain: {name = domain; value = "${variables.host}";}) variables.domains);

  env = {
    inherit (variables) DB_CONNECTION_STRING;
    inherit (variables) ADDRESS_AND_PORT;
    inherit (variables) DATABASE_URL;
  };

  packages = [
    pkgs.openssl
    pkgs.cargo-watch
  ];

  languages = {
    rust = {
      enable = true;
      channel = "stable";
      components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
    };
  };

  scripts.mysql-local.exec = with variables; "mysql -u '${dbUser}' --password='${dbPassword}' -h '${dbHost}' '${dbName}' \"$@\"";
  scripts.migrate.exec = "sqlx migrate run";

  processes = {
    axumBase.exec = "cargo watch -x run";
  };

  services.nginx = {
    enable = lib.mkDefault true;
    httpConfig = lib.mkDefault ''
      server {
        listen ${toString variables.nginxPort};
        listen ${toString variables.nginxSslPort} ssl;
        ssl_certificate     ${config.env.DEVENV_STATE}/mkcert/${certificateName}.pem;
        ssl_certificate_key ${config.env.DEVENV_STATE}/mkcert/${certificateName}-key.pem;
        # ssl_protocols       TLSv1 TLSv1.1 TLSv1.2 TLSv1.3;
        # ssl_ciphers         HIGH:!aNULL:!MD5;

        root ${config.env.DEVENV_ROOT}/src;
        index index.html index.htm;
        server_name ${serverName};

        error_page 497 https://$server_name:$server_port$request_uri;

        location / {
          proxy_pass "http://${variables.host}:${variables.serverListenPort}";
        }
      }
    '';
  };
  
  services.mysql = {
    enable = true;
    package = pkgs.mysql80;
    initialDatabases = lib.mkDefault [
      { name = variables.dbName; }
      # { name = mysql_test_database; }
    ];
    settings.mysql.port = variables.dbPort;
    settings.mysqld.log_bin_trust_function_creators = 1;
    ensureUsers = lib.mkDefault [
      {
        name = variables.dbUser;
        password = variables.dbPassword;
        ensurePermissions = {
          "${variables.dbName}.*" = "ALL PRIVILEGES";
          # "${mysql_test_database}.*" = "ALL PRIVILEGES";
        };
      }
    ];
  };
}
