# sqlx example


This is a small example of using nix to manage sqlx in a project.
The main benefit of the current configuration is the use of a custom pre-commit hook inside the nix devShell.
The hook runs `cargo sqlx prepare --merged` which will ensure that a commit cannot be made without updating sqlx's `sqlx-data.json`.
The hook is disabled during `nix flake check` runs, so it will not cause issues with a sandbox.
Additionally a `build.rs` script is included to set environment variables for docs.rs builds.
