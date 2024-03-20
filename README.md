# Repository for test password-auth crate

## Detail
This crate (https://crates.io/crates/password-auth) that I tried is Password authentication library with a focus on simplicity and ease-of-use, including support for Argon2, PBKDF2, and scrypt password hashing algorithms

## Provided function
### generate_hash()
Generate a password hash for the given password.  
Uses the best available password hashing algorithm given the enabled crate features (typically Argon2 unless explicitly disabled).

this function take password that implmented ```AsRef<[u8]>``` Trait  
```rust
pub fn generate_hash(password: impl AsRef<[u8]>) -> String
```
```AsRef<[u8]>```Trait is implmented for ```str```. So, I should pass &str argument to this function.
```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl AsRef<[u8]> for str {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
```

### verify_password()
Verify the provided password against the provided password hash.  
```rust
pub fn verify_password(
    password: impl AsRef<[u8]>,
    hash: &str
) -> Result<(), VerifyError>
```
Return Ok(()) or VerifyError enum
```rust
pub enum VerifyError {
    Parse(ParseError),
    PasswordInvalid,
}
```

```rust
pub struct ParseError(_);
```