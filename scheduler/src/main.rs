use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;
extern crate rpassword;
mod login;

#[allow(unused_must_use)]
fn main() {
    let user: bool = login::login();
    let mut event_list: Vec<Event> = Vec::new();
    let mut days: Vec<String> =Vec::new();
    days.push("sunday".to_string());
    days.push("monday".to_string());
    days.push("tuesday".to_string());
    days.push("wednesday".to_string());
    days.push("thursday".to_string());
    days.push("friday".to_string());
    days.push("saturday".to_string());
    //let mut user_type: String = String::new();
    let mut answer: String = String::new();
    let mut existing_schd: String = String::new();
    let mut quit: String = String::new();
    

    // println!("Enter 1 or 2 depending on what type of user you are.");
    // println!("Enter 1 if you are a view only user.");
    // println!("Enter 2 if you are a user that can make changes.");
    // loop{
    //     io::stdin()
    //         .read_line(&mut user_type)
    //         .expect("Failed to read user type.");
    //     if user_type.trim() == "1" || user_type.trim() == "2"{
    //         break;
    //     } else {
    //         println!("Please enter 1 or 2");
    //         user_type = String::new();
    //     }
    // }
    //Open existing file
    if user == true{
        println!("Do you want to view an existing schedule? Type yes or no.");
        loop{
            io::stdin()
                .read_line(&mut existing_schd)
                .expect("Failed to read user type.");
            if existing_schd.trim().to_lowercase() == "yes"{
                read_data(&mut open_file(), &mut event_list);
                break;
            } else if existing_schd.trim().to_lowercase() == "no"{
                break;
            } else {
                println!("Try Again");
                existing_schd = String::new();
            }
        }
        loop{
            println!("\nEnter 1 to view the schedule");
            println!("Enter 2 to advance to the next day");
            println!("Enter 3 to add an event");
            println!("Enter 4 to delete an event");
            println!("Enter 5 to quit");
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read answer");
            if answer.trim() == "1"{
                show_schedule(&mut event_list, &mut days);
                answer = String::new();
            } else if answer.trim() == "2"{
                advance_day(&mut event_list, &mut days);
                answer = String::new();
            } else if answer.trim() == "3"{
                add_event(&mut event_list);
                answer = String::new();
            } else if answer.trim() == "4"{
                delete_event(&mut event_list);
                answer = String::new();
            } else if answer.trim() == "5"{
                println!("Would you like to save before exiting? (Enter y or n)");
                io::stdin()
                    .read_line(&mut quit)
                    .expect("Failed to read user type.");
                if quit.trim().to_lowercase() == "y"{
                    save_schedule(&mut event_list);
                }
                break;
            } else {
                println!("Please enter either 1, 2, 3, 4, or 5");
            }
        }
    } else {
        println!("Only users are allowed to access the scheduler.")
    }  
}

/* This method will check what the current day is and scan the events Vec to see what events occour on that day.
   If an event occurs on that day the event log will be printed for that event and it will be removed from the list.
   The current day will then be changed to the next day in the Vec and the day that was advanced will be appended 
   to the end of the Vec.
*/
fn advance_day(events: &mut Vec<Event>, days: &mut Vec<String>){
    let mut indecies: Vec<usize> = Vec::new();
    println!("Here are the events successfully completed on {}:\n", days[0]);

    for i in 0..events.len(){
        if events[i].day == days[0]{
            event_log(&mut events[i]);
            indecies.push(i);
        }
    }
    
    loop{
        if indecies.len() == 0{
            break;
        } else {
            events.remove(indecies[indecies.len() - 1]);
            indecies.remove(indecies.len() - 1);
        }
    }

    if days[0] == "sunday"{
        days.push("sunday".to_string());
    } else if days[0] == "monday"{
        days.push("monday".to_string());
    } else if days[0] == "tuesday"{
        days.push("tuesday".to_string());
    } else if days[0] == "wednesday"{
        days.push("wednesday".to_string());
    } else if days[0] == "thursday"{
        days.push("thursday".to_string());
    } else if days[0] == "friday"{
        days.push("friday".to_string());
    } else if days[0] == "saturday"{
        days.push("saturday".to_string());
    }
    days.remove(0);
}

#[allow(unused_assignments)]
fn save_schedule( v: &mut Vec<Event>) -> std::io::Result<()>{
    let mut file_name:String = String::new();

    println!("What would you like to name the file");
    io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read filr name.");
    file_name = "src/".to_string() + file_name.trim() + ".txt";
    let mut file = File::create(file_name)?;
    for i in 0..v.len(){
        let event_name = v[i].name.to_string();
        let event_day = v[i].day.to_string();
        let event_start = v[i].start_time.to_string();
        let event_end = v[i].end_time.to_string();
        let mut r1: String = String::new();
        let mut r2: String = String::new();
        let mut r3: String = String::new();
        if v[i].resource_1 == true{
            r1 = "y".to_string();
        } else {r1 = "n".to_string();}
        if v[i].resource_2 == true{
            r2 = "y".to_string();
        } else {r2 = "n".to_string();}
        if v[i].resource_3 == true{
            r3 = "y".to_string();
        } else {r3 = "n".to_string();}
        write!(file, "{}\n", event_name)?;
        write!(file, "{}\n", event_day)?;
        write!(file, "{}\n", event_start)?;
        write!(file, "{}\n", event_end)?;
        write!(file, "{}\n", r1)?;
        write!(file, "{}\n", r2)?;
        write!(file, "{}\n", r3)?;

    }
    Ok(())
}

fn show_schedule(events: &mut Vec<Event>, days: &mut Vec<String>){
    for i in 0..days.len(){
        println!("\nHere are the events scheduled for {}:", days[i]);
        for j in 0..events.len(){
            if days[i] == events[j].day.to_lowercase(){
                event_log(&mut events[j]);
            }
        }
    }
}

