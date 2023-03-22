use std::fmt::Display;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct StringBuilder {
    string: String,
}

impl StringBuilder {
    pub fn push(&mut self, str: String) {
        self.string.push_str(&str);
    }

    pub fn pushln(&mut self, str: String) {
        self.string.push_str(&format!("{}\n", str))
    }

    pub fn add(&mut self, str: &str) {
        self.string.push_str(str);
    }

    pub fn addln(&mut self, str: &str) {
        self.string.push_str(&format!("{}\n", str))
    }

    pub fn clear(&mut self) {
        self.string = String::new()
    }
}

impl Display for StringBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
