pub mod login_strategy;
pub(crate) mod router;

#[test]
pub fn test() {
    println!("{}", password_auth::generate_hash("1234qwer"));
}
