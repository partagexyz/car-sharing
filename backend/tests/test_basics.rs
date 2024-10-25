use car_sharing::CarSharing;
use near_sdk::{testing_env, AccountId, Gas};
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_token::NearToken;

// Mocking the VM context for testing purposes
fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}

// Helper function to initialize the contract for testing
fn init_contract() -> CarSharing {
    let context = get_context(accounts(0)).build();
    testing_env!(context);
    CarSharing::init()
}

#[tokio::test]
async fn test_car_sharing_initialization() {
    let contract = init_contract();

    // Check that initial state is as expected
    assert_eq!(contract.users.len(), 0, "Users map should be empty initially");
    assert_eq!(contract.owners.len(), 0, "Owners map should be empty initially");
    assert_eq!(contract.cars.len(), 0, "Cars map should be empty initially");
    assert_eq!(contract.bookings.len(), 0, "Bookings map should be empty initially");
    assert_eq!(contract.users_accounts.len(), 0, "Users accounts should be empty initially");
    assert_eq!(contract.owners_accounts.len(), 0, "Owners accounts should be empty initially");
}

#[tokio::test]
async fn test_create_owner_account() {
    let mut contract = init_contract();
    let result = contract.create_owner_account("owner1".to_string(), "John Doe".to_string());
    assert!(result.is_ok(), "Creating owner account failed");
    // Assert that the owner exists
    assert!(contract.owners.contains_key("owner1"), "Owner account was not created");
    // Assert that the owner's name is correct
    assert_eq!(
        contract.owners.get("owner1").unwrap().name,
        "John Doe",
        "Owner name mismatch"
    );
    // Assert owners_accounts has been updated
    assert_eq!(contract.owners_accounts.len(), 1, "Owners accounts should contain one entry");
    assert_eq!(contract.owners_accounts[0], "owner1".parse::<AccountId>().unwrap(), "Owner account ID mismatch");
}

#[tokio::test]
async fn test_create_user_account() {
    let mut contract = init_contract();
    let result = contract.create_user_account("user1".to_string(), "Alice".to_string(), "DL-123456".to_string());
    assert!(result.is_ok(), "Creating user account failed");
    assert_eq!(
        contract.users.get("user1").unwrap().name,
        "Alice",
        "User name mismatch"
    );
}

#[tokio::test]
async fn test_add_car() {
    let mut contract = init_contract();
    // create the owner account
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    // simulate that owner is the caller
    testing_env!(get_context("owner1".to_string().try_into().unwrap()).build());
    // add a car as owner1
    let result = contract.add_car("car1".to_string(), "owner1".to_string(), 2000000000000000000000); // 2 NEAR per hour
    // verify the result
    assert!(result.is_ok(), "Adding car failed");
    assert_eq!(
        contract.cars.get("car1").unwrap().hourly_rate,
        2000000000000000000000,
        "Car rate mismatch"
    );
}

#[tokio::test]
async fn test_delete_car() {
    let mut contract = init_contract();
    // create owner account
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    // add the car by the owner
    contract.add_car("car1".to_string(), "owner1".to_string(), 2000000000000000000000).unwrap();
    // set the context to simulate the owner1 as the caller
    let context = get_context("owner1".to_string().try_into().unwrap()).build();
    testing_env!(context);
    // attempt to delete car as owner1
    let result = contract.delete_car("car1".to_string());
    // verify the result
    assert!(result.is_ok(), "Deleting car failed: {:?}", result.err());
    // verify that the car was actually deleted
    assert!(contract.cars.get("car1").is_none(), "Car was not deleted");
}

#[tokio::test]
async fn test_book_car() {
    let mut contract = init_contract();
    // create owner and user accounts
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    contract.create_user_account("user1".to_string(), "Alice".to_string(), "DL-123456".to_string()).unwrap();
    // add a car
    contract.add_car("car1".to_string(), "owner1".to_string(), 2000000000000000000000).unwrap();
    // Step 4: Set the context to simulate that 'user1' is calling the contract
    let user_account_id: AccountId = "user1".parse().unwrap();
    testing_env!(get_context(user_account_id).build()); 
    // get current block timestamp
    let now = near_sdk::env::block_timestamp();
    // try to book the car
    let result = contract.book_car(
        "car1".to_string(),
        "user1".to_string(),
        now, // Start time now
        now + 3600000000000, // One hour from now
        near_sdk::NearToken::from_yoctonear(100_000_000_000_000_000_000), // 0.1 NEAR deposit
    );
    // verify the result
    assert!(result.is_ok(), "Booking car failed: {:?}", result.err());
    assert_eq!(
        contract.bookings.len(),
        1,
        "Booking was not created"
    );
}

#[tokio::test]
async fn test_cancel_booking() {
    let mut contract = init_contract();
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    contract.create_user_account("user1".to_string(), "Alice".to_string(), "DL-123456".to_string()).unwrap();
    contract.add_car("car1".to_string(), "owner1".to_string(), 2000000000000000000000).unwrap();
    
    let now = near_sdk::env::block_timestamp();
    contract.book_car(
        "car1".to_string(),
        "user1".to_string(),
        now,
        now + 3600000000000, // One hour from now
        near_sdk::NearToken::from_yoctonear(100_000_000_000_000_000_000), // 0.1 NEAR deposit
    ).unwrap();
    
    let booking_id = contract.bookings.values().next().unwrap().booking_id.clone();
    let result = contract.cancel_booking(booking_id);
    assert!(result.is_ok(), "Canceling booking failed");
    assert_eq!(contract.bookings.len(), 0, "Booking was not canceled");
}

