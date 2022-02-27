use std::process::Command;

pub fn setup() {
    // Checking and installing dependencies
    println!("Checking dependencies..");

    //If os is windows
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args(["/C", "ipfs", "version"])
            .output()
            .expect("failed to execute process");

        //If not installed, install it
        if output.status.code().unwrap() == 1 {
            println!("Installing dependencies..");
            let output = Command::new("sh")
                .arg("-c")
                .arg("choco install ipfs")
                .output()
                .expect("failed to execute process");
            if output.status.code().unwrap() == 1 {
                println!("{:?}", output);
                panic!("Choco not installed install it via https://chocolatey.org/install \n Or install ipfs manually from https://docs.ipfs.io/install/command-line/#official-distributions");
            }
        }

    //If os is linux/anything else
    } else {
        let output = Command::new("sh")
            .arg("-c")
            .arg("ipfs version")
            .output()
            .expect("failed to execute process");

        //If not installed, install it
        if output.status.code().unwrap() == 1 {
            panic!("ipfs not installed install it via https://docs.ipfs.io/install/command-line/#official-distributions");
        }
    };

    //mark setup as done
    println!("Setup done");
}
