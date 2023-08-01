#[derive(Debug)]
pub struct User {
    // Соответствует айди пользователя в телеграме
    pub id: u32,
    // Количество попыток
    pub attempts: u8,
    // Смог ли он победить(учитываются все попытки)
    pub is_won: bool,
    // Количество вопросов
    pub questions_quantity: u8,
}
