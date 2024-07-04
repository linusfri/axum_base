{ pkgs, lib, config, inputs, ... }:
let
  name = "axumBase";
in
{
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

  processes = {
    "${name}".exec = "cargo watch -x run";
  };

  services.linusfri.rustServer = {
    enable = true;
    domains = [ "axum.local" ];
  };

  services.linusfri.mysql = {
    enable = true;
  };
}