fn add_event(mut v: &mut Vec<Event>){
    let event = make_event();
    v.push(event);
    overlap(&mut v);
}

//Opens existing schedule
fn open_file() -> String {
    let mut schedule: String = String::new();
    let mut data = String::new();
    loop{
    println!("What is the schedule name? Name is case sensitive.");
    io::stdin()
        .read_line(&mut schedule)
        .expect("Failed to read user input");
        
        schedule = schedule.trim().to_string() + ".txt";
        let source = "./src/".to_string();
        let schpath = source + &schedule;
        let path = Path::new(&schpath);
    if  path.is_file(){
    let mut f = File::open(schpath).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    break;
    }
    else{
        println!("File does not exist. Try again.");
        schedule = String::new();
    }
    
}
        return data;
}

//Creates save state

//Parse saved date
#[allow(unused_assignments)]
fn read_data(data: &mut String, event_list: &mut Vec<Event>){
    //println!("{}", data);
    let mut name:String = String::new();
    let mut day:String = String::new();
    let mut start_time:String = String::new();
    let mut end_time:String = String::new();
    let mut resource_1:String = String::new();
    let mut resource_2:String = String::new();
    let mut resource_3:String = String::new();
    let a: Vec<String> = data.split("\n").map(|s| s.to_string()).collect();
    let alength = a.len();
    

    
    
    for x in 0..alength{
        if ((x+1) % 7) == 1{
           name = (*a[x]).to_string();
           //println!("{}", name);
        }
        else if ((x+1) % 7) ==2{
           day = (*a[x]).to_string();
           //println!("{}", day);
        }
        else if ((x+1) % 7) ==3{
           start_time = (*a[x]).to_string();
           //println!("{}", start_time);
        }
        else if ((x+1) % 7) ==4{
           end_time = (*a[x]).to_string();
           //println!("{}", end_time);
        }
        else if ((x+1) % 7) ==5{
           resource_1 = (*a[x]).to_string();
           //println!("{}", resource_1);
        }
        else if ((x+1) % 7) ==6{
           resource_2 = (*a[x]).to_string();
           //println!("{}", resource_2);
        }
        else if ((x+1) % 7) == 0{
           //println!("{}", ((x+1) % 7)); 
           resource_3 = (*a[x]).to_string();
           //println!("{}", resource_3);
           recreate_events(name.to_string(), day.to_string(), start_time.to_string(), end_time.to_string(), resource_1.to_string(), resource_2.to_string(), resource_3.to_string(), event_list)
        }
    }
    //println!("{}", event_list.len());


}

#[allow(unused_assignments)]
fn recreate_events(event_name: String, event_day: String, event_start: String, event_end: String, r1: String, r2: String, r3: String, event_list: &mut Vec<Event>){
    // println!("{}", event_name);
    // println!("{}", event_day);
    // println!("{}", event_start);
    // println!("{}", event_end);
    // println!("{}", r1);
    // println!("{}", r2);
    // println!("{}", r3);


    let mut b1:bool=false;
    let mut b2:bool=false;
    let mut b3:bool=false;
    if r1 == "n"{
     b1= false;
    }
    else{
        b1=true;
    }
    if r2 == "n"{
        b2= false;
    }
    else{
        b2=true;
    }
    if r3 == "n"{
        b3= false;
    }
    else{
        b3=true;
    }
    let event = Event{
        name: event_name,
        day: event_day,
        start_time: event_start.trim().parse::<u32>().unwrap(),
        end_time: event_end.trim().parse::<u32>().unwrap(),
        resource_1: b1,
        resource_2: b2,
        resource_3: b3,
    };
    event_list.push(event);
    //event_log(&mut event_list[0]);
}


fn delete_event(v: &mut Vec<Event>){
    let mut event_name = String::new();
    let mut deleted: bool = false;
    if v.len() == 0{
        println!("There are no events scheduled")
    } else {
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
                if deleted == false{
                    if event_name.trim() == v[i].name{
                        v.remove(i);
                        deleted = true;
                    }
                }
            }
            if deleted == true{
                break;
            } else {
                println!("The event name you entered was not found.");
                println!("Please enter a valid event name.");
                event_name = String::new();
            }
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
#[allow(unused_assignments)]
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
        let holder: u32 = event_start.trim().parse().unwrap_or(3000);
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
        let holder_2: u32 = event_end.trim().parse().unwrap_or(3000);
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

/* This method checks for overlaps when creating an event. If an overlap is detected it will check the resource use
   for both Events. If both the events require the same resource the user will be asked to reschedule an Event of 
   their choice. If the Events don't require the same resource the overlap will be allowed.
*/
#[allow(unused_assignments)]
fn overlap(v: &mut Vec<Event>){
    let mut condit: String = String::new();
    let mut overlap: bool = false;
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

                    if overlap == true && v[i].resource_1 == true && v[j].resource_1 == true{
                        println!("Both {} and {} use camerea 1 at the same time.", v[i].name, v[j].name);
                        event_log(&mut v[i]);
                        event_log(&mut v[j]);
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
                    } else if overlap == true && v[i].resource_2 == true && v[j].resource_2 == true{
                        println!("Both {} and {} use camerea 2 at the same time.", v[i].name, v[j].name);
                        event_log(&mut v[i]);
                        event_log(&mut v[j]);
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
                    } else if overlap == true && v[i].resource_3 == true && v[j].resource_3 == true{
                        println!("Both {} and {} use the navigation system at the same time.", v[i].name, v[j].name);
                        event_log(&mut v[i]);
                        event_log(&mut v[j]);
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