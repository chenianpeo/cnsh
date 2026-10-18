use std::process::{Child, ChildStdout, Command, Stdio};

use crate::{error::Error, parser::Pipeline};

pub trait Executor {
    fn exec(&self) -> Result<i32, Error>;
}

impl Executor for Pipeline {
    fn exec(&self) -> Result<i32, Error> {
        if self.commands.is_empty() {
            return Ok(0);
        }

        // init procedure data
        let mut children: Vec<Child> = Vec::new();
        let mut pre_stdout: Option<ChildStdout> = None;
        let commands_len = self.commands.len();

        for (index, command) in self.commands.iter().enumerate() {
            let is_last = index + 1 == commands_len;

            let mut process = Command::new(&command.procedure);
            process.args(&command.arguments);

            // obtain last command output connect current command
            if let Some(stdout) = pre_stdout.take() {
                process.stdin(Stdio::from(stdout));
            }

            // create pipeline output
            if !is_last {
                process.stdout(Stdio::piped());
            }

            // running process, clean all process running if have one not success running
            let mut child = match process.spawn() {
                Ok(child) => child,
                Err(error) => {
                    for running in &mut children {
                        let _ = running.kill();
                    }

                    for running in &mut children {
                        let _ = running.wait();
                    }

                    return Err(Error::Io(error));
                }
            };

            // save current process std out
            pre_stdout = if is_last { None } else { child.stdout.take() };

            children.push(child);
        }

        let mut last_status = 0;

        for (index, child) in children.iter_mut().enumerate() {
            // wait total child process running end
            let status = child.wait()?;

            if index + 1 == commands_len {
                last_status = status.code().unwrap_or(1);
            }
        }

        Ok(last_status)
    }
}
