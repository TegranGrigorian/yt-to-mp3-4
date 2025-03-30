pub struct UserInput{
    user_url: String
}

impl UserInput {
    // Constructor to create a new instance of UserInput
    pub fn new(user_url: String) -> Self {
        UserInput {
            user_url,
        }
    }
    // Method to get the user URL
    pub fn get_user_url(&self) -> &String {
        &self.user_url
    }
    pub fn read_from_console(message: String) -> String {
        // This method can be used to read user input from the console
        println!("{}: ", message); // Print the message to prompt the user for input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        // Trim the input to remove any trailing newline characters
        input.trim().to_string()
    }
    
    // You can add more methods here to handle user input as needed
    
}