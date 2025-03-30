pub struct UserInput{
    user_url: String
}

impl UserInput {
    pub fn new(user_url: String) -> Self {
        UserInput {
            user_url,
        }
    }

    pub fn get_user_url(&self) -> &String {
        &self.user_url
    }
    pub fn read_from_console(message: String) -> String {
        println!("{}: ", message);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }
        
}