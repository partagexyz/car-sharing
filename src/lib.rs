use std::collections::HashMap;
use calimero_sdk::{
    app,
    borsh::{BorshDeserialize, BorshSerialize},
    env,
};

#[app::event]
pub enum Event {
    CarAdded { car_id: String, owner: String },
    CarRented { car_id: String, user: String, duration: u32 },
    CarReturned { car_id: String },
    UserCreated { user_id: String },
    OwnerCreated { owner_id: String },
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct User {
    user_id: String,
    name: String,
    driving_license: String,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Owner {
    owner_id: String,
    name: String,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Car {
    car_id: String,
    owner_id: String,
    available: bool,
}

#[app::state(emits = Event)]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct CarSharing {
    users: HashMap<String, User>,
    owners: HashMap<String, Owner>,
    cars: HashMap<String, Car>,
}

#[app::logic]
impl CarSharing {
    #[app::init]
    pub fn init() -> Self {
        CarSharing::default()
    }
    pub fn create_owner_account(&mut self, owner_id: String, name: String) -> Result<(), Error> {
        if self.owners.contains_key(&owner_id) {
            return Err(Error::OwnerAlreadyExists);
        }
        
        self.owners.insert(owner_id.clone(), Owner { 
            owner_id: owner_id.clone(), 
            name 
        });
        app::emit!(Event::OwnerCreated { owner_id });
        Ok(())
    }
    pub fn create_user_account(&mut self, user_id: String, name: String, driving_license: String) -> Result<(), Error> {
        if self.users.contains_key(&user_id) {
            return Err(Error::UserAlreadyExists);
        }
        self.users.insert(user_id.clone(), User {
            user_id: user_id.clone(),
            name,
            driving_license,
        });
        app::emit!(Event::UserCreated { user_id });
        Ok(())
    }
    pub fn add_car(&mut self, car_id: String, owner_id: String) -> Result<(), Error> {
        if self.cars.contains_key(&car_id) {
            return Err(Error::CarAlreadyExists);
        }
        if !self.owners.contains_key(&owner_id) {
            return Err(Error::OwnerNotFound);
        }

        self.cars.insert(car_id.clone(), Car {
            car_id: car_id.clone(),
            owner_id: owner_id.clone(),
            available: true,
        });
        app::emit!(Event::CarAdded { car_id: car_id.clone(), owner: owner_id });
        Ok(())
    }

    pub fn rent_car(&mut self, car_id: String, user_id: String, duration: u32) -> Result<(), Error> {
        let car = self.cars.get_mut(&car_id).ok_or(Error::CarNotFound)?;
        if !car.available {
            return Err(Error::CarNotAvailable);
        }
        if !self.users.contains_key(&user_id) {
            return Err(Error::UserNotFound);
        }

        car.available = false;
        app::emit!(Event::CarRented { car_id: car_id.clone(), user: user_id.clone(), duration });
        Ok(())
    }

    pub fn return_car(&mut self, car_id: String) -> Result<(), Error> {
        let car = self.cars.get_mut(&car_id).ok_or(Error::CarNotFound)?;
        car.available = true;
        app::emit!(Event::CarReturned { car_id: car_id.clone() });
        Ok(())
    }

    pub fn list_cars(&self) -> Vec<String> {
        self.cars.keys().cloned().collect()
    }

    pub fn get_car_info(&self, car_id: &String) -> Option<&Car> {
        self.cars.get(car_id)
    }

    pub fn check_car_availability(&self, car_id: &String) -> Result<bool, Error> {
        self.cars.get(car_id)
            .map(|car| car.available)
            .ok_or(Error::CarNotFound)
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidProof,
    UserAlreadyExists,
    OwnerAlreadyExists,
    CarAlreadyExists,
    UserNotFound,
    OwnerNotFound,
    CarNotFound,
    CarNotAvailable,
}