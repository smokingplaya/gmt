use std::{io::{BufRead, BufReader}, os::windows::process::CommandExt, process::{Child, Command, Stdio}};
use crate::log::{self, LogType};

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn process_handler(mut cmd: Child) {
    if let Some(stdout) = cmd.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(err) => eprintln!("unable to read line: {}", err),
            }
        }
    }

    let _status = cmd.wait().expect("Не удалось дождаться завершения команды");
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn start(path: String, args: Vec<String>) {
    let cmd = Command::new(path)
        .args(&args)
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match cmd {
        Ok(proc) => process_handler(proc),
        Err(err) => log::print(LogType::Error, &err.to_string()),
    }

    /*
     * хахах блять, оно работает ахуеть
     * TODO: CTextConsoleWin32::GetLine: !GetNumberOfConsoleInputEvents
     * если TODO пофиксить, то все будет работать как нужно хаха
     !(реализовано) но плюсом нужно как-то в рантайме все писать
     */
    //for line in cmd.stdout.lines() {
    //    println!("{}", line.unwrap());
    //}
}