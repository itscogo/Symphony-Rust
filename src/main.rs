use std::io;

fn main() {

    let mut users: Vec<User> = vec![];
    users.push(User{
       name: String::from("Josh"),
       id: 0,
       active: true,
        charge_history: vec![],
        current_checkouts: vec![],
    });
    users.push(User{
       name: String::from("Bob Vance"),
       id: 1,
       active: true,
        charge_history: vec![],
        current_checkouts: vec![],
    });
   
    let mut items: Vec<Item> = vec![];
    items.push(Item {
        name: String::from("Harry Potter"),
        id: 0,
        status: Status::CheckedOut,
        genre: Genre::Fantasy,
        branch: Branch::PointCook,
    });
    items.push(Item {
        name: String::from("The accidental medium"),
        id: 1,
        status: Status::OnShelf,
        branch: Branch::Hoppers,
        genre: Genre::Fantasy,
    });
   
    let mut id_count = 2;
   
    loop{
        println!("1. Create new user");
        println!("2. Display all users");
        println!("3. Display user");
        println!("4. Display items");
        println!("5. Checkout Item");
        println!("6. Checkin Item");

       
        let mut input_choice = String::new();
        io::stdin()
                    .read_line(&mut input_choice)
                    .expect("Input error");
                   
       
        let trimmed = input_choice.trim();
        match trimmed.parse::<u8>(){
           Ok(i) if i == 1 => add_new_user(&mut users, id_count),
           Ok(i) if i == 2 => display_users(&mut users),
           Ok(i) if i == 3 => display_user(&mut users),
           Ok(i) if i == 4 => display_items(&mut items),
           Ok(i) => println!("{} was not a choice", i),
           Err(..) => println!("ERROR: {} is not an integer", trimmed),
        };
    }
}

#[derive(Debug)]
struct User{
    name: String,
    id: u64,
    active: bool,
    charge_history: Vec<u32>,
    current_checkouts: Vec<u32>,
}

#[derive(Debug)]
struct Item{
    name: String,
    id: u64,
    status: Status,
    genre: Genre,
    branch: Branch,
}

#[derive(Debug)]
enum Status {
    OnShelf,
    CheckedOut, 
    OnHold,
}

#[derive(Debug)]
enum Genre {
    Fiction,
    NonFiction, 
    Parenting,
    Society,
    Health, 
    Scifi,
    Fantasy,
}

#[derive(Debug)]
enum Branch {
    Hoppers,
    PointCook, 
    Tarneit,
    Manorlakes,
    Werribee, 
    Williamslanding,
}

fn display_items(items: &mut Vec<Item>){
    for item in items.iter_mut(){
        println!("{:?}", item);
    };
}

fn display_user(users: &mut Vec<User>){
   
    println!("Enter user ID");

    let mut id_input = String::new();
    io::stdin()
            .read_line(&mut id_input)
            .expect("ERROR");
    let id_trimmed = id_input.trim();
   
    match id_trimmed.parse::<u64> (){
        Ok(i) => {
            let mut found = false;
            for user in users.iter_mut(){
                if user.id == i {
                    println!("{:?}", user);
                    found = true;
                    break;
                }
            }
            if found == false{
                println!("No user with ID {}", id_trimmed)
            }
        },
        Err(..) => println!("Error: {} not a number", id_trimmed),
    }
}

fn display_users(users: &mut Vec<User>){
    for user in users.iter_mut(){
        println!("{:?}", user);
    };   
}

fn add_new_user(users: &mut Vec<User>, mut id_count: u64) {
   
    println!("Name: ");
    let mut name = String::new();
    io::stdin()
                .read_line(&mut name)
                .expect("Error");
               
    users.push (User{
        name : name,
        id : id_count ,
        active: true,
        charge_history: vec![],
        current_checkouts: vec![],
    });
    id_count +=1;
}