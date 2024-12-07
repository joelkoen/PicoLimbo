use std::fmt::Display;

#[derive(Debug)]
pub struct Identifier {
    pub namespace: String,
    pub thing: String,
}

impl Identifier {
    pub fn from_string(string: &str) -> Self {
        let mut split = string.split(':');
        let namespace = split.next().unwrap_or("minecraft");
        let thing = split.next().unwrap_or("");
        Identifier {
            namespace: namespace.to_string(),
            thing: thing.to_string(),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.thing)
    }
}
