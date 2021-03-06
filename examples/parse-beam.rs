use clap::Parser;

#[derive(Parser)]
struct Args {
    beam_file_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let beam = beam_file::StandardBeamFile::from_file(&args.beam_file_path)?;
    for chunk in beam.chunks {
        if let beam_file::chunk::StandardChunk::Code(chunk) = chunk {
            for instruction in beamcode::decode_instructions(&chunk.bytecode)? {
                println!("{:?}", instruction);
            }
            return Ok(());
        }
    }
    anyhow::bail!("missing mandatory 'Code' chunk");
}
