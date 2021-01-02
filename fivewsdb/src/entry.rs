use std::fmt;

#[derive(Clone)]
pub struct LogEntry {
    pub who: String,
    pub what: String,
    pub when: String,
    pub r#where: String,
    pub why: String,
}

impl LogEntry {
    pub fn new<T: Into<String>>(who: T, what: T, when: T, r#where: T, why: T) -> LogEntry {
        LogEntry {
            who: who.into(),
            what: what.into(),
            when: when.into(),
            r#where: r#where.into(),
            why: why.into(),
        }
    }

    pub fn from<T>(v: Vec<T>) -> LogEntry
    where
        T: Into<String> + Copy,
    {
        LogEntry {
            who: v[0].into(),
            what: v[1].into(),
            when: v[2].into(),
            r#where: v[3].into(),
            why: v[4].into(),
        }
    }

    pub fn like(&self, field: &str, pattern: &str) -> bool {
        let pattern = pattern.to_lowercase();
        match field {
            "who" => self.who.to_lowercase().contains(&pattern),
            "what" => self.what.to_lowercase().contains(&pattern),
            "when" => self.when.to_lowercase().contains(&pattern),
            "where" => self.r#where.to_lowercase().contains(&pattern),
            "why" => self.why.to_lowercase().contains(&pattern),
            _ => false,
        }
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}",
            self.who, self.what, self.when, self.r#where, self.why
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let entry = LogEntry::new("name", "logged in", "2020-12-14T15:43:32", "", "");
        assert_eq!(entry.to_string(), "name|logged in|2020-12-14T15:43:32||");

        let entry = LogEntry::new(
            "name",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );
        assert_eq!(
            entry.to_string(),
            "name|Access Denied|2020-12-14T15:43:32|System::Login|Username or password was incorrect"
        );

        let entry = LogEntry::new("", "", "", "", "");
        assert_eq!(entry.to_string(), "||||");

        let entry = LogEntry::new("", "", "", "", "Why");
        assert_eq!(entry.to_string(), "||||Why");
    }

    #[test]
    fn test_like_who() {
        let entry = LogEntry::new(
            "name",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );

        assert_eq!(true, entry.like("who", "name"));
    }

    #[test]
    fn test_like_what() {
        let entry = LogEntry::new(
            "name",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );

        assert_eq!(true, entry.like("what", "access denied"));
    }

    #[test]
    fn test_like_when() {
        let entry = LogEntry::new(
            "name",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );

        assert_eq!(true, entry.like("when", "2020-12-14"));
    }

    #[test]
    fn test_like_where() {
        let entry = LogEntry::new(
            "name",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );

        assert_eq!(true, entry.like("where", "System"));
        assert_eq!(true, entry.like("where", "Login"));
        assert_eq!(false, entry.like("where", "syste::login"));
    }

    #[test]
    fn test_like_why() {
        let entry = LogEntry::new(
            "name",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );

        assert_eq!(true, entry.like("why", "username"));
        assert_eq!(true, entry.like("why", "password"));
        assert_eq!(true, entry.like("why", "Username or password"));
        assert_eq!(false, entry.like("why", "Usename or password was incorrect"));
    }
}
