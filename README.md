# sqlx example

**warning**: this is not yet in a usable state. The `cli` crate works fine, but the pre-commit check fails due to not being able to access a database

This is a small example of using nix to manage sqlx in a project.
The main benefit of the current configuration is the use of a custom pre-commit hook inside the nix devShell.
The hook runs `cargo sqlx prepare --merged` which will ensure that a commit cannot be made without updating sqlx's `sqlx-data.json`.
The hook could be modified with the `--check` flag if running the command to overwrite files manually is desired.
