use rayon::prelude::*;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    const CHUNK_SIZE: usize = 25_000;

    let result = (1..)
        .into_iter()
        .step_by(CHUNK_SIZE)
        .find_map(|chunk_start| {
            (chunk_start..chunk_start + CHUNK_SIZE)
                .into_par_iter()
                .find_first(|&i| {
                    let mut buffer = String::with_capacity(input.len() + 20);
                    buffer.push_str(input);
                    buffer.push_str(&i.to_string());

                    let hash = md5::compute(buffer.as_bytes());

                    format!("{:x}", hash).starts_with("000000")
                })
        })
        .expect("A solution exists");

    Ok(result.to_string())
}
