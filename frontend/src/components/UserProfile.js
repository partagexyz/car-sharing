// this component is responsible for displaying user specific functionality
import React, { useContext, useEffect } from 'react';
import { NearContext } from '@/utils/near';
import { Cards } from './Cards';

const UserProfile = () => {
    const { user, wallet, signedAccountId } = useContext(NearContext);
    
    useEffect(() => {
        console.log("User in context:", user);
        console.log("Wallet in context:", wallet);
        console.log("Signed Account ID:", signedAccountId);
    }, [user, wallet, signedAccountId]);

    // if user data hasn't been fetched yet, or there's no user
    if (!user || Object.keys(user).length === 0) {
        console.log("No user data available.");
        return <div>No user data available.</div>;
    }

    // function to add a car for owners
    const addCar = async (carId, hourlyRate) => {
        try {
            const result = await wallet.callMethod('add_car', { 
                car_id: carId, 
                owner_id: signedAccountId, 
                hourly_rate: hourlyRate.toString() 
            });
            if (result.success) {
                console.log('Car added successfully');
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
                console.log('Car deleted successfully');
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
                console.log('Car booked successfully');
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
                console.log('Booking canceled successfully');
            } else {
                console.error('Error canceling booking:', result.error);
            }
        } catch (error) {
            console.error('Error canceling booking:', error);
        }
    };

    return (
        <div>
            <h1>Welcome, {signedAccountId}</h1>
            <p>Role: {user.role}</p>
            {user.role === 'owner' ? (
                <div>
                    <h2>Your Cars</h2>
                    {user.cars && user.cars.length > 0 ? (
                        <Cards cars={user.cars} type="car" onDelete={deleteCar} />
                    ) : (
                        <p>You do not have any cars listed yet.</p>
                    )}
                    <button onClick={() => addCar("new-car-1", "1000000000000000000000000")}>Add Car</button>
                </div>
            ) : (
                <div>
                    <h2>Your Bookings</h2>
                    {user.bookings && user.bookings.length > 0 ? (
                        <Cards cars={user.bookings} type="booking" onCancel={cancelBooking} />
                    ) : (
                        <p>You do not have any bookings yet.</p>
                    )}
                    <button onClick={() => bookCar("some-car-id", "2023-10-23", "2023-10-24")}>Book Car</button>
                </div>
            )}
        </div>
    );
};

export default UserProfile;