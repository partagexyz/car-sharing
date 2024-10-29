// this component is responsible for displaying user specific functionality
import React, { useState, useContext, useEffect, useCallback } from 'react';
import { NearContext } from '@/utils/near';
import { Cards } from './Cards';

const UserProfile = ({ user }) => {
    const { wallet, signedAccountId } = useContext(NearContext);
    const [userData, setUserData] = useState(user);

    // Function to fetch user's profile data based on role
    useEffect(() => {
        if (userData && (userData.cars === undefined && userData.bookings === undefined)) {
            if (userData.role === 'owner') {
                fetchOwnerCars();
            } else if (userData.role === 'user') {
                fetchUserBookings();
            }
        }
    }, [userData, fetchOwnerCars, fetchUserBookings]);

    // function to fetch cars for owners
    const fetchOwnerCars = useCallback(async () => {
        try {
            const cars = await wallet.viewMethod({
                contractId: 'partage.testnet',
                method: 'list_owner_cars', 
                args: { owner_id: signedAccountId }
            });
            setUserData(prev => ({ ...prev, cars }));
        } catch (error) {
            console.error('Error fetching owners cars:', error);
        }
    }, [wallet, signedAccountId]);

    // function to fetch bookings for users
    const fetchUserBookings = useCallback(async () => {
        try {
            const bookings = await wallet.viewMethod({
                contractId: 'partage.testnet',
                method: 'list_user_bookings', 
                args: { user_id: signedAccountId }
            });
            setUserData(prev => ({ ...prev, bookings }));
        } catch (error) {
            console.error('Error fetching user bookings:', error);
        }
    }, [wallet, signedAccountId]);

    // function to add a car for owners
    const addCar = async (carId, hourlyRate) => {
        try {
            const result = await wallet.callMethod('add_car', { 
                car_id: carId, 
                owner_id: signedAccountId, 
                hourly_rate: hourlyRate.toString() 
            });
            if (result.success) {
                fetchOwnerCars();
            } else {
                console.error('Error adding car:', result.error);
            }
        } catch (error) {
            console.error('Error adding car:', error);
        }
    };

    // function to delete a car for owners
    const deleteCar = async (carId) => {
        try {
            const result = await wallet.callMethod('delete_car', { car_id: carId });
            if (result.success) {
                fetchOwnerCars();
            } else {
                console.error('Error deleting car:', result.error);
            }
        } catch (error) {
            console.error('Error deleting car:', error);
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
                user_id: signedAccountId, 
                start_time: startTime, 
                end_time: endTime 
            });

            if (result.success) {
                // fetch bookings to update the list
                fetchUserBookings();
            } else {
                console.error('Error booking car:', result.error);
            }
        } catch (error) {
            console.error('Error booking car:', error);
        }
    };

    // function to cancel a booking for users
    const cancelBooking = async (bookingId) => {
        try {
            const result = await wallet.callMethod('cancel_booking', { booking_id: bookingId });
            if (result.success) {
                fetchUserBookings();
            } else {
                console.error('Error canceling booking:', result.error);
            }
        } catch (error) {
            console.error('Error canceling booking:', error);
        }
    };

    if (!userData) return <div>Loading user profile...</div>;

    return (
        <div>
            <h1>Welcome, {signedAccountId}</h1>
            {userData.role === 'owner' ? (
                <div>
                    <h2>Your Cars</h2>
                    {userData.cars && <Cards cars={userData.cars} type="car" onDelete={deleteCar} />}
                    <button onClick={() => addCar("new-car-1", "1000000000000000000000000")}>Add Car</button>
                </div>
            ) : (
                <div>
                    <h2>Your Bookings</h2>
                    {userData.bookings && <Cards cars={userData.bookings} type="booking" onCancel={cancelBooking} />}
                    <button onClick={() => bookCar("some-car-id", "2023-10-23", "2023-10-24")}>Book Car</button>
                </div>
            )}
        </div>
    );
};

export default UserProfile;