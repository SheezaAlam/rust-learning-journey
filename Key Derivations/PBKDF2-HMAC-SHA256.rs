use pbkdf2::pbkdf2_hmac;
use hmac::Hmac;
use sha2::Sha256;

fn main() {
    // password and salt (normally user input and random salt)
    let password = b"mypassword";
    let salt = b"unique_salt";

    // key length and iterations
    let iterations = 100_000;
    let mut key = [0u8; 32]; // 32-byte derived key for AES-256

    // Run PBKDF2 with HMAC-SHA256
    pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut key);

    println!("Derived key: {:x?}", key);
}
/*output
Derived key: [60, 77, f, 1a, c1, 5d, d, 5f, 81, 40, a1, e9, 80, 63, 9c, 76, 24, c, 19, cf, d8, f9, 28, 65, 69, b1, 7d, f3, 5c, 81, c4, 2a]*/
What PBKDF2 Does

PBKDF2 takes:

A password (user input, weak),

A random salt (prevents rainbow table attacks),

Many iterations (slows brute force),

A chosen output length (we control this).

And outputs:

A sequence of bytes of exactly the length you request.
