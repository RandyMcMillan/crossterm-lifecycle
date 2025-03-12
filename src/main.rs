// Cargo.toml
// [package]
// name = "crossterm-lifecycle"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// tokio = { version = "1.44.0", features = ["full"] }
// ratatui = "0.26"
// crossterm = "0.27"

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::spawn;

use env_logger::{Builder, Env};
use log::{debug, error, info, trace, warn};
use ratatui::{/*backend::Backend,*/ Terminal};
use std::{env, io};

async fn logger(arg1: &str, arg2: &str) {
    trace!("This is a trace message. \n{} {}\n", arg1, arg2);
    debug!("This is a debug message. \n{} {}\n", arg1, arg2);
    info!("This is a info message. \n{} {}\n", arg1, arg2);
    warn!("This is a warn message. \n{} {}\n", arg1, arg2);
    if arg2 != "pre" {
        error!("This is a error message. \n{} {}\n", arg1, arg2);
    }
}
async fn pre() -> Result<(), Box<dyn std::error::Error>> {
	println!("pre");
    let mut child = Command::new("echo")
        .arg("pre")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("stdout was not configured");
    let stderr = child.stderr.take().expect("stderr was not configured");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let stdout_handle = spawn(async move {
        let mut lines = stdout_reader.lines();
        while let Some(_line) = lines.next_line().await.unwrap_or(Some("0".to_string())) {
            logger("stdout: \n{}", &_line).await;
            //error!("stdout: \n{}", &_line);
        }
    });

    let stderr_handle = spawn(async move {
        let mut lines = stderr_reader.lines();
        while let Some(line) = lines.next_line().await.unwrap_or(Some("0".to_string())) {
            logger("stderr: \n{}", &line).await;
        }
    });

    let _status = child.wait().await?;

    stdout_handle.await?;
    stderr_handle.await?;

    //println!("Exited with status: {}", _status);

    let option_handle = spawn(async move {
        process_options(Some(10), Some("pre-hello".to_string())).await;
    });
    option_handle.await?;

    let option_handle = spawn(async move {
        process_options(Some(20), None).await;
    });
    option_handle.await?;

    let option_handle = spawn(async move {
        process_options(None, Some("pre-world".to_string())).await;
    });
    option_handle.await?;

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut args: Vec<String> = env::args().collect();

    let mut _pre_count = 0;
    let mut _mid_count = 0;
    let mut _post_count = 0;
    let mut output_count = 0;
    let mut output_files: Vec<String> = Vec::new();
    let mut verbose_count = 0;

    // Simulating command-line arguments (for testing)
    //#[cfg(debug_assertions)]
    //args.push("-v".to_string());
    //#[cfg(debug_assertions)]
    //args.push("-vv".to_string());
    //#[cfg(debug_assertions)]
    //args.push("-o".to_string());
    //#[cfg(debug_assertions)]
    //args.push("test1.txt".to_string());
    //#[cfg(debug_assertions)]
    //args.push("-o".to_string());
    //#[cfg(debug_assertions)]
    //args.push("test2.txt".to_string());
    #[cfg(debug_assertions)]
    args.push("extra_arg".to_string());
    args.push("-2".to_string());

    let mut i = 1; // Start from 1 to skip the executable path
    while i < args.len() {
        match args[i].as_str() {
            "-1" | "--pre" => {
                _pre_count += 1;
            }
            "-2" | "--mid" => {
                _mid_count += 1;
            }
            "-3" | "--post" => {
                _post_count += 1;
            }
            "-v" | "--verbose" => {
                verbose_count += 1;
            }
            "-vv" => {
                verbose_count += 2;
            }
            "--output" | "-o" => {
                i += 1;
                if i < args.len() {
                    output_files.push(args[i].clone());
                    output_count += 1;
                } else {
                    eprintln!("Error: Missing output file after --output");
                    std::process::exit(1);
                }
            }
            arg => {
                warn!("Argument: {}", arg);
            }
        }
        i += 1;
    }

    debug!("_precount: {}", _pre_count);
    debug!("_midcount: {}", _mid_count);
    debug!("_postcount: {}", _post_count);
    debug!("verbose_count: {}", verbose_count);
    debug!(
        "output_files: {:?} output_count: {} ",
        output_files, output_count
    );

    let mut builder = Builder::new();
    //use log::{debug, error, info, trace, warn};
    if verbose_count > 0 {
        if verbose_count == 1 {
            //warn

            builder = Builder::from_env(Env::default().default_filter_or(
            "warn,libp2p_gossipsub::behaviour=error,eframe=error,egui_glow=error,egui_winit=error,egui_extras=error,mio::poll=error",
        ))
        }
        if verbose_count == 2 {
            //info

            builder = Builder::from_env(Env::default().default_filter_or(
            "info,libp2p_gossipsub::behaviour=error,eframe=error,egui_glow=error,egui_winit=error,egui_extras=error,mio::poll=error",
        ))
        }
        if verbose_count == 3 {
            //error

            builder = Builder::from_env(Env::default().default_filter_or(
            "error,libp2p_gossipsub::behaviour=error,eframe=error,egui_glow=error,egui_winit=error,egui_extras=error,mio::poll=error",
        ))
        }
        if verbose_count == 4 {
            //debug

            builder = Builder::from_env(Env::default().default_filter_or(
            "debug,libp2p_gossipsub::behaviour=error,eframe=error,egui_glow=error,egui_winit=error,egui_extras=error,mio::poll=error",
        ))
        }
        if verbose_count > 4 {
            //trace

            builder = Builder::from_env(Env::default().default_filter_or(
            "trace,libp2p_gossipsub::behaviour=trace,eframe=trace,egui_glow=trace,egui_winit=trace,egui_extras=trace,mio::poll=error",
        ))
        }
    }
    builder.init();

    if _pre_count > 0 {
        let _do_it_handle = pre().await; //print before ratatui
                                         // Setup terminal
    }
    let backend = ratatui::backend::CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::cursor::MoveTo(0, 0)
    )?;
    terminal.clear()?;
    if _mid_count > 0 {
        let _do_it_handle = mid().await;
    }
    // Your ratatui drawing code here (e.g., terminal.draw(...))
    terminal.draw(|f| {
        use ratatui::{/*prelude::*,*/ widgets::*};
        let block = Block::default()
            .title(" crossterm lifecycle (alternate screen) ")
            .borders(Borders::ALL);
        f.render_widget(block, f.size());
    })?;

    //let do_it_handle = do_it().await;//prints below ratatui screen
    // Wait for a key press before exiting
    if crossterm::event::poll(std::time::Duration::from_millis(5000))? {
        crossterm::event::read()?;
    }

    // Restore terminal settings and exit alternate screen
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    )?;
    if _post_count > 0 {
        let _do_it_handle = post().await; //prints below ratatui screen after exit
    }

    Ok(())
}

