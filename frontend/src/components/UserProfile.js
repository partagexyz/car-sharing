import React, { useState, useContext, useEffect } from 'react';
import { NearContext } from '@/wallets/near';

const UserProfile = ({ user }) => {
    const { wallet } = useContext(NearContext);
    const [cars, setCars] = useState([]);
    const [bookings, setBookings] = useState([]);
    const [userData, setUserData] = useState(null);
    const isOwner = user.role === 'owner';

    // Function to fetch user's profile data
    const fetchUserData = async () => {
        try {
            const contract = await wallet.getContract();
            const userData = await contract.getUser(user.id);
            setUserData(userData);
            // If the user has cars or bookings, fetch them after getting userData
            if(userData && userData.role === 'owner') {
                fetchCars();
            } else {
                fetchBookings();
            }
        } catch (error) {
            console.error('Error fetching user data:', error);
        }
    };

    // function to fetch cars for owners
    const fetchCars = async () => {
        try {
            const carsList = await wallet.callMethod('list_owner_cars', { owner_id: user.id });
            setCars(carsList);
        } catch (error) {
            console.error('Error fetching cars:', error);
        }
    };

    // function to add a car for owners
    const addCar = async (carData) => {
        try {
            const result = await wallet.callMethod('add_car', { owner_id: user.id, car: carData });
            if (result.success) {
                fetchCars();
            } else {
                console.error('Error adding car:', result.error);
            }
        } catch (error) {
            console.error('Error adding car:', error);
        }
    };

    // function to fetch bookings for users
    const fetchBookings = async () => {
        try {
            const bookingsList = await wallet.callMethod('list_user_bookings', { user_id: user.id });
            setBookings(bookingsList);
        } catch (error) {
            console.error('Error fetching bookings:', error);
        }
    };

    // function to book a car for users
    const bookCar = async (carId, startDate, endDate) => {
        try {
            // convert dates to nanoseconds to match smart contract
            const startTime = new Date(startDate).getTime() * 1000000;
            const endTime = new Date(endDate).getTime() * 1000000;

            const result = await wallet.callMethod('book_car', {
                car_id: carId,
                user_id: user.id, 
                start_time: startTime, 
                end_time: endTime 
            });

            if (result.success) {
                // fetch bookings to update the list
                fetchBookings();
            } else {
                console.error('Error booking car:', result.error);
            }
        } catch (error) {
            console.error('Error booking car:', error);
        }
    };

    // Fetch data when component mounts
    React.useEffect(() => {
        fetchUserData();
    }, []);

    // If user data hasn't been fetched yet, show a loading state
    if (!userData) {
        return <div>Loading user profile...</div>;
    }

    return (
        <div>
            <h1>Welcome, {userData.name}</h1>
            {isOwner ? (
                <div>
                    <h2>Your Cars</h2>
                    <ul>
                        {cars.map(car => (
                            <li key={car.id}>{car.model}</li>
                        ))}
                    </ul>
                    <button onClick={() => addCar({ car_id: "new-car-1", hourly_rate: "1000000000000000000000000" })}>Add Car</button>
                </div>
            ) : (
                <div>
                    <h2>Your Bookings</h2>
                    <ul>
                        {bookings.map(booking => (
                            <li key={booking.id}>{booking.carModel}</li>
                        ))}
                    </ul>
                    <button onClick={() => bookCar("some-car-id", "2023-10-23", "2023-10-24")}>Book Car</button>
                </div>
            )}
        </div>
    );
};

export default UserProfile;