mod model;

// Если пользователь есть в актуальной дб - возвращает структуру User, в противном случае возвращает None
pub async fn check_user(id : u8) -> Option<model::User> {
    todo!()
}

// Функция добавляет пользователя в базу данных, в случае ошибки - возвращает Err()
pub async fn try_add_user(user : model::User) -> Result<(), Err(e)> {
    todo!();
}
