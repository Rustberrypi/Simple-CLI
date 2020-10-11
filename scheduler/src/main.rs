use std::io;

fn main() {
    let event1 = make_event();
    println!("{}", event1.name);
}

// An event has a name, day, start time, end time, and any of the three resources it needs to be using.
// resources.1 is camera #1, resources.2 is camera #2, resources.3 is the navigation system.
struct Event{
    name: String, 
    day: String,
    start_time: u32,
    end_time: u32,
    resource_1: bool,
    resource_2: bool,
    resource_3: bool,
}

fn make_event() -> Event {
    let mut event_name = String::new();
    let mut event_day = String::new();
    let mut event_start = String::new();
    let mut event_end = String::new();
    let mut condit = String::new();
    let mut r1: bool = false;
    let mut r2: bool = false;
    let mut r3: bool = false;

    println!("Enter the name of the event.");
    loop{
        io::stdin()
            .read_line(&mut event_name)
            .expect("Failed to read event name.");
        event_name = event_name.trim().to_string();
        if event_name == "" {
            println!("The name of the event must not be blank.");
        } else { break; }
    }

    println!("Enter the day of the week you would like the event to take place. (ex: Monday)");
    loop{
        io::stdin()
            .read_line(&mut event_day)
            .expect("failed to read event day.");
        event_day = event_day.trim().to_lowercase();
        if event_day == "monday" ||
           event_day == "tuesday" ||
           event_day == "wednesday" ||
           event_day == "thursday" ||
           event_day == "friday" ||
           event_day == "saturday" ||
           event_day == "sunday" {
            break;
        } else { 
            println!("Please enter a valid day of the week."); 
            event_day = String::new();
        }
    }

    println!("Please enter a start time in the 'XXXX' format. (ex: 5:24pm would be 1724)");
    loop{ 
        io::stdin()
            .read_line(&mut event_start)
            .expect("Failed to read start time.");
        let mut holder: u32 = event_start.trim().parse().unwrap_or(3000);
        if holder > 0 && holder < 2401 {
            break;
        } else {
            println!("please enter a time between 0 and 2400");
            event_start = String::new();
        }
    }
    let event_start: u32 = event_start.trim().parse().unwrap();

    println!("Please enter a end time in the 'XXXX' format. (ex: 5:24pm would be 1724)");
    loop{ 
        io::stdin()
            .read_line(&mut event_end)
            .expect("Failed to read end time.");
        let mut holder_2: u32 = event_end.trim().parse().unwrap_or(3000);
        if holder_2 > 0 && holder_2 < 2401 {
            break;
        } else {println!("please enter a time between 0 and 2400");}
    }
    let event_end: u32 = event_end.trim().parse().unwrap();

    println!("Do you need to use camerea 1? (enter y or n)");
    loop{
        io::stdin().read_line(&mut condit).expect("failed to read answer");
        if condit.trim().to_lowercase() == "y"{
            r1 = true;
            condit = "".to_string();
            break;
        } else if condit.trim().to_lowercase() == "n"{
            r1 = false;
            condit = "".to_string();
            break;
        } else {
            println!("please enter y or n");
            condit = "".to_string();
        }    
    }

    println!("Do you need to use camerea 2? (enter y or n)");
    loop{
        io::stdin().read_line(&mut condit).expect("failed to read answer");
        if condit.trim().to_lowercase() == "y"{
            r2 = true;
            condit = "".to_string();
            break;
        } else if condit.trim().to_lowercase() == "n"{
            r2 = false;
            condit = "".to_string();
            break;
        } else {
            println!("please enter y or n");
            condit = "".to_string();
        }    
    }
    
    println!("Do you need to use the navigation systems? (enter y or n)");
    loop{
        io::stdin().read_line(&mut condit).expect("failed to read answer");
        if condit.trim().to_lowercase() == "y"{
            r3 = true;
            condit = "".to_string();
            break;
        } else if condit.trim().to_lowercase() == "n"{
            r3 = false;
            condit = "".to_string();
            break;
        } else {
            println!("please enter y or n");
            condit = "".to_string();
        }    
    }
    
    let event = Event {
        name: event_name,
        day: event_day,
        start_time: event_start,
        end_time: event_end,
        resource_1: r1,
        resource_2: r2,
        resource_3: r3,
    };
    return event;
}