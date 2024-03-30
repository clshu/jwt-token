use dotenv::dotenv;
use dotenv_codegen::dotenv;

mod jwt;

fn main() {
    dotenv().ok();
    println!("secret: {}", dotenv!("APP_SECRET"));
}
