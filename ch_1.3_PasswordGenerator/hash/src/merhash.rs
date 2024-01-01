/// 梅森哈希
///
/// 用梅森素數 127 計算哈希值
/// 
/// # 示例
/// use hash::merhash::mersenne_hash;
/// 
/// let seed = String::from("jdxjp");
/// let hash = mersenne_hash(&seed);
/// assert_eq!(2000375, hash);
pub fn mersenne_hash(seed: &str) -> usize {
    let mut hash: usize = 0;

    for (i, c) in seed.chars().enumerate() {
        hash += (i + 1) * (c as usize);
    }

    (hash % 127).pow(3) - 1
}
