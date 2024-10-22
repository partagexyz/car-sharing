import React from 'react';

const UserProfile = ({ user }) => {
    const isOwner = user.role === 'owner';

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