async fn mid() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n  mid");
    let mut child = Command::new("ls")
        .arg("-l")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("stdout was not configured");
    let stderr = child.stderr.take().expect("stderr was not configured");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let stdout_handle = spawn(async move {
        let mut lines = stdout_reader.lines();
        while let Some(line) = lines.next_line().await.unwrap_or(Some("0".to_string())) {
            //spacing for ratatui border
            println!("  {}", line);
        }
    });

    let stderr_handle = spawn(async move {
        let mut lines = stderr_reader.lines();
        while let Some(line) = lines.next_line().await.unwrap_or(Some("0".to_string())) {
            eprintln!("stderr: {}", line);
        }
    });

    let _status = child.wait().await?;

    stdout_handle.await?;
    stderr_handle.await?;

    //println!("Exited with status: {}", _status);

    let option_handle = spawn(async move {
        process_options(Some(10), Some("mid-hello".to_string())).await;
    });
    option_handle.await?;

    let option_handle = spawn(async move {
        process_options(Some(20), None).await;
    });
    option_handle.await?;

    let option_handle = spawn(async move {
        process_options(None, Some("mid-world".to_string())).await;
    });
    option_handle.await?;

    Ok(())
}
async fn post() -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("echo")
        .arg("post")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("stdout was not configured");
    let stderr = child.stderr.take().expect("stderr was not configured");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let stdout_handle = spawn(async move {
        let mut lines = stdout_reader.lines();
        while let Some(line) = lines.next_line().await.unwrap_or(Some("0".to_string())) {
            println!("{}", line);
        }
    });

    let stderr_handle = spawn(async move {
        let mut lines = stderr_reader.lines();
        while let Some(line) = lines.next_line().await.unwrap_or(Some("0".to_string())) {
            eprintln!("stderr: {}", line);
        }
    });

    let _status = child.wait().await?;

    stdout_handle.await?;
    stderr_handle.await?;

    //println!("Exited with status: {}", _status);

    let option_handle = spawn(async move {
        process_options(Some(10), Some("post-hello".to_string())).await;
    });
    option_handle.await?;

    let option_handle = spawn(async move {
        process_options(Some(20), None).await;
    });
    option_handle.await?;

    let option_handle = spawn(async move {
        process_options(None, Some("post-world".to_string())).await;
    });
    option_handle.await?;

    Ok(())
}

#[allow(unused_assignments)]
async fn process_options(mut opt1: Option<i32>, mut opt2: Option<String>) {
    let mut o1 = opt1.clone();
    let mut o2 = opt2.clone();

    while o1.is_some() || o2.is_some() {
        match (o1.take(), o2.take()) {
            (Some(val1), Some(val2)) => {
                //prints twice - once for each option
                println!("  Both Some: {}, {}", val1, val2);
                // process both values
            }
            (Some(val1), None) => {
                println!("  Opt1 Some: {}", val1);
                // process opt1
            }
            (None, Some(val2)) => {
                println!("  Opt2 Some: {}", val2);
                // process opt2
            }
            (None, None) => {
                //should not happen since we already checked is_some() above.
            }
        }
        // o1 = opt1;
        // o2 = opt2;
        opt1 = None;
        opt2 = None;
    }
}
