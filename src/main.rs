use std::io::{Write, BufWriter};

use serde_query::Deserialize;

#[derive(Debug, Deserialize)]
struct Commits {
    #[query(".[].sha")]
    shas: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("./commits.json")?;
    let commits: Commits = serde_json::from_str(&input)?;

    let stdout = std::io::stdout().lock();
    let mut writer = BufWriter::new(stdout);
    for sha in commits.shas.iter() {
        writer.write_all(sha.as_bytes())?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;

    Ok(())
}