#[tokio::test]
async fn test_rent_car() {
    let mut contract = init_contract();
    // create owner and user accounts
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    contract.create_user_account("user1".to_string(), "Alice".to_string(), "DL-123456".to_string()).unwrap();
    // add a car associated with owner1
    contract.add_car("car1".to_string(), "owner1".to_string(), 2000000000000000000000).unwrap();
    // set the testing environment for user1
    testing_env!(get_context("user1".parse().unwrap())
        .prepaid_gas(Gas::from_gas(10u64.pow(12)))
        .attached_deposit(NearToken::from_yoctonear(1_000_000_000_000_000_000_000_000u128))
        .build());
    // attempt to rent the car for 1 hour
    let result = contract.rent_car("car1".to_string(), "user1".to_string(), 1); // Rent for 1 hour
    assert!(result.is_ok(), "Renting car failed");
    // verify that the car is no longer available
    let car = contract.cars.get("car1").unwrap();
    assert!(!car.available, "Car should not be available after renting");
}

#[tokio::test]
async fn test_return_car() {
    let mut contract = init_contract();
    let now = near_sdk::env::block_timestamp();
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    contract.create_user_account("user1".to_string(), "Alice".to_string(), "DL-123456".to_string()).unwrap();
    contract.add_car("car1".to_string(), "owner1".to_string(), 2000000000000000000000).unwrap();
    
    // Simulate renting
    testing_env!(get_context(accounts(1)).prepaid_gas(Gas::from_gas(10u64.pow(12))).attached_deposit(NearToken::from_yoctonear(1_000_000_000_000_000_000_000_000u128)).build());
    contract.rent_car("car1".to_string(), "user1".to_string(), 1).unwrap();
    
    // Return the car
    let result = contract.return_car("car1".to_string());
    assert!(result.is_ok(), "Returning car failed");
    assert!(contract.cars.get("car1").unwrap().available, "Car should be available after return");
}

// Test helper functions
#[tokio::test]
async fn test_is_owner() {
    let mut contract = init_contract();
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    assert!(contract.is_owner(&"owner1".parse().unwrap()), "Should be recognized as owner");
    assert!(!contract.is_owner(&"user1".parse().unwrap()), "Should not be recognized as owner");
}

#[tokio::test]
async fn test_is_user() {
    let mut contract = init_contract();
    contract.create_user_account("user1".to_string(), "Alice".to_string(), "DL-123456".to_string()).unwrap();
    assert!(contract.is_user(&"user1".parse().unwrap()), "Should be recognized as user");
    assert!(!contract.is_user(&"owner1".parse().unwrap()), "Should not be recognized as user");
}

// Test read-only functions
#[tokio::test]
async fn test_list_owner_cars() {
    let mut contract = init_contract();
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    for i in 1..=3 {
        contract.add_car(format!("car{}", i), "owner1".to_string(), 2000000000000000000000).unwrap();
    }
    let cars = contract.list_owner_cars("owner1".to_string());
    assert_eq!(cars.len(), 3, "Owner should have 3 cars");
}

#[tokio::test]
async fn test_list_available_cars() {
    let mut contract = init_contract();
    let now = near_sdk::env::block_timestamp();
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    contract.create_user_account("user1".to_string(), "Alice".to_string(), "DL-123456".to_string()).unwrap();
    
    contract.add_car("car1".to_string(), "owner1".to_string(), 2000000000000000000000).unwrap();
    contract.add_car("car2".to_string(), "owner1".to_string(), 2000000000000000000000).unwrap();
    
    // Book car2
    contract.book_car(
        "car2".to_string(),
        "user1".to_string(),
        now,
        now + 3600000000000, // Book for 1 hour from now
        near_sdk::NearToken::from_yoctonear(100_000_000_000_000_000_000), // 0.1 NEAR deposit
    ).unwrap();
    
    let available_cars = contract.list_available_cars();
    assert_eq!(available_cars.len(), 1, "Only one car should be available");
    assert_eq!(available_cars[0].car_id, "car1", "The available car should be car1");
}

#[tokio::test]
async fn test_list_user_bookings() {
    let mut contract = init_contract();
    contract.create_owner_account("owner1".to_string(), "John Doe".to_string()).unwrap();
    contract.create_user_account("user1".to_string(), "Alice".to_string(), "DL-123456".to_string()).unwrap();
    contract.create_user_account("user2".to_string(), "Bob".to_string(), "DL-789012".to_string()).unwrap();
    contract.add_car("car1".to_string(), "owner1".to_string(), 2000000000000000000000).unwrap();
    
    let now = near_sdk::env::block_timestamp();
    contract.book_car(
        "car1".to_string(),
        "user1".to_string(),
        now,
        now + 3600000000000, // Book for 1 hour from now
        near_sdk::NearToken::from_yoctonear(100_000_000_000_000_000_000), // 0.1 NEAR deposit
    ).unwrap();
    
    contract.book_car(
        "car1".to_string(),
        "user2".to_string(),
        now + 3600000000000, // Book after user1's booking
        now + 7200000000000, // Book for 1 hour from that time
        near_sdk::NearToken::from_yoctonear(100_000_000_000_000_000_000), // 0.1 NEAR deposit
    ).unwrap();
    
    let user1_bookings = contract.list_user_bookings("user1".to_string());
    assert_eq!(user1_bookings.len(), 1, "User1 should have 1 booking");
    
    let user2_bookings = contract.list_user_bookings("user2".to_string());
    assert_eq!(user2_bookings.len(), 1, "User2 should have 1 booking");
}