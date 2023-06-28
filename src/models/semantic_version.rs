use std::str::FromStr;

#[derive(Default)]
pub struct SemanticVersion {
    workspace: bool,

    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}

impl From<String> for SemanticVersion {
    fn from(value: String) -> Self {
        Self::from_str(&value).expect("cannot perform conversion from string to semantic-version")
    }
}

impl PartialEq for SemanticVersion {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for SemanticVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.major.partial_cmp(&other.major) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        match self.minor.partial_cmp(&other.minor) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.patch.partial_cmp(&other.patch)
    }
}

impl FromStr for SemanticVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = SemanticVersion::default();

        match s {
            "workspace" => {
                v.workspace = true;
                Ok(v)
            }
            s => {
                if let Some(stripped) = s.strip_prefix('^') {
                    return Self::from_str(stripped);
                }

                let parts: Vec<&str> = s.split('.').collect();

                if !parts.is_empty() {
                    v.major = parts[0].parse().unwrap_or_default();
                }

                if parts.len() > 1 {
                    v.minor = parts[1].parse().unwrap_or_default();
                }

                if parts.len() > 2 {
                    v.patch = parts[2].parse().unwrap_or_default();
                }

                Ok(v)
            }
        }
    }
}

impl ToString for SemanticVersion {
    fn to_string(&self) -> String {
        if self.workspace {
            return "workspace".to_string();
        }

        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
