use std::env;
use std::process::Command;

fn main() {

    for argument in env::args() {

        if argument.len() >= 10 {

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

        if argument.len() >= 15 {
            
            // Kill process in the port
            let open_port_kill = &argument[..15];

            if open_port_kill == "open_port_kill=" {

                let port = &argument[15..argument.len()];

                let output = Command::new("lsof")
                    .args(&["-t", &format!("-i:{port}", port=port), "-sTCP:LISTEN"])
                    .output()
                    .expect("failed to execute process");

                let mut pid = String::from_utf8_lossy(&output.stdout);

                println!("{:?}", &pid[0..5]);

                let mut lines = pid.lines();

                println!("{:?}", lines.next());

                Command::new("kill")
                    .args(&["-9", &pid[0..5]])
                    .spawn()
                    .expect("failed to execute process");
            }

        }

    }

}
