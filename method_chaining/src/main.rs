#[derive(Debug)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        Ticket {
            title,
            description,
            status,
        }
    }

    // Setter for title (returns `self` for chaining)
    pub fn set_title(mut self, new_title: String) -> Self {
        self.title = new_title;
        self
    }

    // Setter for description (returns `self` for chaining)
    pub fn set_description(mut self, new_description: String) -> Self {
        self.description = new_description;
        self
    }

    // Setter for status (returns `self` for chaining)
    pub fn set_status(mut self, new_status: String) -> Self {
        self.status = new_status;
        self
    }
}

fn main() {
    // Create a new Ticket and chain setters;
    let ticket = Ticket::new("Title".into(), "Description".into(), "To-Do".into())
        .set_title("New Title".into())
        .set_description("New Description".into())
        .set_status("In Progress".into());

    println!("{:?}", ticket);
    let ticket = ticket
        .set_title("Debugging".into())
        .set_description("Fix all the bugs".into())
        .set_status("Done".into());
    // Print the modified ticket
    println!("{:?}", ticket);
}
