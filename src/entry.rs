use std::fmt;

#[derive(Debug, Clone)]
pub struct FiveWsEntry {
    who: String,
    what: String,
    when: String,
    r#where: String,
    why: String,
}

impl FiveWsEntry {
    pub fn new<T: Into<String>>(who: T, what: T, when: T, r#where: T, why: T) -> FiveWsEntry {
        FiveWsEntry {
            who: who.into(),
            what: what.into(),
            when: when.into(),
            r#where: r#where.into(),
            why: why.into(),
        }
    }

    pub fn from<T>(v: Vec<T>) -> FiveWsEntry where T: Into<String> + Copy {
        FiveWsEntry {
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

impl fmt::Display for FiveWsEntry {
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
        let entry = FiveWsEntry::new("ingi", "logged in", "2020-12-14T15:43:32", "", "");
        assert_eq!(entry.to_string(), "ingi|logged in|2020-12-14T15:43:32||");

        let entry = FiveWsEntry::new(
            "ingi",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );
        assert_eq!(
            entry.to_string(),
            "ingi|Access Denied|2020-12-14T15:43:32|System::Login|Username or password was incorrect"
        );

        let entry = FiveWsEntry::new("", "", "", "", "");
        assert_eq!(entry.to_string(), "||||");

        let entry = FiveWsEntry::new("", "", "", "", "Why");
        assert_eq!(entry.to_string(), "||||Why");
    }

    #[test]
    fn test_like_who() {
        let entry = FiveWsEntry::new(
            "ingi",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );

        assert_eq!(true, entry.like("who", "ingi"));
    }

    #[test]
    fn test_like_what() {
        let entry = FiveWsEntry::new(
            "ingi",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );

        assert_eq!(true, entry.like("what", "access denied"));
    }

    #[test]
    fn test_like_when() {
        let entry = FiveWsEntry::new(
            "ingi",
            "Access Denied",
            "2020-12-14T15:43:32",
            "System::Login",
            "Username or password was incorrect",
        );

        assert_eq!(true, entry.like("when", "2020-12-14"));
    }

    #[test]
    fn test_like_where() {
        let entry = FiveWsEntry::new(
            "ingi",
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
        let entry = FiveWsEntry::new(
            "ingi",
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
