// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketDescription` type,
//   enforcing that the description is not empty and is not longer than 500 bytes.
//   Implement the traits required to make the tests pass too.

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct TicketDescription(String);

#[derive(Debug)]
pub struct ParseDescriptionError(String);

impl Display for ParseDescriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseDescriptionError(msg) => write!(f, "{}", msg),
        }
    }
}

impl TryFrom<String> for TicketDescription {
    type Error = ParseDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 500 {
            return Err(ParseDescriptionError(
                "The description cannot be longer than 500 bytes".into(),
            ));
        }

        if value.is_empty() {
            return Err(ParseDescriptionError(
                "The description cannot be empty".into(),
            ));
        }

        return Ok(TicketDescription(value));
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = ParseDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 500 {
            return Err(ParseDescriptionError(
                "The description cannot be longer than 500 bytes".into(),
            ));
        }

        if value.is_empty() {
            return Err(ParseDescriptionError(
                "The description cannot be empty".into(),
            ));
        }

        return Ok(TicketDescription(value.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let description = TicketDescription::try_from("A description".to_string()).unwrap();
        assert_eq!(description.0, "A description");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketDescription::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The description cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let description = "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.".to_string();
        let err = TicketDescription::try_from(description).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The description cannot be longer than 500 bytes"
        );
    }

    #[test]
    fn test_try_from_str() {
        let description = TicketDescription::try_from("A description").unwrap();
        assert_eq!(description.0, "A description");
    }
}
