use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use near_sdk::env;
use near_sdk::env::{attached_deposit, block_timestamp, predecessor_account_id};
use near_sdk::{AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};
use near_sdk::log;
use near_token::NearToken;

#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct User {
    pub user_id: String,
    pub name: String,
    driving_license: String,
}
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Owner {
    pub owner_id: String,
    pub name: String,
}
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct Car {
    pub car_id: String,
    pub owner_id: String,
    pub available: bool,
    pub hourly_rate: u128,
    // add vehicle licence or registration certificate (carte grise)
}
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct Booking {
    pub booking_id: String,
    car_id: String,
    user_id: String,
    start_time: u64,
    end_time: u64,
    deposit: u128,
}

#[near_bindgen]
#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct CarSharing {
    pub users: HashMap<String, User>,
    pub owners: HashMap<String, Owner>,
    pub cars: HashMap<String, Car>,
    pub bookings: HashMap<String, Booking>,
    pub users_accounts: Vec<AccountId>,
    pub owners_accounts: Vec<AccountId>,
}

#[near_bindgen]
impl CarSharing {
    #[init]
    pub fn init() -> Self {
        Self::default() //initialized the contract with all fields in structure set to default values
    }
    #[handle_result]
    pub fn create_owner_account(&mut self, owner_id: String, name: String) -> Result<(), Error> {
        if self.owners.contains_key(&owner_id) {
            return Err(Error::OwnerAlreadyExists);
        }
        // store owner information
        let account_id: AccountId = owner_id.parse().map_err(|_| Error::InvalidAccountId)?;
        self.owners.insert(
            owner_id.clone(),
            Owner {
                owner_id: owner_id.clone(),
                name,
            },
        );
        self.owners_accounts.push(account_id);
        log!("Event::OwnerCreated, owner_id: {}", owner_id);
        Ok(())
    }

    #[handle_result]
    pub fn create_user_account(
        &mut self,
        user_id: String,
        name: String,
        driving_license: String,
    ) -> Result<(), Error> {
        if self.users.contains_key(&user_id) {
            return Err(Error::UserAlreadyExists);
        }
        // store user information
        let account_id: AccountId = user_id.parse().map_err(|_| Error::InvalidAccountId)?;

        self.users.insert(
            user_id.clone(),
            User {
                user_id: user_id.clone(),
                name,
                driving_license,
            },
        );
        self.users_accounts.push(account_id);
        log!("Event::UserCreated, user_id: {}", user_id);
        Ok(())
    }

    #[handle_result]
    pub fn add_car(
        &mut self,
        car_id: String,
        owner_id: String,
        hourly_rate: u128,
    ) -> Result<(), Error> {
        // Ensure caller has permission to add a car
        let caller: AccountId = predecessor_account_id();
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
        self.cars.insert(
            car_id.clone(),
            Car {
                car_id: car_id.clone(),
                owner_id: owner_id.clone(),
                available: true,
                hourly_rate,
            },
        );
        log!("Event: CarAdded, car_id: {}, owner: {}", car_id.clone(), owner_id.clone());
        Ok(())
    }

    // delete_car allows owners to remove a car from the system
    #[handle_result]
    pub fn delete_car(&mut self, car_id: String) -> Result<(), Error> {
        // get the caller account id
        let caller = predecessor_account_id().to_string();
        // retrieve the car to check its ownership
        let car = self.cars.get(&car_id).ok_or(Error::CarNotFound)?;
        // Ensure the caller is the owner of the car
        if car.owner_id != caller {
            return Err(Error::Unauthorized);
        }
        // remove the car from the mapping
        self.cars.remove(&car_id);
        log!("Event: Car deleted, car_id: {}", car_id);
        Ok(())
    }

