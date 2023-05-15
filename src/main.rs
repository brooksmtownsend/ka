use clap::Parser;
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
/// A simple CLI tool to kill processes by name
///
/// Example usage: ka nats
/// Multiple things: ka wasmcloud nats wash
struct Args {
    /// Name to search for in the process list, both process name and exe path
    #[arg(name = "name")]
    names: Vec<String>,

    /// Dry run, don't actually kill anything just output the processes running
    #[arg(short, long)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();

    let mut sys = System::new_all();
    sys.refresh_all();

    let all_processes = sys.processes();

    let names = args.names;

    let killz = all_processes
        .iter()
        .filter_map(|(pid, process)| {
            names
                .iter()
                .find(|name| process_matches(process, name))
                .map(|_name| {
                    if args.dry_run {
                        eprintln!(
                            "[{}] {} {}",
                            process.pid().to_string(),
                            process.name(),
                            process.exe().to_string_lossy()
                        );
                        None
                    } else {
                        eprintln!("Killing process: {:?}", process);
                        Some(kill_process(pid))
                    }
                })
        })
        .collect::<Vec<_>>();

    for kill in killz {
        match kill {
            Some(Err(e)) => eprintln!("Failed to kill process: {:?}", e),
            _ => (),
        }
    }
}

fn process_matches(process: &Process, name: &str) -> bool {
    process.name() == name
        || process
            .exe()
            .to_str()
            .expect("path to be valid unicode")
            .contains(name)
}

fn kill_process(pid: &Pid) -> std::io::Result<std::process::Child> {
    std::process::Command::new("kill")
        .arg("-9")
        .arg(pid.to_string())
        .spawn()
}
