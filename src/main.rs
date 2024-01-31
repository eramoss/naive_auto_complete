use std::collections::VecDeque;

use naive_auto_complete::{tokens_vec_deque, Context};

fn main() {
    let tokens = tokens_vec_deque!["rtx", "and", "asd", "qwe", "zxc"];
    let tokens_to_add = tokens_vec_deque!["rtx", "and", "hgr", "qwe", "zxc", "asd"];
    let mut ctx = Context::create(tokens);
    ctx.add(tokens_to_add);
    dbg!(ctx);
}