    // book_car allows users to book a car in advance with a deposit
    #[payable]
    #[handle_result]
    pub fn book_car(
        &mut self,
        car_id: String,
        user_id: String,
        start_time: u64,
        end_time: u64,
        deposit: NearToken,
    ) -> Result<(), Error> {
        // Convert user_id to AccountId
        let user_account_id: AccountId = user_id.parse().map_err(|_| Error::InvalidAccountId)?;
        // Ensure the driver is valid, the car exists, and is available
        if !self.is_user(&user_account_id) {
            return Err(Error::InvalidUser);
        }
        if !self.cars.contains_key(&car_id) {
            return Err(Error::CarNotFound);
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
        let booking_id: String = format!("{}-{}-{}", car_id, user_id, start_time);
        // Create booking
        self.bookings.insert(
            booking_id.clone(),
            Booking {
                booking_id: booking_id.clone(),
                car_id: car_id.clone(),
                user_id: user_id.clone(),
                start_time,
                end_time,
                deposit: deposit.as_yoctonear(),
            },
        );
        // Emit event
        log!("Event: CarBooked, car_id: {}, user: {}, start_time: {}, end_time: {}, deposit: {}", car_id, user_id.clone(), start_time, end_time, deposit_amount.as_yoctonear());
        Ok(())
    }

    #[payable]
    #[handle_result]
    pub fn cancel_booking(&mut self, booking_id: String) -> Result<(), Error> {
        if let Some(booking) = self.bookings.remove(&booking_id) {
            let _car_id: String = booking.car_id.clone();
            let user_id: String = booking.user_id.clone();
            let deposit: u128 = booking.deposit;
            // No refund is processed: the 10% deposit is retained
            log!("Event: BookingCancelled, booking_id: {}, user: {}, deposit_retained: {}", booking_id.clone(), user_id.clone(), deposit);
            Ok(())
        } else {
            Err(Error::BookingNotFound)
        }
    }

    // rent_car allows users to rent a car immediately with payment
    #[payable]
    #[handle_result]
    pub fn rent_car(
        &mut self,
        car_id: String,
        user_id: String,
        duration: u32,
    ) -> Result<(), Error> {
        // Convert user_id to AccountId
        let user_account_id: AccountId = user_id.parse().map_err(|_| Error::InvalidAccountId)?;
        // Ensure the caller has a valid user account w/ driving license
        if !self.is_user(&user_account_id) {
            return Err(Error::InvalidUser);
        }
        // Calculate end_time based on current time and duration
        let start_time: u64 = block_timestamp(); //check that block_timestamp returns values in nanoseconds
        let end_time: u64 = start_time + (duration as u64 * 3600000000000); // convert duration in hours to nanoseconds

        // Ensure required payment is attached
        let required_deposit: NearToken = NearToken::from_yoctonear(self.calculate_rental_fee(&car_id, duration)?);
        let attached_deposit: NearToken = attached_deposit().into();
        if attached_deposit < required_deposit {
            return Err(Error::InsufficientPayment);
        }

        // Check if the car is available
        let car: &mut Car = self.cars.get_mut(&car_id).ok_or(Error::CarNotFound)?;
        if !car.available {
            return Err(Error::CarNotAvailable);
        }
        // Mark car as unavailable
        car.available = false;

        let booking_id = format!("{}-{}-{}", car_id, user_id.to_string(), start_time);
        let deposit = env::attached_deposit();
        self.book_car(
            car_id.clone(),
            user_id.clone(),
            start_time,
            end_time,
            attached_deposit.clone(),
        )?;
        // Emit the rent event
        log!("Event: CarRented, car_id: {}, user: {}, duration: {}", car_id.clone(), user_id.clone(), duration);
        Ok(())
    }

    #[payable]
    #[handle_result]
    pub fn return_car(&mut self, car_id: String) -> Result<(), Error> {
        let now: u64 = block_timestamp();
        let car: &mut Car = self.cars.get_mut(&car_id).ok_or(Error::CarNotFound)?;
        car.available = true;
        // Cancel the booking that corresponds to the current rental
        if let Some(booking) = self
            .bookings
            .iter()
            .find(|(_, b)| b.car_id == car_id && now >= b.start_time && now <= b.end_time)
        {
            let booking_id: String = booking.0.clone();
            let user_id: String = booking.1.user_id.clone();
            let deposit: u128 = booking.1.deposit;
            self.bookings.remove(&booking_id);
            log!("Event: BookingCancelled, booking_id: {}, user: {}, deposit_retained: {}", booking_id.clone(), user_id.clone(), deposit);
        }
        log!("Event: CarReturned, car_id: {}", car_id.clone());
        Ok(())
    }

    // Helper functions
    pub fn is_owner(&self, account_id: &AccountId) -> bool {
        self.owners_accounts.contains(account_id)
    }

    pub fn is_user(&self, account_id: &AccountId) -> bool {
        self.users_accounts.contains(account_id)
    }

    #[handle_result]
    pub fn calculate_rental_fee(&self, car_id: &str, duration: u32) -> Result<u128, Error> {
        let car: &Car = self.cars.get(car_id).ok_or(Error::CarNotFound)?;
        Ok((duration as u128) * car.hourly_rate)
    }

    pub fn is_car_booked(&self, car_id: &str, start_time: u64, end_time: u64) -> bool {
        self.bookings.values().any(|booking: &Booking| {
            booking.car_id == car_id
                && ((start_time >= booking.start_time && start_time < booking.end_time)
                    || (end_time > booking.start_time && end_time <= booking.end_time)
                    || (start_time <= booking.start_time && end_time >= booking.end_time))
        })
    }

    // read-only functions
    pub fn list_owner_cars(&self, owner_id: String) -> Vec<Car> {
        self.cars
            .values()
            .cloned()
            .filter(|car| car.owner_id == owner_id)
            .collect()
    }
    pub fn list_available_cars(&self) -> Vec<Car> {
        self.cars
            .values()
            .cloned()
            .filter(|car| car.available)
            .collect()
    }
    pub fn list_user_bookings(&self, user_id: String) -> Vec<Booking> {
        self.bookings
            .values()
            .filter(|b| b.user_id == user_id)
            .cloned()
            .collect()
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
    InvalidUser,
    InvalidHourlyRate,
    InvalidBookingTime,
    BookingNotFound,
    InvalidAccountId,
}