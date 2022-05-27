use beamop::op::Op;
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
            let ops = beamop::decode_ops(&chunk.bytecode)?;
            let mut reader = &chunk.bytecode[..];
            for (i, op) in ops.into_iter().enumerate() {
                let start = chunk.bytecode.len() - reader.len();
                let _ = Op::decode(&mut reader)?;
                let end = chunk.bytecode.len() - reader.len();
                let expected = &chunk.bytecode[start..end];

                let encoded = beamop::encode_ops(&[op.clone()])?;
                assert_eq!(encoded, expected, "[{}] {:?}", i, op);
            }
            return Ok(());
        }
    }
    anyhow::bail!("missing mandatory 'Code' chunk");
}
