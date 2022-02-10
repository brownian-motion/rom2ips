use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    in_file: String,

    #[clap(short, long)]
    out_file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let rom_bytes = fs::read(args.in_file)?;
    let rom_size = rom_bytes.len();
    let patch = ips::Patch::new(
        vec![ips::Hunk::new(0, rom_bytes)],
        Some(rom_size),
    );
    fs::write(args.out_file, patch.ips_file_bytes())?;

    Ok(())
}
