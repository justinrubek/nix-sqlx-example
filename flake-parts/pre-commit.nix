{
  inputs,
  self,
  ...
}: {
  perSystem = {
    lib,
    pkgs,
    self',
    ...
  }: {
    pre-commit = {
      check.enable = true;

      settings = {
        src = ../.;
        hooks = {
          alejandra.enable = true;
          rustfmt.enable = true;

          # sqlx check custom hook
          sql-prepare = let
            wrapper = pkgs.symlinkJoin {
              name = "sqlx-prepare-wrapper";
              paths = [self'.packages.rust-toolchain pkgs.sqlx-cli];
              nativeBuildInputs = [pkgs.makeWrapper];
              postBuild = ''
                wrapProgram $out/bin/cargo-sqlx \
                  --prefix PATH: ${pkgs.lib.makeBinPath [self'.packages.rust-toolchain]}
                  --set SQLX_OFFLINE true
              '';
            };
          in {
            enable = false; # only run in the devShell
            entry = "${wrapper}/bin/cargo-sqlx prepare --merged";
            # add `--check` to check only. Without it the file will be updated when the hook is run
            # entry = "cargo sqlx prepare --merged --check";
            pass_filenames = false;
          };
        };
      };
    };
  };
}
