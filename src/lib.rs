use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
};

#[derive(Debug)]
struct Token {
    value: String,
    occurrences: u32,
    children: HashMap<String, Box<Token>>,
}

#[derive(Debug)]
pub struct Context {
    root: Token,
    depth: u32,
}

/// Contexts with same root token value is considered equal
struct PoolContexts {
    contexts: HashMap<String, Context>,
}

impl Context {
    pub fn new(root: String) -> Context {
        Context {
            root: Token {
                value: root,
                occurrences: 0,
                children: HashMap::new(),
            },
            depth: 0,
        }
    }

    pub fn create(mut tokens: VecDeque<String>) -> Context {
        let mut ctx = Context::new(tokens.pop_front().expect("Empty tokens"));
        let mut current = ctx.root.borrow_mut();
        for token in tokens {
            current = current
                .children
                .entry(token.clone())
                .and_modify(|e| {
                    e.occurrences += 1;
                })
                .or_insert(Box::new(Token {
                    value: token.clone(),
                    occurrences: 1,
                    children: HashMap::new(),
                }));
            ctx.depth += 1;
        }
        ctx
    }

    pub fn add(&mut self, mut tokens: VecDeque<String>) {
        let mut current = self.root.borrow_mut();
        tokens.pop_front();
        let mut added_depth: u32 = 0;
        for token in tokens {
            current = current
                .children
                .entry(token.clone())
                .and_modify(|e| {
                    e.occurrences += 1;
                })
                .or_insert(Box::new(Token {
                    value: token.clone(),
                    occurrences: 1,
                    children: HashMap::new(),
                }));

            if added_depth < self.depth {
                added_depth += 1;
            } else {
                self.depth += 1;
            }
        }
    }
}

#[macro_export]
macro_rules! tokens_vec_deque {
    // Match an empty invocation
    () => {
        VecDeque::new()
    };

    // Match a non-empty invocation and create a VecDeque with the specified values
    ($($elem:expr),*) => {
        {
            let mut temp_vecdeque = VecDeque::new();
            $(temp_vecdeque.push_back($elem.to_string());)*
            temp_vecdeque
        }
    };
}
