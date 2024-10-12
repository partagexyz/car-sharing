use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use calimero_sdk::{app, borsh::{BorshDeserialize, BorshSerialize}};

use near_sdk::env;
use near_sdk::AccountId;
use near_sdk::env::predecessor_account_id;
//use near_sdk::env::attached_deposit;
use near_sdk::env::block_timestamp;
use near_token::NearToken;

#[app::event]
pub enum Event {
    CarAdded { car_id: String, owner: String },
    CarRented { car_id: String, user: String, duration: u32 },
    CarReturned { car_id: String },
    UserCreated { user_id: String },
    OwnerCreated { owner_id: String },
    CarBooked { car_id: String, user: String, start_time: u64, end_time: u64, deposit: u128 },
    BookingCancelled { booking_id: String, user: String, deposit_retained: u128 },
}

#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct User {
    user_id: String,
    name: String,
    driving_license: String,
}
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Owner {
    owner_id: String,
    name: String,
}
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[derive(Clone)]
pub struct Car {
    car_id: String,
    owner_id: String,
    available: bool,
    hourly_rate: u128,
}
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Booking {
    booking_id: String,
    car_id: String,
    user_id: String,
    start_time: u64,
    end_time: u64,
    deposit: u128,
}

#[app::state(emits = Event)]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
pub struct CarSharing {
    users: HashMap<String, User>,
    owners: HashMap<String, Owner>,
    cars: HashMap<String, Car>,
    bookings: HashMap<String, Booking>,
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
    pub fn add_car(&mut self, car_id: String, owner_id: String, hourly_rate: u128) -> Result<(), Error> {
        // Ensure caller has permission to add a car
        let caller = predecessor_account_id();
        if !self.is_owner(&caller) {
            return Err(Error::Unauthorized);
        }
        if self.cars.contains_key(&car_id) {
            return Err(Error::CarAlreadyExists);
        }
        if !self.owners.contains_key(&owner_id) {
            return Err(Error::OwnerNotFound);
        }
        // Validate the hourly rate to prevent invalid inputs
        if hourly_rate == 0 {
            return Err(Error::InvalidHourlyRate);
        }
        self.cars.insert(car_id.clone(), Car {
            car_id: car_id.clone(),
            owner_id: owner_id.clone(),
            available: true,
            hourly_rate,
        });
        app::emit!(Event::CarAdded { car_id: car_id.clone(), owner: owner_id.clone() });
        Ok(())
    }
    // book_car allows users to book a car in advance with a deposit
    pub fn book_car(&mut self, car_id: String, user_id: String, start_time: u64, end_time: u64, deposit: NearToken) -> Result<(), Error> {
        // Ensure the car exists, driver is valid, and car is available
        if !self.cars.contains_key(&car_id) {
            return Err(Error::CarNotFound);
        }
        if !self.is_valid_driver(&user_id) {
            return Err(Error::InvalidDriver);
        }
        if start_time >= end_time {
            return Err(Error::InvalidBookingTime);
        }
        let car = self.cars.get(&car_id).ok_or(Error::CarNotFound)?;
        if !car.available {
            return Err(Error::CarNotAvailable);
        }
        // Ensure the car is not already booked for this period
        if self.is_car_booked(&car_id, start_time, end_time) {
            return Err(Error::CarNotAvailable);
        }
        // Calculate deposit required (10% of rental fee)
        let rental_duration: u64 = (end_time - start_time) / 3600000000000; // Convert to hours
        let rental_fee: u128 = self.calculate_rental_fee(&car_id, rental_duration as u32)?;
        let deposit_amount: NearToken = NearToken::from_yoctonear((rental_fee / 10) * 9); // 10% of rental fee
        //let attached_deposit: NearToken = NearToken::from_yoctonear(env::attached_deposit());
        // Check if enough deposit was attached
        if deposit < deposit_amount {
            return Err(Error::InsufficientDeposit);
        }
        // Generate a unique booking ID
        let booking_id = format!("{}-{}-{}", car_id, user_id, start_time);
        // Create booking
        self.bookings.insert(booking_id.clone(), Booking {
            booking_id: booking_id.clone(),
            car_id: car_id.clone(),
            user_id: user_id.clone(),
            start_time,
            end_time,
            deposit: deposit.as_yoctonear(),
        });
        // Emit event
        app::emit!(Event::CarBooked { car_id, user: user_id.clone(), start_time, end_time, deposit: deposit_amount.as_yoctonear() });
        Ok(())
    }
    pub fn cancel_booking(&mut self, booking_id: String) -> Result<(), Error> {
        if let Some(booking) = self.bookings.remove(&booking_id) {
            let _car_id = booking.car_id.clone();
            let user_id = booking.user_id.clone();
            let deposit = booking.deposit;
            // No refund is processed: the 10% deposit is retained
            app::emit!(Event::BookingCancelled {
                booking_id: booking_id.clone(),
                user: user_id.clone(),
                deposit_retained: deposit
            });
            Ok(())
        } else {
            Err(Error::BookingNotFound)
        }
    }
    // rent_car allows users to rent a car immediately with payment
    pub fn rent_car(&mut self, car_id: String, user_id: String, duration: u32) -> Result<(), Error> {
        // Ensure the user has a valid license
        if !self.is_valid_driver(&user_id) {
            return Err(Error::InvalidDriver);
        }
        // Calculate end_time based on current time and duration
        let start_time = env::block_timestamp(); //check that block_timestamp returns values in nanoseconds
        let end_time = start_time + (duration as u64 * 3600000000000); // convert duration in hours to nanoseconds
        // Ensure required payment is attached
        let required_deposit: NearToken = NearToken::from_yoctonear(self.calculate_rental_fee(&car_id, duration)?);
        let attached_deposit: NearToken = env::attached_deposit().into();

        if attached_deposit < required_deposit {
            return Err(Error::InsufficientPayment);
        }
        // get the car reference before any mutable operation on self
        let mut car: &mut Car = &mut self.cars.get(&car_id).ok_or(Error::CarNotFound)?.clone();
        // Ensure rental conditions are met
        if !car.available {
            return Err(Error::CarNotAvailable);
        }
        if !self.users.contains_key(&user_id) {
            return Err(Error::UserNotFound);
        }
        // Make the booking before changing car status
        self.book_car(car_id.clone(), user_id.clone(), start_time, end_time, attached_deposit.clone())?;
        // Change car status in self.cars
        car.available = false;
        // Emit the rent event
        app::emit!(Event::CarRented { car_id: car_id.clone(), user: user_id.clone(), duration });
        Ok(())
    }
    pub fn return_car(&mut self, car_id: String) -> Result<(), Error> {
        let now = block_timestamp();
        let car = self.cars.get_mut(&car_id).ok_or(Error::CarNotFound)?;
        car.available = true;
        // Cancel the booking that corresponds to the current rental
        if let Some(booking) = self.bookings.iter().find(|(_, b)| b.car_id == car_id && now >= b.start_time && now <= b.end_time) {
            let booking_id = booking.0.clone();
            let user_id = booking.1.user_id.clone();
            let deposit = booking.1.deposit;
            self.bookings.remove(&booking_id);
            app::emit!(Event::BookingCancelled { 
                booking_id: booking_id.clone(), 
                user: user_id.clone(),
                deposit_retained: deposit 
            });
        }
        app::emit!(Event::CarReturned { car_id: car_id.clone() });
        Ok(())
    }
    pub fn list_cars(&self) -> Vec<String> {
        self.cars.keys().cloned().collect()
    }
    pub fn get_car_info(&self, car_id: String) -> Option<&Car> {
        self.cars.get(&car_id)
    }
    pub fn check_car_availability(&self, car_id: String) -> Result<bool, Error> {
        self.cars.get(&car_id)
            .map(|car| car.available)
            .ok_or(Error::CarNotFound)
    }
    // Helper methods, private functions
    fn is_owner(&self, account_id: &AccountId) -> bool {
        self.owners.contains_key(&account_id.to_string())
    }
    fn is_valid_driver(&self, user_id: &String) -> bool {
        self.users.get(user_id).map(|user| !user.driving_license.is_empty()).unwrap_or(false)
    }
    fn calculate_rental_fee(&self, car_id: &str, duration: u32) -> Result<u128, Error> {
        let car = self.cars.get(car_id).ok_or(Error::CarNotFound)?;
        Ok((duration as u128) * car.hourly_rate)
    }
    fn is_car_booked(&self, car_id: &str, start_time: u64, end_time: u64) -> bool {
        self.bookings.values().any(|booking| {
            booking.car_id == car_id && 
            ((start_time >= booking.start_time && start_time < booking.end_time) ||
             (end_time > booking.start_time && end_time <= booking.end_time) ||
             (start_time <= booking.start_time && end_time >= booking.end_time))
        })
    }    
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    InvalidProof,
    UserAlreadyExists,
    OwnerAlreadyExists,
    CarAlreadyExists,
    UserNotFound,
    OwnerNotFound,
    CarNotFound,
    CarNotAvailable,
    InsufficientDeposit,
    InsufficientPayment,
    Unauthorized,
    InvalidDriver,
    InvalidHourlyRate,
    InvalidBookingTime,
    BookingNotFound,
}