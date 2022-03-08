use std::process::Command;

pub fn run(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").arg(name).spawn()
    } else {
        Command::new("sh").arg("-c").arg(name).spawn()
    };

    if !output.is_err() {
        return Ok(());
    } else {
       return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", output),
        )));
    }
}
