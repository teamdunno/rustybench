use std::collections::HashMap;

pub fn get_indexed_scores() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(
        "dhry".to_string(),
        116700
    );
    scores.insert(
        "whets".to_string(),
        55
    );
    scores.insert(
        "execl".to_string(),
        43
    );
    scores.insert(
        "fstime".to_string(),
        3960
    );
    scores.insert(
        "fsdisk".to_string(),
        5800
    );
    scores.insert(
        "pipe".to_string(),
        12440
    );
    scores.insert(
        "context1".to_string(),
        4000
    );
    scores.insert(
        "spawn".to_string(),
        126
    );
    scores.insert(
        "shell8".to_string(),
        60
    );
    scores.insert(
        "syscall".to_string(),
        15000
    );

    scores.clone()
}

pub fn get_score(bench_scores: Vec<i32>, index_score: i32) -> HashMap<String, Vec<i64>> {
    let mut result = HashMap::new();
    let mut seperately_indexed = vec![];
    let mut final_result = vec![];
    let mut temporary_thingy: i64 = 0;
    bench_scores.iter().for_each(
        |score| {
            let ratio: i32 = *score as i32 / index_score as i32;
            let index = ratio * 10 as i32;
            seperately_indexed.push(index as i64);
            temporary_thingy += index as i64;
        }
    );
    // sum the scores
    final_result.push((temporary_thingy / seperately_indexed.len() as i64) as i64);
    result.insert(String::from("final"), final_result);
    result.insert(String::from("seperate"), seperately_indexed);

    result
}
