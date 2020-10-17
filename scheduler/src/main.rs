use std::io;

fn main() {
    let mut event_list: Vec<Event> = Vec::new();
    add_event(&mut event_list);
    add_event(&mut event_list);
    delete_event(&mut event_list);
    for i in 0..event_list.len(){
        event_log(&mut event_list[i]);
    }
}

fn add_event(mut v: &mut Vec<Event>){
    let event = make_event();
    v.push(event);
    overlap(&mut v);
}

fn delete_event(mut v: &mut Vec<Event>){
    let mut event_name = String::new();
    let mut deleted: bool = false;
    println!("Here is the list of scheduled events: \n");
    for i in 0..v.len(){
        event_log(&mut v[i]);
    }
    println!("Enter the name of the event you would like to delete");
    loop{
        io::stdin()
            .read_line(&mut event_name)
            .expect("Failed to read event name.");
        for i in 0..v.len(){
            if event_name.trim() == v[i].name{
                v.remove(i);
                deleted = true;
            }
        }
        if deleted == true{
            break;
        }
        else{
            println!("The event name you entered was not found.");
            println!("Please enter a valid event name.");
            event_name = String::new();
        }
    }
    
}

fn event_log(event : &mut Event){
    println!("Name:               {}", event.name);
    println!("Day:                {}", event.day);
    println!("Start time:         {}", event.start_time);
    println!("End time:           {}", event.end_time);
    println!("Camera 1 in use:    {}", event.resource_1);
    println!("Camera 2 in use:    {}", event.resource_2);
    println!("Nav system in use:  {} \n", event.resource_3);
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

/* When creating an event the list of current events scheduled will be passed to the function.
   This is done to insure that if the event you are trying to create overlaps with an already existing event,
   and wants to use the same resources as that event it will ask the user if they would like to reschedule
   the event they are currently making or the already existing event.
*/
fn make_event() -> Event {
    let mut event_name = String::new();
    let mut event_day = String::new();
    let mut event_start = String::new();
    let mut event_end = String::new();
    let mut condit = String::new();
    /*let mut overlap = false;
    let mut ov_index: usize = 5000;
    let mut ov_r1: bool = false;
    let mut ov_r2: bool = false;
    let mut ov_r3: bool = false;*/
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
            condit = String::new();
            break;
        } else if condit.trim().to_lowercase() == "n"{
            r1 = false;
            condit = String::new();
            break;
        } else {
            println!("please enter y or n");
            condit = String::new();
        }    
    }

    println!("Do you need to use camerea 2? (enter y or n)");
    loop{
        io::stdin().read_line(&mut condit).expect("failed to read answer");
        if condit.trim().to_lowercase() == "y"{
            r2 = true;
            condit = String::new();
            break;
        } else if condit.trim().to_lowercase() == "n"{
            r2 = false;
            condit = String::new();
            break;
        } else {
            println!("please enter y or n");
            condit = String::new();
        }    
    }
    
    println!("Do you need to use the navigation systems? (enter y or n)");
    loop{
        io::stdin().read_line(&mut condit).expect("failed to read answer");
        if condit.trim().to_lowercase() == "y"{
            r3 = true;
            condit = String::new();
            break;
        } else if condit.trim().to_lowercase() == "n"{
            r3 = false;
            condit = String::new();
            break;
        } else {
            println!("please enter y or n");
            condit = String::new();
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

fn overlap(v: &mut Vec<Event>){
    let mut condit: String = String::new();
    let mut overlap: bool = false;
    let mut ov_r1: bool = false;
    let mut ov_r2: bool = false;
    let mut ov_r3: bool = false;
    for i in 0..v.len(){
        for j in (i+1)..v.len(){
            loop {
                if v[i].day == v[j].day{
                    if v[i].start_time <= v[j].start_time && v[i].end_time > v[j].start_time{
                        overlap = true;
                    } else if v[i].start_time < v[j].end_time && v[i].end_time >= v[j].end_time{
                        overlap = true;
                    } else if v[i].start_time > v[j].start_time && v[i].end_time < v[j].end_time{
                        overlap = true;
                    } else if v[i].start_time < v[j].start_time && v[i].end_time > v[j].end_time{
                        overlap = true;
                    } else {
                        overlap = false;
                        break;
                    }

                    if overlap == true && v[i].resource_1 == v[j].resource_1{
                        println!("Both {} and {} use camerea 1 at the same time.", v[i].name, v[j].name);
                        println!("If you would like to reschedule {} enter 1.", v[i].name);
                        println!("If you would like to reschedule {} enter 2.", v[j].name);
                        loop{
                            io::stdin()
                                .read_line(&mut condit)
                                .expect("Failed to read input.");
                            if condit.trim() == "1"{
                                v[i] = make_event();
                                condit = String::new();
                                break;
                            } else if condit.trim() == "2"{
                                v[j] = make_event();
                                condit = String::new();
                                break;
                            } else{
                                println!("Please enter 1 or 2");
                                condit = String::new();
                            }
                        }
                    } else if overlap == true && v[i].resource_2 == v[j].resource_2{
                        println!("Both {} and {} use camerea 2 at the same time.", v[i].name, v[j].name);
                        println!("If you would like to reschedule {} enter 1.", v[i].name);
                        println!("If you would like to reschedule {} enter 2.", v[j].name);
                        loop{
                            io::stdin()
                                .read_line(&mut condit)
                                .expect("Failed to read input.");
                            if condit.trim() == "1"{
                                v[i] = make_event();
                                condit = String::new();
                                break;
                            } else if condit.trim() == "2"{
                                v[j] = make_event();
                                condit = String::new();
                                break;
                            } else{
                                println!("Please enter 1 or 2");
                                condit = String::new();
                            }
                        }
                    } else if overlap == true && v[i].resource_3 == v[j].resource_3{
                        println!("Both {} and {} use the navigation system at the same time.", v[i].name, v[j].name);
                        println!("If you would like to reschedule {} enter 1.", v[i].name);
                        println!("If you would like to reschedule {} enter 2.", v[j].name);
                        loop{
                            io::stdin()
                                .read_line(&mut condit)
                                .expect("Failed to read input");
                            if condit.trim() == "1"{
                                v[i] = make_event();
                                condit = String::new();
                                break;
                            } else if condit.trim() == "2"{
                                v[j] = make_event();
                                condit = String::new();
                                break;
                            } else{
                                println!("Please enter 1 or 2");
                                condit = String::new();
                            }
                        }
                    } else {break;}
                } else {break;}
            }
        }
    }
}