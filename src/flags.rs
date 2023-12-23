use std::collections::HashSet;

#[derive(Debug)]
pub struct FlagsHolder {
    flags: HashSet<String>,
}

impl Default for FlagsHolder {
    fn default() -> Self {
        FlagsHolder{flags: HashSet::new()}
    }
}

impl FlagsHolder {
    pub fn new(flags: Vec<String>) -> Self {
        FlagsHolder{flags: HashSet::from_iter(flags.into_iter())}
    }

    pub fn contains(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }
}
