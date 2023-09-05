# NixOS module to run mentoriabot.
flake:

{ config, pkgs, lib, ... }:
let
  cfg = config.services.mentoriabot;
  absPath = with lib.types; either path (strMatching "/[^\n:]+");
  botPackage = cfg.package;
  workdir = cfg.workdir;
  mkDisableOption = desc: (lib.mkEnableOption desc) // { default = true; example = false; };

  hostSystem = pkgs.stdenv.hostPlatform.system;
  # mentoriabot = flake.packages.${hostSystem}.default;
in
{
  options.services.mentoriabot = {
    enable = lib.mkEnableOption "Enable the mentoriabot service. The bot will run upon startup.";
    workdir = lib.mkOption {
      type = absPath;
      description = ''
        The working directory the bot should start at. It should contain at least a 'secrets/' folder and a config.json file.
        The 'secrets/' folder must already contain (besides the OAuth client secret) the auth token for the bot to log in.
        You may obtain this by running the bot's binary with MRB_AUTH=1 and logging into the bot's Google Account with
        a web browser.
      '';
      example = "/home/user/botfolder";
    };
    package = lib.mkOption {
      type = lib.types.package;
      description = "The mentoriabot package to use. Defaults to the one built with the source flake.";
      default = pkgs.hello;
    };
    protect = mkDisableOption "Enables sandboxing and general security measures for the bot.";
  };

  config = lib.mkIf cfg.enable {
    systemd.services.mentoriabot = {
      after = [ "network-online.target" ]; # requires a network connection

      wantedBy = [ "multi-user.target" ]; # starts on boot (becomes a boot dependency)

      description = "A mentorship management bot.";

      # don't restart if it restarts 10 times over 20 seconds
      startLimitIntervalSec = 20;
      startLimitBurst = 10;

      serviceConfig = {
        ExecStart = "${lib.getExe botPackage}";
        Type = "simple"; # just runs to completion

        Restart = "on-failure"; # restart on exit code 1 or segfault or whatever
        WorkingDirectory = workdir;
      } // lib.optionalAttrs cfg.protect {
        # only enable these options if 'services.mentoriabot.protect == true'
        RuntimeDirectory = "mentoriabot"; # create a /run/ directory for temp stuff
        RuntimeDirectoryMode = "0755";
        BindReadOnlyPaths = [
          "${workdir}:/run/mentoriabot/workdir"
        ];
        WorkingDirectory = "/run/mentoriabot/workdir";

        # security options
        DynamicUser = "yes"; # don't run as root
        NoNewPrivileges = "yes"; # child processes won't be able to... well we don't even have those
        LockPersonality = "yes"; # we don't need whatever that is
        PrivateTmp = "yes"; # remove access to /tmp
        PrivateDevices = "yes"; # remove access to devices

        ProtectHome = "yes"; # forbid home access
        ProtectSystem = "strict"; # read-only FS
        ProtectClock = "yes"; # read-only clock
        ProtectHostname = "yes"; # not like we need that info
        ProtectKernelLogs = "yes"; # why would we access that
        ProtectKernelTunables = "yes"; # why would we need kernel settings
        ProtectProc = "invisible"; # we don't need to see other processes

        RestrictNamespaces = "yes"; # debatable
        RestrictRealtime = "yes"; # some cpu scheduling stuff we don't need
        RestrictSUIDSGID = "yes"; # we don't install things
        SystemCallArchitectures = "native"; # we don't need to use other architectures' instructions
      };
    };
  };
}
