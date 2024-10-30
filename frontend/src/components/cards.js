import React from 'react';
import styles from '@/styles/app.module.css';

export const Cards = ({ cars, type, onDelete, onCancel }) => {
  return (
    <div className={styles.grid}>
      {cars.map((car, index) => (
        <div key={index} className={styles.card}>
          {type === 'car' ? (
            <>
              <strong>Car ID:</strong> {car.car_id}<br />
              <strong>Hourly Rate:</strong> {car.hourly_rate} yoctoNEAR<br />
              <strong>Available:</strong> {car.available ? 'Yes' : 'No'}
              {onDelete && <button onClick={() => onDelete(car.car_id)}>Delete Car</button>}
            </>
          ) : type === 'booking' ? (
            <>
              <strong>Booking ID:</strong> {car.id}<br />
              <strong>Car ID:</strong> {car.car_id}<br />
              <strong>Start Time:</strong> {new Date(booking.start_time / 1000000)}<br />
              <strong>End Time:</strong> {new Date(booking.end_time / 1000000)}
              {onCancel && <button onClick={() => onCancel(car.booking_id)}>Cancel Booking</button>}
            </>
          ) : null}
        </div>
      ))}
    </div>
  );
};