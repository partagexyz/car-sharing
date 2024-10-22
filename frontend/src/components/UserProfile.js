import React, { useState, useContext } from 'react';
import { NearContext } from '@/wallets/near';

const UserProfile = ({ user }) => {
    const { wallet } = useContext(NearContext);
    const [cars, setCars] = useState([]);
    const [bookings, setBookings] = useState([]);
    const isOwner = user.role === 'owner';

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

    return (
        <div>
            <h1>Welcome, {user.name}</h1>
            {isOwner ? (
                <div>
                    <h2>Your Cars</h2>
                    <ul>
                        {user.cars.map(car => (
                            <li key={car.id}>{car.model}</li>
                        ))}
                    </ul>
                </div>
            ) : (
                <div>
                    <h2>Your Bookings</h2>
                    <ul>
                        {user.bookings.map(booking => (
                            <li key={booking.id}>{booking.carModel}</li>
                        ))}
                    </ul>
                </div>
            )}
        </div>
    );
};

export default UserProfile;