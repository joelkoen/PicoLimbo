use std::process::Stdio;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync::{oneshot, Mutex};
use tracing::{debug, error, info};

pub struct ServerManager {
    start_command: String,
    process: Arc<Mutex<Option<tokio::process::Child>>>,
    child_stdin: Arc<Mutex<Option<tokio::process::ChildStdin>>>,
    done: Arc<AtomicBool>,
    stop_cancel: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl ServerManager {
    const DONE_MESSAGE: &'static str = ")! For help, type ";
    const STOP_COMMAND: &'static [u8; 5] = b"stop\n";

    pub fn new(start_command: String) -> Self {
        Self {
            start_command,
            process: Arc::new(Mutex::new(None)),
            child_stdin: Arc::new(Mutex::new(None)),
            done: Arc::new(AtomicBool::new(false)),
            stop_cancel: Arc::new(Mutex::new(None)),
        }
    }

    /// Starts the server asynchronously.
    ///
    /// Spawns a background task that:
    /// - Reads stdout line-by-line, setting `done` to true if a line contains "Done".
    /// - Waits for the process to exit and then resets the `done` flag.
    pub async fn start_server(&mut self) {
        // Check if the server is already running.
        if self.process.lock().await.is_some() {
            error!("Server is already running!");
            return;
        }

        // Split the start_command into program and arguments.
        let mut parts = self.start_command.split_whitespace();
        let program = match parts.next() {
            Some(p) => p,
            None => {
                error!("No command provided to start the server.");
                return;
            }
        };
        let args: Vec<&str> = parts.collect();

        // Spawn the process with piped stdout and stdin.
        let mut child = match Command::new(program)
            .args(&args)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                error!("Failed to start server: {}", e);
                return;
            }
        };

        let child_stdout = child.stdout.take();
        let child_stdin = child.stdin.take();

        {
            let mut proc_lock = self.process.lock().await;
            *proc_lock = Some(child);
            let mut stdin_lock = self.child_stdin.lock().await;
            *stdin_lock = child_stdin;
        }

        // Clone Arcs for use in the background task.
        let process_arc = self.process.clone();
        let done_arc = self.done.clone();

        // Spawn an async task to monitor the process.
        tokio::spawn(async move {
            if let Some(stdout) = child_stdout {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    println!("{line}");
                    if line.contains(Self::DONE_MESSAGE) {
                        done_arc.store(true, Ordering::SeqCst);
                    }
                }
            }

            // Wait for the process to exit.
            if let Some(mut child_proc) = process_arc.lock().await.take() {
                match child_proc.wait().await {
                    Ok(status) => {
                        debug!("Process exited with status: {:?}", status);
                        info!("Server entering sleep state.");
                    }
                    Err(e) => error!("Error waiting for process: {:?}", e),
                }
            }

            // Reset the done flag once the process is no longer running.
            done_arc.store(false, Ordering::SeqCst);
        });
    }

    /// Schedules a stop command to be sent after the given delay.
    /// A pending delayed stop can be canceled by calling `cancel_stop`.
    pub async fn schedule_stop(&self, delay: Duration) {
        // Create a oneshot channel to enable cancellation.
        let (cancel_tx, cancel_rx) = oneshot::channel();
        {
            let mut cancel_lock = self.stop_cancel.lock().await;
            *cancel_lock = Some(cancel_tx);
        }

        info!(
            "The server is scheduled to stop in {} seconds",
            delay.as_secs()
        );
        let stop_cancel_clone = self.stop_cancel.clone();
        let child_stdin = self.child_stdin.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = tokio::time::sleep(delay) => {
                    Self::stop_now(child_stdin).await;
                }
                _ = cancel_rx => {
                    info!("Delayed stop command cancelled");
                }
            }
            // Clear the cancellation token.
            let mut cancel_lock = stop_cancel_clone.lock().await;
            *cancel_lock = None;
        });
    }

    /// Cancels any pending delayed stop command.
    pub async fn cancel_stop(&self) {
        let mut lock = self.stop_cancel.lock().await;
        if let Some(sender) = lock.take() {
            let _ = sender.send(());
        }
    }

    /// Immediately sends the "stop" command to the server process via its stdin.
    async fn stop_now(child_stdin: Arc<Mutex<Option<tokio::process::ChildStdin>>>) {
        let mut lock = child_stdin.lock().await;
        if let Some(stdin) = lock.as_mut() {
            info!("Shutting down backend server");
            if let Err(e) = stdin.write_all(ServerManager::STOP_COMMAND).await {
                error!("Failed to write to server stdin: {:?}", e);
            }
            if let Err(e) = stdin.flush().await {
                error!("Failed to flush server stdin: {:?}", e);
            }
        } else {
            error!("Server is not running or stdin is not available.");
        }
    }

    /// Returns the current server status.
    pub async fn get_server_status(&self) -> ServerStatus {
        let proc_guard = self.process.lock().await;
        if proc_guard.is_none() {
            ServerStatus::Offline
        } else if self.done.load(Ordering::SeqCst) {
            ServerStatus::Online
        } else {
            ServerStatus::Starting
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum ServerStatus {
    Offline,
    Starting,
    Online,
}
