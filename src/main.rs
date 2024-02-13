use std::collections::VecDeque;

use naive_auto_complete::{tokens, PoolContexts};

fn main() {
    let text = include_str!("../assets/text.txt").to_lowercase();
    let tokens = tokens::create_tokens(text.as_str());
    let pool_ctx = PoolContexts::create_pool(tokens.clone());

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();

    input = get_the_last_50_chars(input);
    let input = tokens::split_into_trigrams(input.as_str());

    let ctx = pool_ctx
        .contexts
        .get(
            input
                .first()
                .unwrap_or(&"and".to_string())
                .to_string()
                .as_str(),
        )
        .unwrap_or_else(|| pool_ctx.contexts.get("and").unwrap());

    let mut vec = VecDeque::new();
    for word in input {
        vec.push_back(word.to_string());
    }
    let mut result = VecDeque::new();
    for _ in 0..15 {
        let next = ctx.clone().find_possible_next(vec.clone());
        result.push_back(next.clone());
        vec.push_back(next.to_string());
    }

    if ctx.root.value == "and" {
        result.push_front("and".to_string());
    } else {
        result.pop_front();
        result.pop_front();
    }
    println!("{}", tokens::merge_trigrams(result.into()));
}

fn get_the_last_50_chars(s: String) -> String {
    let mut result = String::new();
    for c in s.chars().rev().take(50) {
        result.push(c);
    }
    result.chars().rev().collect()
}
