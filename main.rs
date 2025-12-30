use std::io;
use std::io::Read;
use std::io::Write;


fn get_string_input(prompt:&str) -> String {
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut result:String = input.trim().to_string();
    return result;
}

fn validate_password(password: &str) -> bool {
    let mut has_letter = false;
    let mut has_number = false;
    let mut has_special = false;

    for c in password.chars() {
        if c.is_alphabetic() {
            has_letter = true;
        }else if c.is_numeric() {
            has_number = true;
        }else if c.is_ascii_punctuation() || c.is_ascii_graphic() && !c.is_alphanumeric() {
            // is _ascii_punctuation covers common symbols like !@#$%^&*
            //is_ascii_graphic covers all printabl echaracters except space

            has_special = true;
        }
    }

    //Checking if all conditions are met
    return has_letter && has_number && has_special;
}


fn validate_username(username: &str) -> bool { // Validate susername inputed
    let mut has_letter = false;
    let mut has_number = false;

    for i in username.chars() {
        if i.is_alphabetic() {
            has_letter = true;
        }else if i.is_numeric() {
            has_number = true;
        }
    }
    return has_letter && has_number;
}

fn main() {
    println!("CBT software is now online...\n");
    let mut username:String = String::new();
    let mut password:String = String::new();

    let email:String =String::new();
    
    
    
    loop {
        loop {
        let username = get_string_input("Enter your Matriculation number. Note that matriculation number can only contain letters and numbers\n").to_uppercase();
        
        println!("");  
        println!("Matriculation number:{} , Valid:{}",username ,validate_username(&username)); //Referenced the value gotten from the user iinput -> Username
        println!("");

        if validate_username(&username) {
            break;  
        }else {
            println!("Invalid matriculation number!. Be sure that your username contains letters and numbers only");
          }
        }
            
        println!("Is this matriculation number correct or you'll like to change it ?\n");
        println!("Input Y to correct it or N to proceed");

        let mut redo = String::new();
        io::stdin().read_line(&mut redo).expect("Failed to read input");
        let redo:String = redo.trim().to_uppercase();
        if redo == "Y" {
            continue;
        }else if redo == "N" {
            break;
        }else {
            println!("Invalid input. Be sure that your input is either Y or N. Starting over...")
        }
      
      }



    let email_domain = "pau.edu.ng";
    loop{
        let email = get_string_input("\nEnter your PAU email").to_lowercase();
        let parts_of_email:Vec<&str> = email.split('@').collect();
        let domain = parts_of_email[1];

        if email.contains('@')  && email_domain.contains(&domain) {
            break;
        }

        else {
            println!("Invalid email . Be sure to put an '@' and check spellings of the domain");
        }
    }

    
    loop{
    let password = get_string_input("\nEnter your password. Note that password must contain letters, numbers and special characters\n");
       println!("Password:{} , Valid:{}",password ,validate_password(&password)); //Referenced the value gotten from password

        if validate_password(&password) == true {
            break;
        }
        else {
            println!("Invalid password! . Be sure password contains letters, numbers and special characters");
        }
    }

    /*let mut file = std::fs::File::open("christiantheologyCBT.png").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
*/


}

