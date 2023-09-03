# Mentoria Bot

This bot is responsible for assigning students to mentors / teachers using the `/schedule` slash command, and to take care of the backend work to make it happen.

## Table of Contents

- [Configuration](#configuration)
    - [Google API](#google-api)
    - [Database](#database)
- [Building & Running](#building--running)
- [Formatting](#formatting)
- [Compatibility](#compatibility)
- [Structure](#structure)
- [Functionality](#functionality)

## Configuration

Copy `config.example.json` to `config.json`, replacing your bot's token and main guild ID
(where slash commands will be registered).

### Google API

A Google Account is required to run the bot.

Add Google API OAuth2 secrets (`client-secret.json`) to the `secrets/` folder. Then, when the bot runs for the
first time, it will ask you to specify `MRB_AUTH=1` as an environment variable in order to log into
the Google Account representing the bot.

### Database

This project uses the Postgres database. Ensure it is properly installed (v15 recommended).

It is recommended to create a `.env` file with `DATABASE_URL` for the Postgres database URL (with username and password).
This should be the same `DATABASE_URL` as the `database_url` in `config.json`.
Optionally add `DATABASE_TEST_URL` as well for the test Postgres database URL (in order to be able to run database tests).
Note that migrations are automatically run for the test database.

Run `diesel migration run` to apply migrations (from the `migrations/` folder) to the main database, from `DATABASE_URL`.
Use `diesel migration redo --all` to **wipe the migrations** and re-apply them **(WARNING: Leads to loss of data)**.

## Building & Running

Use `cargo run` to build and run the bot. Use just `cargo check` if just checking if it would compile without running it.

### Using Nix

This repository exports a Nix flake. You can run the bot through Nix with `nix run .`

You can also run a full check (build, run clippy and check formatting) using `nix flake check`.

## Formatting

Use `cargo +nightly fmt --all` (installing Rust nightly is required for this command, but not for building/running the bot).

## Development

To develop this project, you need at least:
- A fairly recent Rust toolchain with `rustc` and `cargo` (see Compatibility below);
- `clippy`;
- Nightly toolchain's `rustfmt` for the formatting;
- `diesel-cli` to run the database migrations with the `diesel` command.
    - Can be installed through `cargo` with `cargo install diesel-cli`. Requires postgresql and some other drivers on your system.

### Using Nix

You can run `nix develop .` to spawn a dev shell with all the required tools to develop (mentioned above).

## Compatibility

- Tested with rustc v1.70.

## Structure

- The `crates/` folder contains the multiple Rust crates (modules) which make the bot work.
    - The `crates/bot/` folder contains the source of the binary crate (executable), which contains code for the bot's commands (in `commands/`), along with the entrypoint in `main.rs` and other dependencies (such as `authenticate.rs` and `config.rs`).
    - The `crates/lib/` folder contains most of the bot's non-command logic, including database code (`db/`), models (`model/`), Google API providers (`notification/`), general utilities (`util/`), and errors (`error.rs`).
    - The `crates/loadmentors/` folder contains a crate which is responsible for parsing CSV files with mentor information (obtained through Google Forms) and writing the obtained information to the database.
    - The `crates/forms/` folder contains the code used by the bot for Discord interaction forms, or data requests through several consecutive messages with interactions (such as buttons, select menus and modals).
    - The `crates/macros/` folder contains Rust procedural macros which are applied to structures meant to represent components in a Discord interaction form, and are meant to be used in conjunction with the code in `forms/`, as those macros generate code relevant to that module.
- The `locales/` folder contains translation strings used throughout the bot for user-facing messages. Currently, it only contains strings for two languages: English and Brazilian Portuguese.
- The `migrations/` folder contains database migration SQL files.
- The `secrets/` folder (not in the repository) is used for Google API secrets, as specified in [Configuration](#configuration).
- The `target/` folder (not in the repository) is where Rust will output the build results.

## Functionality

- **Commands:**
    - `/schedule` (PT-BR: `/marcar`): Executed by a student to schedule a session with a mentor.
        - This will add the session to the database (`Session` model), and requires a pre-existing `User` model
        for the student and a pre-existing `Teacher` (which requires `User`) model for the mentor.
        - This will automatically create a Google Calendar event, associated with a Google Meet call
        (with an invite sent to both the student and the mentor).
        - This will also send an e-mail to both the student and the mentor.
