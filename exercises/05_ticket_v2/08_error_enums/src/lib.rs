use crate::TicketNewError::{DescriptionError, TitleError};

enum TicketNewError {
    TitleError { reason: String },
    DescriptionError { reason: String },
}

fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    Ticket::new(title.clone(), description.clone(), status.clone())
        .unwrap_or_else(|err| {
            match err {
                TitleError { reason } => panic!("{reason}"),
                DescriptionError => Ticket {
                    title,
                    description: "Description not provided".to_string(),
                    status,
                }
            }
        })
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TitleError { reason: "Title cannot be empty".to_string() });
        }
        if title.len() > 50 {
            return Err(TitleError { reason: "Title cannot be longer than 50 characters".to_string() });
        }
        if description.is_empty() {
            return Err(DescriptionError { reason: "Description cannot be empty".to_string() });
        }
        if description.len() > 500 {
            return Err(DescriptionError { reason: "Description cannot be longer than 500 characters".to_string() });
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 characters")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
