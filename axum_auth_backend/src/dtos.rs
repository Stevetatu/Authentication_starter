use core::str;
use chrono::{Datetime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{User, UserRole};

#[derive(Validate, Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegisterUserDto {

    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
        #[validate(
            length(min = 1, message = "email is required"),
            email(message = "Invalid email format")
        )]

    #[validate(length(min = 6, max = 100))]
    pub password: String,
        #[validate(length(min = 1, message = "Confirm password is required"),
        must_match(other = "password", message = "Passwords do not match")

    )]

    #[validate(email)]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters long")
    )]

    #[serde(rename = "passwordconfirm")]
    pub password_confirm: String,
    
}

#[derive(Validate, Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginUserDto {
    #[validate(length(min = 1, message = "Email is required"), email(message = "Invalid email format"))]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters long")
    )]

    pub password: String,
}

#[derive(Validate, Debug, Clone, Serialize, Deserialize)]
pu struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

pub stuct FilterUserDto {
    pub name: String>,
    pub email: String,
    pub role: String,
    #[serde(rename = "createdAt")]
    pub created_at: Datetime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Datetime<Utc>,
}

impl FilterUserDto{
    pub fn filter_user(user: &User) -> FilterUserDto {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            verified: user.verified,
            role: user.role.to_str().to_string(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }

    pub fn filter_users(users: &[User]) -> Vec<FilterUserDto> {
        users.iter().map(FilterUserDto::filter_user).collect()
    }
}

#[derive(Validate, Debug, Clone, Serialize, Deserialize)]
pub struct UserData{
    pub user: FilterUserDto,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Validate, Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response{
    pub status: &'static str,
    pub message: String,
} 

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct NameUdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct RoleUpdateDto {
    #[validate(length(min = 1, message = "Role is required"))]
    pub role: UserRole,
}

fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError>{
    match role{
        UserRole::Admin | UserRole::User => Ok(()),
        _ => Err(validator::ValidationError::new("Invalid user role")),
    }
}

#[derive(Default, Serialize, Deserialize, Validate, Debug, Clone)]
pub struct UserPasswordUpdateDto{
    #[validate(
        length(min = 1, message = "New password is required"),
        length(min = 6, message = "New password must be at least 6 characters long")
    )]
    pub new_password: String,

    #[validate(
        length(min = 1, message = "Confirm new password is required"),
        length(min = 6, message = "Confirm new password must be at least 6 characters long"),
        must_match(other = "new_password", message = "Passwords do not match")
    )]
    pub new_password_confirm: String,

    #[validate(
        length(min = 1, message = "Old password is required"),
        length(min = 6, message = "Old password must be at least 6 characters long")
    )]
    pub old_password: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct VerifyEmailQueryDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct ForgotPasswordDto {
    #[validate(length(min = 1, message = "Email is required"), email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct ResetPasswordRequestDto {
    #[validate(length(min = 1, message = "Token is required"))]
    pub token: String,

    #[validate(
        length(min = 1, message = "New password is required"), 
        length(min = 6, message = "New password must be at least 6 characters long")
    )]
    pub new_password_confirm: String,
}