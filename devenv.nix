{ pkgs, lib, config, inputs, ... }:
let
  name = "axumBase";
in
{
  env = {
    OPENSSL_DIR = "${config.env.DEVENV_PROFILE}";
  };

  packages = [
    pkgs.openssl.dev
    pkgs.cargo-watch
    pkgs.sqlx-cli
    pkgs.pkg-config
  ];

  languages = {
    rust = {
      enable = true;
      channel = "stable";
      components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
    };
  };

  processes = {
    "${name}".exec = "cargo watch -x run";
  };

  services.linusfri.rustServer = {
    enable = true;
    domains = [ "axum.local" ];
  };

  services.linusfri.mysql = {
    enable = true;
    dbName = "axum_base";
  };
}
