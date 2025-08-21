// File contains code for hashing and verifying passwords using argon2 algorithm
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, 
        PasswordHasher, 
        PasswordVerifier, 
        SaltString
    },
    Argon2,
}