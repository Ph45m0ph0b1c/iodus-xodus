# iOdus backend (iodus-xodus)

This repository is the backend of [iOdus](https://github.com/burghr/iOdus), a
native macOS front end for Xbox PC games. It is a fork of
[Xodus](https://github.com/xodus-gaming/xodus) with the backend patches of
[Xodus Nexus](https://github.com/domsvoboda/xodus-nexus) and a few iOdus changes.

The backend does all the work with the Microsoft account: sign-in, the library,
licenses, downloads, the decryption of the game executable, achievements, friends,
and messages. iOdus starts `xodus-cli` as a subprocess and reads its JSON output.
The account tokens stay in this program and in the macOS Keychain.

> **Unofficial project.** It is not affiliated with, endorsed by, or sponsored by
> Microsoft or Xbox. All trademarks belong to their owners. Use it at your own risk.

## Branches

| Branch | Content |
|---|---|
| `iodus` (default) | Xodus `53ea781` + the 76 Nexus backend patches + the iOdus changes |
| `main` | The Xodus upstream branch, unchanged |

The `iodus` branch has one commit for each Nexus patch, in the order of
[`nexus/backend-source.lock.json`](nexus/backend-source.lock.json). The patches
come from Xodus Nexus `develop` at `6100e3a`. See [`nexus/README.md`](nexus/README.md).

The README of Xodus upstream is in
[`docs/xodus-upstream-README.md`](docs/xodus-upstream-README.md).

## The iOdus changes

On top of Xodus and the Nexus patches, the `iodus` branch has these changes:

1. **`xodus-cli service`:** The account service is a library with `run()`. The
   `xodus-service` program and the new `xodus-cli service` command both call it.
   With the service in `xodus-cli`, one program creates and changes the Keychain
   items, so macOS does not ask for your password at each token change.
   - The service reads its socket path from `XODUS_SOCKET`.
   - It removes a stale socket file before it starts.
   - It prints `XODUS_SERVICE_READY` when it listens.
2. **Game arguments:** `xodus-cli run` passes the arguments after `--` to the game.
   iOdus uses this to start Unity games with `-window-mode borderless`, because
   D3DMetal crashes on a focus change in exclusive full screen.
3. **Project rule:** [`CLAUDE.md`](CLAUDE.md) has the rule "Xbox Live: real data
   only" for all work on this backend.

## Build

You need Rust 1.98 or later (the workspace sets `rust-version = "1.98.0"`),
`protobuf`, and `cmake`. On macOS:

```sh
brew install protobuf cmake
rustup update stable
cargo build --locked --release -p xodus-cli -p xodus-service
```

The programs are in `target/release/`. For iOdus, sign them with the local
signing identity after each build (`scripts/sign-backend.sh` in the iOdus
repository). A program with a new signature cannot use the Keychain items of the
old one, so sign out and sign in again in iOdus after the first signed build.

## The commands that iOdus uses

| Command | Use in iOdus |
|---|---|
| `account --json` | Is an account stored? |
| `login` | Opens the Microsoft sign-in window |
| `logout --device` | Removes the account and the device items |
| `library --json --market <region>` | The owned and Game Pass entitlements |
| `profile --json` | The gamertag, the gamerpic, and the gamerscore |
| `resolve-package <product> --json --market <region>` | The package of a game, its size, and its version |
| `streaming <product> <folder> --progress-json --control-stdin` | The download of a game, with progress lines and pause or resume on stdin |
| `run <folder> <wine> --exe <exe> --materialize-chain --market <region> [-- <game arguments>]` | Gets the license, decrypts the executable into a temporary launch view, and starts the game with Wine |
| `service` | The account service on `$XODUS_SOCKET`, for the Xbox runtime in Wine |
| `achievements <title id> --json` | The achievements of one title (read only) |
| `native-social friends` | The friends list |
| `native-social people` | Profiles and the title history (JSON request on stdin) |
| `native-social listen` | One process for the inbox, conversations, and sending messages (JSON lines on stdin and stdout) |

Run `xodus-cli --help` and `xodus-cli <command> --help` for all commands and options.

## Update the Nexus patches

1. Get the new Nexus `develop` commit and its `backend-source.lock.json`.
2. Start a new branch from the Xodus commit that the lock file names.
3. Apply each patch as one commit, in the order of the lock file.
4. Apply the iOdus commits again (the service command, the game arguments, and
   `CLAUDE.md`).
5. Copy the lock file to `nexus/`, and change `nexus/README.md`.
6. Build, and test iOdus with it, before the branch replaces `iodus`.

## Project rules

See [`CLAUDE.md`](CLAUDE.md). Code and tests never send made-up or test data to
Xbox Live or other Microsoft services. Reads are allowed. Tests of code that
writes use a fake server or injected answers.

## License

- **Xodus:** GPL-3.0. See [`LICENSE`](LICENSE).
- **Xodus Nexus patches:** MIT, in this GPL-3.0 work.
- **iOdus changes:** GPL-3.0, the same as the rest of this repository.

If you give someone a program that contains this backend (for example an iOdus
app bundle with `xodus-cli`), the GPL requires that they can get its source.
