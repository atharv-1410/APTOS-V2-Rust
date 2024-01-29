// struct for Ticket and Event
struct Ticket {
    event_id: u64,
    price: u64,
    owner: String, // Assuming owner is identified by a string 
    is_for_sale: bool,
}

struct Event {
    organizer: String, // Assuming organizer is identified by a string 
    initial_price: u64,
    total_tickets: u64,
    event_date: u64,
    is_sale_allowed: bool,
}

// functions for the Ticket Market
impl Ticket {
    fn new(event_id: u64, price: u64, owner: String, is_for_sale: bool) -> Self {
        Ticket {
            event_id,
            price,
            owner,
            is_for_sale,
        }
    }
}

impl Event {
    fn new(organizer: String, initial_price: u64, total_tickets: u64, event_date: u64, is_sale_allowed: bool) -> Self {
        Event {
            organizer,
            initial_price,
            total_tickets,
            event_date,
            is_sale_allowed,
        }
    }
}

impl Event {
    // Ticket buying API
    fn buy_ticket(&mut self, event_id: u64, buyer: String, buying_price: u64) -> Result<(), &'static str> {
        // Check if event allows ticket sales
        if !self.is_sale_allowed {
            return Err("Ticket sales not allowed for this event");
        }

        // Find available tickets for the event
        // Assuming tickets are stored in a vector
        let tickets: &mut Vec<Ticket> = self.get_tickets_mut(event_id)?;

        // Search for the cheapest available ticket
        let cheapest_ticket = self.get_cheapest_ticket(tickets)?;

        // Check if buyer's price is greater than or equal to the cheapest ticket
        if buying_price < cheapest_ticket.price {
            return Err("Buying price is lower than the cheapest available ticket");
        }

        // Transfer ticket ownership
        cheapest_ticket.owner = buyer;
        cheapest_ticket.is_for_sale = false;

        // Assume transferring tokens here (not implemented)
        
        Ok(())
    }

    // Ticket selling API
    fn sell_ticket(&mut self, event_id: u64, seller: String, selling_price: u64) -> Result<(), &'static str> {
        // Check if event allows ticket sales
        if !self.is_sale_allowed {
            return Err("Ticket sales not allowed for this event");
        }

        // Create a new ticket and add it to the tickets vector
        // Here, we're assuming that tickets are stored in a vector in the Event struct
        let new_ticket = Ticket::new(event_id, selling_price, seller, true);
        // Push the new ticket to the event's ticket vector
        // Assuming self.tickets is a vector of tickets
        self.tickets.push(new_ticket);

        Ok(())
    }

    // Internal function to get the cheapest available ticket
    fn get_cheapest_ticket(&self, tickets: &Vec<Ticket>) -> Result<&Ticket, &'static str> {
        let cheapest_ticket = tickets.iter().min_by_key(|t| t.price).ok_or("No tickets available")?;
        Ok(cheapest_ticket)
    }

    // Internal function to get mutable reference to tickets for an event
    fn get_tickets_mut(&mut self, event_id: u64) -> Result<&mut Vec<Ticket>, &'static str> {
        // Here, assuming tickets are stored in a vector in the Event struct
        // Assuming event_id corresponds to the index of the event in some events vector
        Ok(&mut self.tickets)
    }
}

fn main() {
    // Example usage
    let mut event = Event::new(
        "Organizer1".to_string(),
        100,  // Initial ticket price
        1000, // Total tickets
        1643443200, // Event date (Unix timestamp)
        true, // Is sale allowed
    );

}
