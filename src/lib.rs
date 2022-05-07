use beam_file::chunk::CodeChunk;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {}

pub fn parse_code_chunk(code_chunk: &CodeChunk) -> Result<(), ParseError> {
    todo!()
}
