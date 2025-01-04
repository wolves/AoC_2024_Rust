pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let mut buffer = String::with_capacity(input.len() + 20);
    let mut i = 0;

    let result = loop {
        i += 1;

        buffer.clear();
        buffer.push_str(input);
        buffer.push_str(&i.to_string());

        let hash = md5::compute(buffer.as_bytes());

        if format!("{:x}", hash).as_bytes().starts_with(b"000000") {
            break i;
        }
    };

    Ok(result.to_string())
}
