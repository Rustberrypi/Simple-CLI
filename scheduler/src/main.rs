use std::io;

fn main() {
    println!("Hello, world!");
}

// An event has a name, day, start time, end time, and any of the three resources it needs to be using.
// resources.1 is camera #1, resources.2 is camera #2, resources.3 is the navigation system.
struct Event{
    name: String, 
    day: String,
    start_time: String,
    end_time: String,
    resources_1: bool,
    resources_2: bool,
    resources_3: bool,
}

/*fn make_event() -> Event {
    let mut event_name = String::new();
    let mut event_day = String::new();
    let mut event_start = String::new();
    let mut event_end = String::new();
    let mut r1 = bool::new();
    let mut r2 = bool::new();
    let mut r1 = bool::new();

    println!("Enter the name of the event");
    io::stdin()
        .read_line(&mut event_name)
        .expect("Failed to read event name.");

    println!("Enter the day of the week you would like the event to take place. (ex: Monday)");
    io:stdin()
        .read_line(&mut event_day)
        .expect(failed)
}*/