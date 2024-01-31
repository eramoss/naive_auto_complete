use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
};

#[derive(Debug, Clone)]
pub struct Context {
    root: Token,
    depth: u32,
}

/// Contexts with same root token value is considered equal
#[derive(Debug)]
struct PoolContexts {
    contexts: HashMap<String, Context>,
}
impl PoolContexts {
    pub fn new() -> PoolContexts {
        PoolContexts {
            contexts: HashMap::new(),
        }
    }
    pub fn create_pool(mut tokens: VecDeque<String>) -> PoolContexts {
        let mut pool = PoolContexts::new();
        assert!(tokens.len() > 0);
        loop {
            let ctx = pool
                .contexts
                .entry(tokens.front().unwrap().clone())
                .and_modify(|c| {
                    c.add(tokens.clone());
                })
                .or_insert(Context::create(tokens.clone()));
            tokens.pop_front();
            if tokens.len() == 0 {
                break;
            }
        }
        pool
    }
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

    pub fn find_possible_next(&mut self, mut tokens: VecDeque<String>) -> String {
        let empty_token = &mut Box::new(Token {
            value: String::new(),
            occurrences: 0,
            children: HashMap::new(),
        });
        let mut current = &mut self.root;
        tokens.pop_front(); // Remove root token
        for token in tokens {
            let next: &mut Token = current.children.get_mut(&token).unwrap_or_default();
            current = next;
        }
        Self::major_occurrences(current.to_owned()).value
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

#[derive(Debug, Clone)]
pub(crate) struct Token {
    value: String,
    occurrences: u32,
    children: HashMap<String, Box<Token>>,
}
impl Default for &mut Box<Token> {
    fn default() -> Self {
        let token = Token {
            value: String::new(),
            occurrences: 0,
            children: HashMap::new(),
        };
        Box::leak(Box::new(Box::new(token)))
    }
}
mod tokens {
    use std::collections::VecDeque;

    pub fn create_tokens(text: &str) -> VecDeque<String> {
        let mut tokens = VecDeque::new();
        for token in text.split_whitespace() {
            tokens.push_back(token.to_string());
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens;
    use crate::Context;
    use crate::PoolContexts;
    use std::collections::VecDeque;
    use std::ops::Deref;
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
    fn create_contexts() {
        let text = "rtx and rtx asd rtx and qwe zxc rtx and qwe";
        let tokens = tokens::create_tokens(text);
        let pool_ctx = PoolContexts::create_pool(tokens.clone());

        let rtx_ctx = pool_ctx.contexts.get("rtx").unwrap();
        dbg!(&rtx_ctx);
        let mut tokens = tokens_vec_deque!("rtx");
        for _ in 0..50 {
            let next = rtx_ctx.clone().find_possible_next(tokens.clone());
            dbg!(&next);
            tokens.push_back(next);
        }
    }
    #[test]
    fn test_split_into_words() {
        let text = "rtx and asd qwe zxc";
        let tokens = tokens_vec_deque!["rtx", "and", "asd", "qwe", "zxc"];
        assert_eq!(tokens, tokens::create_tokens(text));
    }
    #[test]
    fn test_find_possible() {
        let tokens = tokens_vec_deque!["rtx", "and", "asd", "qwe", "zxc"];
        let tokens_to_add = tokens_vec_deque!["rtx", "and", "hgr", "qwe", "zxc", "asd"];
        let more_added = tokens_vec_deque!["rtx", "jty", "hgr", "qwe", "zxc", "asd"];
        let more = tokens_vec_deque!["rtx", "and", "hgr", "qwe", "zxc", "asd"];
        let mut ctx = Context::create(tokens);
        ctx.add(tokens_to_add);
        ctx.add(more_added);
        ctx.add(more);

        assert_eq!(
            ctx.find_possible_next(tokens_vec_deque!["rtx", "and"]),
            "hgr".to_string()
        );
    }
}
