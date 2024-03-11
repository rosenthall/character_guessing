// Нужно для хранения пользователей в течении дня.
#[derive(Debug)]
pub struct UserDbEntry {
    // Соответствует айди пользователя в телеграме
    pub id: u64,
    // Количество попыток
    pub attempts: u8,
    // Смог ли он победить(учитываются все попытки)
    pub is_won: bool,
    // Количество вопросов
    pub questions_quantity: u8,
}

// Нужно для постоянного хранения победителей и их запаса вопросов к Chatgpt
#[derive(Debug)]
pub struct WinnerEntry {
    // Айди в телеграме
    pub id: u64,
    // Количество оставшихся вопросов к chatgpt.
    pub requests: u16,
}
