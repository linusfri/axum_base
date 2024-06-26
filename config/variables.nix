let
  domains = [
    "axum.local"
  ];

  mainDomain = builtins.elemAt domains 0;

  host = "127.0.0.1";

  dbPassword = "admin";
  dbUser = "user";
  dbPort = "3306";
  dbName = "db";
  dbHost = host;
  dbProtocol = "mysql";

  serverListenPort = "8080";

  nginxPort = "8000";
  nginxSslPort = "4430";
in
{
  inherit
    domains
    mainDomain
    host
    dbPassword
    dbUser
    dbPort
    dbName
    dbHost
    serverListenPort
    nginxPort
    nginxSslPort;

  DB_CONNECTION_STRING="${dbProtocol}://${dbUser}:${dbPassword}@${dbHost}:${dbPort}/${dbName}";
  DATABASE_URL="${dbProtocol}://${dbUser}:${dbPassword}@${dbHost}:${dbPort}/${dbName}"; # REQUIRED FOR SQLX
  ADDRESS_AND_PORT="${host}:${serverListenPort}";
}