//! The Xodus account service. The `xodus-service` binary and the
//! `xodus-cli service` command both call `run`.
//!
//! iOdus runs the service as `xodus-cli service`, so that one program creates
//! and changes the Keychain items. On macOS, a second program must ask the
//! user for the password each time it changes an item of another program.

use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;

use tokio::net::UnixListener;
use tokio_util::sync::CancellationToken;
use xodus::tokens::TokenManager;

mod account_metadata;
mod connection;
mod simple_context;
mod store_license;
mod store_updates;
mod utils;

const XML_MAGIC: u32 = 0x58445358;
const PROTO_MAGIC: u32 = 0x58445350;

/// Runs the service until Ctrl+C or SIGINT. The socket path is
/// `$XODUS_SOCKET`, or `xodus.sock` in the runtime folder.
/// The caller sets up tracing. Returns the process exit code.
pub async fn run() -> i32 {
    if let Err(error) = xodus::secrets::init_secrets() {
        xodus::tokens::store::TokenStoreError::from(error).report();
        return 1;
    }
    let tokens = Arc::new(TokenManager::with_keychain_and_memory());
    if let Err(error) =
        xodus::tokens::device::ensure_device_credentials(&reqwest::Client::new(), &tokens).await
    {
        error.report();
        return 1;
    }
    let xodus::models::secrets::Token::Legacy(device_token) = (match tokens.get_device_sts_token() {
        Ok(token) => token,
        Err(error) => {
            error.report();
            return 1;
        }
    }) else {
        xodus::tokens::store::TokenStoreError::DeviceAuthentication.report();
        return 1;
    };

    let socket_path = match std::env::var("XODUS_SOCKET") {
        Ok(path) if !path.is_empty() => path,
        _ => format!("{}/xodus.sock", utils::get_runtime_dir()),
    };
    let cancellation = CancellationToken::new();
    let trigger = cancellation.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failure to handle ctrl_c");
        trigger.cancel();
    });
    {
        // A socket file from a stopped service blocks the bind. Remove only a socket.
        if let Ok(metadata) = std::fs::symlink_metadata(&socket_path) {
            use std::os::unix::fs::FileTypeExt;
            if metadata.file_type().is_socket() {
                _ = std::fs::remove_file(&socket_path);
            }
        }
        let listener = match UnixListener::bind(&socket_path) {
            Ok(listener) => listener,
            Err(error) => {
                eprintln!("Unable to bind to socket {socket_path}: {error}");
                return 1;
            }
        };
        let perms = Permissions::from_mode(0o600);
        _ = tokio::fs::set_permissions(&socket_path, perms).await;
        // A parent process waits for this line before it starts a game.
        println!("XODUS_SERVICE_READY {socket_path}");
        loop {
            let accept = tokio::select! {
                r = listener.accept() => r,
                _ = cancellation.cancelled() => break,
            };
            let Ok(accept) = accept else { continue };

            let token = cancellation.clone();
            let device_token = device_token.clone();
            let tokens = tokens.clone();
            tokio::spawn(async move {
                connection::router::route(accept.0, token, device_token, tokens).await
            });
        }
    }

    _ = tokio::fs::remove_file(socket_path).await;
    0
}
