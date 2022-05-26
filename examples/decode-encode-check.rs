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
            let ops = beamop::decode_ops(&chunk.bytecode)?;
            let encoded = beamop::encode_ops(&ops)?;
            assert_eq!(encoded, chunk.bytecode);
            return Ok(());
        }
    }
    anyhow::bail!("missing mandatory 'Code' chunk");
}
