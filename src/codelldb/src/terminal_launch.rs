// Windows terminal processes must inherit the attached console. LLVM's launch
// path uses CREATE_NO_WINDOW when all three standard handles are redirected,
// which makes CONIN$/CONOUT$ handles unusable in the child.
use crate::prelude::*;
use adapter_protocol::{Either, LaunchRequestArguments};
use lldb::*;
use std::fs::OpenOptions;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use winapi::um::winbase::CREATE_SUSPENDED;

pub fn launch(target: &SBTarget, info: &SBLaunchInfo, args: &LaunchRequestArguments) -> Result<SBProcess, Error> {
    let executable = info.executable_file();
    let executable = if executable.is_valid() { executable } else { target.executable() };
    let mut command = Command::new(executable.path());
    command.args(info.arguments());
    if let Some(cwd) = info.working_directory() {
        command.current_dir(cwd);
    }
    command.env_clear();
    let environment = info.environment();
    for index in 0..environment.num_entries() {
        command.env(environment.name_at_index(index).to_str()?, environment.value_at_index(index).to_str()?);
    }

    let paths = match &args.stdio {
        None => vec![],
        Some(Either::First(path)) => vec![Some(path.clone())],
        Some(Either::Second(paths)) => paths.clone(),
    };
    if paths.len() > 3 {
        bail!("Windows terminal launching supports stdin, stdout and stderr only.");
    }
    for fd in 0..3 {
        let path = paths.get(fd).and_then(Option::as_deref)
            .unwrap_or(if fd == 0 { "CONIN$" } else { "CONOUT$" });
        let console = path.eq_ignore_ascii_case("CONIN$") || path.eq_ignore_ascii_case("CONOUT$");
        let file = OpenOptions::new().read(fd == 0).write(fd != 0)
            .create(fd != 0 && !console).truncate(fd != 0 && !console).open(path)?;
        match fd {
            0 => { command.stdin(Stdio::from(file)); }
            1 => { command.stdout(Stdio::from(file)); }
            _ => { command.stderr(Stdio::from(file)); }
        }
    }

    // No application code runs before LLDB owns the process. Its Windows
    // thread resume operation releases the initial CREATE_SUSPENDED count.
    // Command handles Windows argument quoting and Unicode environments.
    let mut child = command.creation_flags(CREATE_SUSPENDED).spawn()?;
    let attach = SBAttachInfo::new();
    attach.set_process_id(child.id() as ProcessID);
    attach.set_wait_for_launch(false, false);
    match target.attach(&attach) {
        Ok(process) => Ok(process),
        Err(error) => {
            log_errors!(child.kill());
            log_errors!(child.wait());
            Err(Box::new(error).into())
        }
    }
}
