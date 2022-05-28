use beamop::instruction::Instruction;
use beamop::Decode;
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
            let instructions = beamop::decode_instructions(&chunk.bytecode)?;
            let mut reader = &chunk.bytecode[..];
            for (i, instruction) in instructions.into_iter().enumerate() {
                let start = chunk.bytecode.len() - reader.len();
                let _ = Instruction::decode(&mut reader)?;
                let end = chunk.bytecode.len() - reader.len();
                let expected = &chunk.bytecode[start..end];

                let encoded = beamop::encode_instructions(&[instruction.clone()])?;
                assert_eq!(encoded, expected, "[{}] {:?}", i, instruction);
            }
            return Ok(());
        }
    }
    anyhow::bail!("missing mandatory 'Code' chunk");
}
