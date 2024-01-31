use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
};

#[derive(Debug, Clone)]
pub struct Token {
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

    pub fn find_possible_next(&mut self, mut tokens: VecDeque<String>) -> Token {
        let mut current = &mut self.root;
        tokens.pop_front(); // Remove root token
        for token in tokens {
            let next: &mut Token = current.children.get_mut(&token).unwrap_or_else(|| todo!());
            current = next;
        }
        Self::major_occurrences(current.to_owned())
    }
    fn major_occurrences(token: Token) -> Token {
        let empty_token = Token {
            value: String::new(),
            occurrences: 0,
            children: HashMap::new(),
        };
        let mut major_occurrences = empty_token;
        for (_, child) in token.children {
            if child.occurrences > major_occurrences.occurrences {
                major_occurrences = *child;
            }
        }
        major_occurrences
    }
}

#[cfg(test)]
mod tests {
    use crate::Context;
    use std::collections::VecDeque;

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
    #[test]
    fn test_find_possible() {
        let tokens = tokens_vec_deque!["rtx", "and", "asd", "qwe", "zxc"];
        let tokens_to_add = tokens_vec_deque!["rtx", "and", "hgr", "qwe", "zxc", "asd"];
        let more_added = tokens_vec_deque!["rtx", "jty", "hgr", "qwe", "zxc", "asd"];
        let mut ctx = Context::create(tokens);
        ctx.add(tokens_to_add);
        ctx.add(more_added);

        assert_eq!(
            ctx.find_possible_next(tokens_vec_deque!["rtx"]).value,
            "and".to_string()
        );
    }
}
