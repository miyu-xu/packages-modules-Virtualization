use sha2::{Digest, Sha256};
use std::io::{Cursor, Read, Result, Write};

use crate::algorithms::HashAlgorithm;

pub struct HashTree {
    pub tree: Vec<u8>,
    pub root_hash: Vec<u8>,
}

impl HashTree {
    pub fn from<R: Read>(
        input: &mut R,
        input_size: usize,
        salt: &[u8],
        block_size: usize,
        algorithm: HashAlgorithm,
    ) -> Result<Self> {
        let salt = zero_pad_salt(salt, algorithm);
        let tree = generate_hash_tree(input, input_size, &salt, block_size, algorithm)?;
        let root_hash = if tree.is_empty() {
            let mut data = Vec::new();
            input.read_to_end(&mut data)?;
            hash_one_block(&data, &salt, block_size, algorithm)?
        } else {
            let first_block = &tree[0..block_size];
            hash_one_block(first_block, &salt, block_size, algorithm)?
        };
        Ok(HashTree { tree, root_hash })
    }
}

pub fn generate_hash_tree<R: Read>(
    input: &mut R,
    input_size: usize,
    salt: &[u8],
    block_size: usize,
    algorithm: HashAlgorithm,
) -> Result<Vec<u8>> {
    let digest_size = digest_size(algorithm);
    let levels = calc_hash_levels(input_size, block_size, digest_size);
    let tree_size = levels.iter().map(|r| r.len()).sum();
    let mut hash_tree = vec![0; tree_size];

    for (n, cur) in levels.iter().enumerate() {
        if n == 0 {
            let pad_size = round_to_multiple(input_size, block_size) - input_size;
            let mut input = input.chain(Cursor::new(vec![0; pad_size]));
            let mut level0 = Cursor::new(&mut hash_tree[cur.start..cur.end]);

            let mut a_block = vec![0; block_size];
            let mut num_blocks = (input_size + block_size - 1) / block_size;
            while num_blocks > 0 {
                input.read_exact(&mut a_block)?;
                let h = hash_one_block(&a_block, salt, block_size, algorithm)?;
                level0.write_all(&h).unwrap();
                num_blocks -= 1;
            }
        } else {
            let prev = &levels[n - 1];
            let cur_and_prev = &mut hash_tree[cur.start..prev.end];
            let (cur, prev) = cur_and_prev.split_at_mut(prev.start - cur.start);
            let mut cur = Cursor::new(cur);
            for data in prev.chunks(block_size) {
                let h = hash_one_block(data, salt, block_size, algorithm)?;
                cur.write_all(&h).unwrap();
            }
        }
    }
    Ok(hash_tree)
}

fn hash_one_block(
    input: &[u8],
    salt: &[u8],
    block_size: usize,
    algorithm: HashAlgorithm,
) -> Result<Vec<u8>> {
    let mut hasher = new_hasher(algorithm);
    hasher.update(salt);
    hasher.update(input);
    let pad_size = block_size - input.len();
    hasher.update(vec![0; pad_size]);
    Ok(hasher.finalize().to_vec())
}

type Range = std::ops::Range<usize>;

fn calc_hash_levels(input_size: usize, block_size: usize, digest_size: usize) -> Vec<Range> {
    let mut level_sizes = Vec::new();
    loop {
        let input_size = *level_sizes.last().unwrap_or(&input_size);
        if input_size <= block_size {
            break;
        }
        let num_blocks = (input_size + block_size - 1) / block_size;
        let hashes_size = round_to_multiple(num_blocks * digest_size, block_size);
        level_sizes.push(hashes_size);
    }

    let mut ranges = level_sizes
        .iter()
        .rev()
        .scan(0, |prev_end, size| {
            let range = *prev_end..*prev_end + size;
            *prev_end = range.end;
            Some(range)
        })
        .collect::<Vec<_>>();
    ranges.reverse();
    ranges
}

fn round_to_multiple(n: usize, unit: usize) -> usize {
    (n + unit - 1) & !(unit - 1)
}

fn zero_pad_salt(salt: &[u8], algorithm: HashAlgorithm) -> Vec<u8> {
    if salt.is_empty() {
        salt.to_vec()
    } else {
        let padded_len = round_to_multiple(salt.len(), block_size_for_hash(algorithm));
        let mut salt = salt.to_vec();
        salt.resize(padded_len, 0);
        salt
    }
}

fn new_hasher(algorithm: HashAlgorithm) -> Sha256 {
    match algorithm {
        HashAlgorithm::SHA256 => Sha256::new(),
    }
}

fn digest_size(algorithm: HashAlgorithm) -> usize {
    match algorithm {
        HashAlgorithm::SHA256 => 32,
    }
}

fn block_size_for_hash(algorithm: HashAlgorithm) -> usize {
    match algorithm {
        HashAlgorithm::SHA256 => 64,
    }
}
