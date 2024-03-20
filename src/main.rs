use password_auth::{generate_hash, verify_password};

// generate and verify password that hashed
// The algorithm is Argon2
fn main() {
    // the time of verify take a lot time
    // We should not do this function in current thread in a real application
    // we shoud use tokio::task_blockng()

    // Collect case
    {
        let database_password = "Hello";
        // generate_hash take &str (impl AsRef<[u8]> trait)
        let hashed = generate_hash(database_password);
        println!("{}", hashed);

        let inputed_password = "Hello";
        verify_pass(inputed_password, &hashed);
    }

    // Incollect case
    {
        let database_password = "Rust_is_strong";
        // generate_hash take &str (impl AsRef<[u8]> trait)
        let hashed = generate_hash(database_password);
        println!("{}", hashed);

        let inputed_password = "Rust_is_boring";
        verify_pass(inputed_password, &hashed);
    }
}

fn verify_pass(inputed_password: &str, hashed: &str) {
    // verify_password take password that user input (&str) and password that hashed (e.g. there is database)
    match verify_password(inputed_password, &hashed) {
        Ok(_) => println!("collect"),
        Err(e) => println!("error due to {}", e),
    }
}
