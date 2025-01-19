mod daily_fears;
use std::io;

fn main() -> io::Result<()> {
    match daily_fears::get_daily_fears() {
        Ok(fears) => {
            println!("\nThanks for sharing your fears. Here they are:");
            daily_fears::write_fears_to_md(&fears)?;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
