use actix_identity::Identity;
use sha2::{Sha256, Digest};
use lazy_static::lazy_static;
use std::collections::HashSet;
use mongodb::bson::Bson;

use crate::db;
use crate::models::{User, UserInfo};
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
    if username.len() > 20 || username.len() < 5 { return false }
    for i in username.chars() {
        if !VALID_USERNAME_CHARACTER_SET.contains(&i) { return false }
    }
    true
}

fn valid_password(password: &String) -> bool {
    if password.len() > 50 || password.len() < 5 { return false }
    true
}

pub fn create_user
    (form: &UserForm, role: i32) -> Option<Result<Bson, mongodb::error::Error>>
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
    };

    let user_info = UserInfo {
        username: form.username.clone(),        
        role: role,
        clinic: form.clinic.clone(),
    };

    println!("{:?}", user_info);
    
    let _t = db::create::<User>(&user).expect("failed to create user");
    let t = db::create::<UserInfo>(&user_info);
    Some(t)
}

fn remember_user
    (id: &Identity, username: &String, role: i32)
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
    (id: &Identity, role: i32) -> bool
{
    match id.identity() {
        Some(identity) => {
            let role_in_id = &identity.to_string()[..2];
            let role_in_id: i32 = role_in_id.parse::<i32>().unwrap();
            role_in_id == role
        }
        _ => false
    }
}

pub fn check_staff_authority
    (id: &Identity, clinic: bson::oid::ObjectId) -> bool
{
    if let Some(i) = get_identity(id) {
        match i {
            Ok(user) => match user.clinic {
                Some(c) => return c == clinic,
                None => return false                    
            }
            Err(_) => return false
        }
    }
    return false
}

pub fn login
    (form: &UserForm, id: &Identity) -> Option<User>
{
    match db::get::<User, String>(form.username.clone()) {
        Ok(user) => {
            if hash_password(&form.password) == user.password_hash {
                let user_info = db::get::<UserInfo, String>(form.username.clone()).unwrap();
                remember_user(&id, &user.username, user_info.role);
                Some(user)
            } else {
                None
            }
        }
        Err(_) => None
    }
}

pub fn logout
    (id: &Identity)
{
    id.forget();
}

pub fn get_identity
    (id: &Identity)
     -> Option<Result<UserInfo, mongodb::error::Error>>
{
    match id.identity() {
        Some(identity) => {
            let user_id = &identity.to_string()[2..];
            Some(db::get::<UserInfo, String>(user_id.to_string()))
        }
        _ => None
    }
}
