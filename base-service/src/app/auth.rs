use actix_identity::Identity;
use sha2::{Sha256, Digest};
use lazy_static::lazy_static;
use std::collections::HashSet;
use mongodb::bson::Bson;

use crate::db;
use crate::models::User;
use crate::api::form::UserForm;

lazy_static! {
    pub static ref VALID_USERNAME_CHARACTER_SET: HashSet<char> = get_valid_username_character_set();
}

fn hash_password(password: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

fn get_valid_username_character_set() -> HashSet<char> {
    let mut valid_character: HashSet<char> = HashSet::new();
    for i in 'a'..='z' {
        valid_character.insert(i);
    }
    for i in 'A'..='Z' {
        valid_character.insert(i);
    }
    for i in '0'..='9' {
        valid_character.insert(i);
    }
    valid_character.insert('.');
    valid_character.insert('_');
    valid_character
}

fn valid_username(username: &String) -> bool {
    if username.len() > 20 || username.len() < 6 { return false }
    for i in username.chars() {
        if !VALID_USERNAME_CHARACTER_SET.contains(&i) { return false }
    }
    true
}

fn valid_password(password: &String) -> bool {
    if password.len() > 50 || password.len() < 8 { return false }
    println!("{}", password.len());    
    true
}

fn create_user
    (form: &UserForm, role: u8) -> Option<Result<Bson, mongodb::error::Error>>
{
    if !valid_username(&form.username) {
        return None
    }

    if !valid_password(&form.username) {
        return None
    }
    
    let user = User {
        username: form.username.clone(),
        password_hash: hash_password(&form.password),
        role: role,
    };

    Some(db::create::<User>(&user))
}

pub fn create_admin_user
    (id: &Identity, form: &UserForm) -> Option<Result<Bson, mongodb::error::Error>>
{
    if check_role(id, 0) {
        create_user(form, 0)
    } else {
        None
    }
}

pub fn create_staff_user
    (id: &Identity, form: &UserForm) -> Option<Result<Bson, mongodb::error::Error>>
{
    if check_role(id, 0) {
        create_user(form, 0)
    } else {
        None
    }        
}

fn remember_user
    (id: &Identity, username: &String, role: u8)
{
    match role {
        0 => {
            id.remember(format!("{}{}", "00", username));
        }
        1 => {
            id.remember(format!("{}{}", "01", username));
        }
        _ => {}
    }
}

pub fn check_role
    (id: &Identity, role: u8) -> bool
{
    match id.identity() {
        Some(identity) => {
            let role_in_id = &identity.to_string()[..2];
            let role_in_id: u8 = role_in_id.parse::<u8>().unwrap();
            role_in_id == role
        }
        _ => false
    }
}

pub fn login
    (form: &UserForm, id: &Identity) -> Option<User>
{
    match db::get::<User, String>(form.username.clone()) {
        Ok(user) => {
            if hash_password(&form.password) == user.password_hash {
                remember_user(&id, &user.username, user.role);
                Some(user)
            } else {
                None
            }
        }
        Err(_) => {
            None
        }
    }
}

pub fn logout
    (id: &Identity)
{
    id.forget();
}
