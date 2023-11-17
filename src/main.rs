use std::io;

fn main() {

    let mut Users: Vec<User> = vec![];
    Users.push(User{
       name: String::from("Josh"),
       id: 0,
       active: true,
        charge_history: vec![],
        current_checkouts: vec![],
    });
    Users.push(User{
       name: String::from("Bob Vance"),
       id: 1,
       active: true,
        charge_history: vec![],
        current_checkouts: vec![],
    });
   
    let mut Items: Vec<Item> = vec![];
    Items.push(Item {
        name: String::from("Harry Potter"),
        id: 0,
        status: Status::CheckedOut,
        genre: Genre::Fantasy,
        branch: Branch::PointCook,
    });
    Items.push(Item {
        name: String::from("The accidental medium"),
        id: 1,
        status: Status::OnShelf,
        
    });
   
    let mut id_count = 2;
   
    loop{
        println!("1. Create new user");
        println!("2. Display all users");
        println!("3. Display user");
        println!("4. Display Items");
        println!("5. Checkout Item");
        println!("6. Checkin Item")

       
        let mut input_choice = String::new();
        io::stdin()
                    .read_line(&mut input_choice)
                    .expect("Input error");
                   
       
        let trimmed = input_choice.trim();
        match trimmed.parse::<u8>(){
           Ok(i) if i == 1 => Users = add_new_user(Users, id_count),
           Ok(i) if i == 2 => Users = display_users(Users),
           Ok(i) if i == 3 => Users = display_user(Users),
           Ok(i) if i == 4 => Items = display_items(Items),
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
    Non-Fiction, 
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

fn display_items(Items: Vec<Item>) -> Vec<Item>{
    for item in &Items{
        println!("{:?}", item);
    };
  return Items  
}

fn display_user(Users: Vec<User>) -> Vec<User>{
   
    println!("Enter user ID");

    let mut id_input = String::new();
    io::stdin()
            .read_line(&mut id_input)
            .expect("ERROR");
    let id_trimmed = id_input.trim();
   
    match id_trimmed.parse::<u64> (){
        Ok(i) => {
            let mut found = false;
            for user in &Users{
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
   
    return Users
}

fn display_users(Users: Vec<User>) -> Vec<User>{
    for user in &Users{
        println!("{:?}", user);
    };

    return Users;    
}

fn add_new_user(mut Users: Vec<User>, mut id_count: u64) -> Vec<User>{
   
    println!("Name: ");
    let mut name = String::new();
    io::stdin()
                .read_line(&mut name)
                .expect("Error");
               
    Users.push (User{
        name : name,
        id : id_count ,
        active: true,
        charge_history: vec![],
        current_checkouts: vec![],
    });
    id_count +=1;
     return Users;
}