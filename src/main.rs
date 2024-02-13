use std::{collections::VecDeque, io::Read, net::IpAddr};

use naive_auto_complete::{tokens, tokens_vec_deque, Context, PoolContexts};

fn main() {
    let text = include_str!("../assets/text.txt").to_lowercase();
    let tokens = tokens::create_tokens(text.as_str());
    let pool_ctx = PoolContexts::create_pool(tokens.clone());

    // read input of user:
    let mut input = String::new(); // Initialize the input variable
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();

    input = get_the_last_50_chars(input);
    let input: Vec<&str> = input.split_whitespace().collect();

    let ctx = pool_ctx
        .contexts
        .get(input.first().unwrap().to_string().as_str())
        .unwrap();

    let mut vec = VecDeque::new();
    for word in input {
        vec.push_back(word.to_string());
    }
    for _ in 0..5 {
        let next = ctx.clone().find_possible_next(vec.clone());
        print!("{} ", next);
        vec.push_back(next);
    }
}

fn get_the_last_50_chars(s: String) -> String {
    let mut result = String::new();
    for c in s.chars().rev().take(50) {
        result.push(c);
    }
    result.chars().rev().collect()
}
