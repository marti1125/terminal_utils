use std::env;
use std::process::Command;

fn main() {

    for argument in env::args() {

        // To list any process listening to the port
        let open_port = &argument[..10];

        if open_port == "open_port=" {
            let port = &argument[10..argument.len()];
            let cmd = "-i:";

            Command::new("lsof")
                .args(&[cmd.to_owned() + port])
                .spawn()
                .expect("failed to execute process");
        }

    }

}
