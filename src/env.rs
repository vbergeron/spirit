use crate::AST;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    vars: HashMap<EnvKey, AST>,
    natives: HashMap<String, fn(Vec<AST>) -> Result<AST, String>>,
    frames: u8,
    debug: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct EnvKey {
    frame: u8,
    name: String,
}

impl EnvKey {
    pub fn new(frame: u8, name: String) -> EnvKey {
        EnvKey {
            frame: frame,
            name: name,
        }
    }
}

impl Env {
    pub fn new(debug: bool) -> Env {
        return Env {
            vars: HashMap::new(),
            natives: HashMap::new(),
            frames: 1,
            debug: debug,
        };
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn frame_push(&mut self) -> () {
        self.frames += 1;
        if self.debug {
            println!("{}> FRAME PUSH", self.frames);
        }
    }

    pub fn frame_pop(&mut self) -> () {
        if self.debug {
            println!("{}< FRAME POP", self.frames);
        }
        self.frames -= 1;
    }

    pub fn add_var(&mut self, name: String, value: AST) -> () {
        let key = EnvKey::new(self.frames, name);
        if self.debug {
            println!("{}: SET {:?} <- {:?}", self.frames, key, value);
        }
        self.vars.insert(key, value);
    }

    pub fn get_var(&self, name: String) -> Option<&AST> {
        let mut i: u8 = self.frames;

        while i > 0 {
            let key = EnvKey::new(i, name.clone());
            let var = self.vars.get(&key);

            if var.is_some() {
                if self.debug {
                    println!("{}: GET {:?}", self.frames, key);
                }
                return var;
            }

            i -= 1;
        }
        return None;
    }

    pub fn del_var(&mut self, name: String) -> () {
        let key = EnvKey::new(self.frames, name);
        if self.debug {
            println!("{}: DEL {:?}", self.frames, key);
        }
        self.vars.remove(&key);
    }

    pub fn add_native(&mut self, name: String, value: fn(Vec<AST>) -> Result<AST, String>) -> () {
        self.natives.insert(name, value);
    }

    pub fn get_native(&mut self, name: String) -> Option<&fn(Vec<AST>) -> Result<AST, String>> {
        self.natives.get(&name)
    }
}
