// UserDbEntry Stores daily user data.
#[derive(Debug)]
pub struct UserDbEntry {
    pub id: u64, // User's ID in Telegram.
    pub attempts: u8, // Number of attempts made by the user.
    pub is_won: bool, // Indicates if the user has won.
    pub questions_quantity: u8, // Number of questions asked.
}

// WinnerEntry Stores winners and their remaining ChatGPT questions.
#[derive(Debug)]
pub struct WinnerEntry {
    pub id: u64, // User's ID in Telegram.
    pub requests: u16, // Number of remaining questions for ChatGPT.
}