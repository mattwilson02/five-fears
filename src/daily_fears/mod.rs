use chrono::Local;
use std::fs::File;
use std::io::{self, Write};

pub fn get_daily_fears() -> io::Result<Vec<String>> {
    let mut fears: Vec<String> = Vec::new();
    println!("Please share 5 fears you're currently experiencing.");
    println!("Press Enter after typing each fear.\n");

    for i in 1..=5 {
        print!("Fear #{}: ", i);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let fear: String = input.trim().to_string();
        fears.push(fear);
    }

    Ok(fears)
}

pub fn write_fears_to_md(fears: &[String]) -> io::Result<()> {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("fears_output/fears_{}.md", date);

    let mut file = File::create(&filename)?;

    writeln!(file, "# Fear Journal - {}\n", date)?;
    writeln!(file, "## Today's Fears:\n")?;

    for (i, fear) in fears.iter().enumerate() {
        writeln!(file, "{}. {}", i + 1, fear)?;
    }

    writeln!(
        file,
        "\n---\nJournal entry created on {}",
        Local::now().format("%B %d, %Y at %H:%M")
    )?;

    println!("\nFears have been written to {}", filename);
    Ok(())
}
