pub type AdventResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn split_blocks(content: &str) -> Vec<Vec<&str>> {
    let mut blocks = Vec::new();
    let mut current_block = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() && !current_block.is_empty() {
            blocks.push(current_block);
            current_block = Vec::new();
        } else {
            current_block.push(line);
        }
    }

    if !current_block.is_empty() {
        blocks.push(current_block);
    }
    blocks
}
