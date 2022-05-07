use clap::Parser;

#[derive(Parser)]
struct Args {
    beam_file_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let beam = beam_file::StandardBeamFile::from_file(&args.beam_file_path)?;
    for chunk in beam.chunks {
        if let Some(beam_file::chunk::CodeChunk(chunk)) = chunk {
            beamop::parse_code_chunk(&chunk)?;
            return Ok(());
        }
    }
    anyhow::bail!("missing mandatory 'Code' chunk");
}
