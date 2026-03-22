pub fn is_authorized(user_id: i64, super_user_id: i64) -> bool {
    user_id == super_user_id
}
