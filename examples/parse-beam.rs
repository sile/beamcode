use clap::Parser;

#[derive(Parser)]
struct Args {
    beam_file_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let beam = beam_file::StandardBeamFile::from_file(&args.beam_file_path)?;
    let mut literal_table = None;
    for chunk in &beam.chunks {
        if let beam_file::chunk::StandardChunk::LitT(chunk) = chunk {
            literal_table = Some(beamop::LiteralTable::new(chunk)?);
            break;
        }
    }

    let literal_table =
        literal_table.ok_or_else(|| anyhow::anyhow!("missing a mandatory 'LitT' chunk"))?;
    for chunk in beam.chunks {
        if let beam_file::chunk::StandardChunk::Code(chunk) = chunk {
            beamop::parse_code_chunk(&chunk, &literal_table)?;
            return Ok(());
        }
    }
    anyhow::bail!("missing mandatory 'Code' chunk");
}
