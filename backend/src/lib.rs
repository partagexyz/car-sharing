use calimero_sdk::{
    app,
    borsh::{BorshDeserialize, BorshSerialize},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use near_sdk::AccountId;
use near_sdk::env::{
    attached_deposit,
    block_timestamp,
    predecessor_account_id,
};
use near_token::NearToken;

#[app::event]
pub enum Event {
    OwnerCreated {
        owner_id: String,
    },
    UserCreated {
        user_id: String,
    },
    CarAdded {
        car_id: String,
        owner: String,
    },
    CarDeleted {
        car_id: String,
    },
    CarBooked {
        car_id: String,
        user: String,
        start_time: u64,
        end_time: u64,
        deposit: u128,
    },
    BookingCancelled {
        booking_id: String,
        user: String,
        deposit_retained: u128,
    },
    CarRented {
        car_id: String,
        user: String,
        duration: u32,
    },
    CarReturned {
        car_id: String,
    },
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
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct Car {
    car_id: String,
    owner_id: String,
    available: bool,
    hourly_rate: u128,
    // add vehicle licence or registration certificate (carte grise)
}
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
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
    users_accounts: Vec<AccountId>,
    owners_accounts: Vec<AccountId>,
}

#[app::logic]
impl CarSharing {
    #[app::init]
    pub fn init() -> Self {
        CarSharing::default()
    }

    pub fn create_owner_account(
        &mut self, 
        owner_id: String, 
        name: String
    ) -> Result<(), Error> {
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
        app::emit!(Event::OwnerCreated { owner_id });
        Ok(())
    }

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
        app::emit!(Event::UserCreated { user_id });
        Ok(())
    }

    pub fn add_car(
        &mut self,
        car_id: String,
        owner_id: String,
        hourly_rate: u128,
    ) -> Result<(), Error> {
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
        self.cars.insert(
            car_id.clone(),
            Car {
                car_id: car_id.clone(),
                owner_id: owner_id.clone(),
                available: true,
                hourly_rate,
            },
        );
        app::emit!(Event::CarAdded {
            car_id: car_id.clone(),
            owner: owner_id.clone()
        });
        Ok(())
    }

    // delete_car allows owners to remove a car from the system
    pub fn delete_car(
        &mut self, 
        car_id: String
    ) -> Result<(), Error> {
        // Ensure caller has permission to delete a car
        let caller = predecessor_account_id();
        if !self.is_owner(&caller) {
            return Err(Error::Unauthorized);
        }
        if self.cars.remove(&car_id).is_none() {
            return Err(Error::CarNotFound);
        }
        app::emit!(Event::CarDeleted {
            car_id: car_id.clone()
        });
        Ok(())
    }

    // book_car allows users to book a car in advance with a deposit
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
            return Err(Error::InvalidDriver);
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
        let booking_id = format!("{}-{}-{}", car_id, user_id, start_time);
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
        app::emit!(Event::CarBooked {
            car_id,
            user: user_id.clone(),
            start_time,
            end_time,
            deposit: deposit_amount.as_yoctonear()
        });
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
    pub fn rent_car(
        &mut self,
        car_id: String,
        user_id: String,
        duration: u32,
    ) -> Result<(), Error> {
        // Convert user_id to AccountId
        let user_account_id: AccountId = user_id.parse().map_err(|_| Error::InvalidAccountId)?;

        // Ensure the user has a valid license
        if !self.is_user(&user_account_id) {
            return Err(Error::InvalidDriver);
        }
        // Calculate end_time based on current time and duration
        let start_time = block_timestamp(); //check that block_timestamp returns values in nanoseconds
        let end_time = start_time + (duration as u64 * 3600000000000); // convert duration in hours to nanoseconds
                                                                       // Ensure required payment is attached
        let required_deposit: NearToken =
            NearToken::from_yoctonear(self.calculate_rental_fee(&car_id, duration)?);
        let attached_deposit: NearToken = attached_deposit().into();

        if attached_deposit < required_deposit {
            return Err(Error::InsufficientPayment);
        }
        // get the car reference before any mutable operation on self
        let mut car: Car = self.cars.get(&car_id).ok_or(Error::CarNotFound)?.clone();
        // Ensure rental conditions are met
        if !car.available {
            return Err(Error::CarNotAvailable);
        }
        if !self.users.contains_key(&user_id) {
            return Err(Error::UserNotFound);
        }
        // Make the booking before changing car status
        self.book_car(
            car_id.clone(),
            user_id.clone(),
            start_time,
            end_time,
            attached_deposit.clone(),
        )?;
        // Change car status in self.cars
        car.available = false;
        // Emit the rent event
        app::emit!(Event::CarRented {
            car_id: car_id.clone(),
            user: user_id.clone(),
            duration
        });
        Ok(())
    }

    pub fn return_car(&mut self, car_id: String) -> Result<(), Error> {
        let now = block_timestamp();
        let car = self.cars.get_mut(&car_id).ok_or(Error::CarNotFound)?;
        car.available = true;
        // Cancel the booking that corresponds to the current rental
        if let Some(booking) = self
            .bookings
            .iter()
            .find(|(_, b)| b.car_id == car_id && now >= b.start_time && now <= b.end_time)
        {
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
        app::emit!(Event::CarReturned {
            car_id: car_id.clone()
        });
        Ok(())
    }

    // Helper functions
    fn is_owner(&self, account_id: &AccountId) -> bool {
        self.owners_accounts.contains(account_id)
    }

    fn is_user(&self, account_id: &AccountId) -> bool {
        self.users_accounts.contains(account_id)
    }

    fn calculate_rental_fee(&self, car_id: &str, duration: u32) -> Result<u128, Error> {
        let car = self.cars.get(car_id).ok_or(Error::CarNotFound)?;
        Ok((duration as u128) * car.hourly_rate)
    }

    fn is_car_booked(
        &self, 
        car_id: &str, 
        start_time: u64, 
        end_time: u64
    ) -> bool {
        self.bookings.values().any(|booking| {
            booking.car_id == car_id
                && ((start_time >= booking.start_time && start_time < booking.end_time)
                    || (end_time > booking.start_time && end_time <= booking.end_time)
                    || (start_time <= booking.start_time && end_time >= booking.end_time))
        })
    }
    
    // read-only functions
    pub fn list_owner_cars(&self, owner_id: String) -> Vec<Car> {
        self.cars.values().cloned().filter(|car| car.owner_id == owner_id).collect()
    }
    pub fn list_avalaible_cars(&self) -> Vec<Car> {
        self.cars.values().cloned().filter(|car| car.available).collect()
    }
    pub fn list_user_bookings(&self, user_id: String) -> Vec<Booking> {
        self.bookings.values().filter(|b| b.user_id == user_id).cloned().collect()
    }
    /*
    pub fn get_car_info(&self, car_id: String) -> Option<&Car> {
        self.cars.get(&car_id)
    }
    pub fn check_car_availability(&self, car_id: String) -> Result<bool, Error> {
        self.cars.get(&car_id)
            .map(|car| car.available)
            .ok_or(Error::CarNotFound)
    }
    */
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
    InvalidAccountId,
}
