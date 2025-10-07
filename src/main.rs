use std::io::{self, Read};
use std::thread;
use std::time::Duration;
use std::error::Error;

use copypasta::{ClipboardContext, ClipboardProvider};

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();
    io::stdin()
	.read_to_string(&mut contents)
	.map_err(|e| format!("couldn't read from stdin: {e}"))?;

    let mut ctx = ClipboardContext::new()
	.map_err(|e| format!("clipboard error: {e}"))?;
    ctx.set_contents(contents.clone())
	.map_err(|e| format!("clipboard copying error: {e}"))?;

    thread::sleep(Duration::from_secs(3));
    
    Ok(())
}
