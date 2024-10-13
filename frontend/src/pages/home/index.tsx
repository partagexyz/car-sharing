import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import * as nearAPI from 'near-api-js';
import { getAvailableCars, bookCar } from '../../utils/node';
import onLogOut from '../login/Authenticate';
import { Car } from '../../utils/storage';

function HomePage({ near, account }: { near: nearAPI.Near | null, account: nearAPI.Account | null }) {
  const navigate = useNavigate();
  const [availableCars, setAvailableCars] = useState<Car[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!near || !account) {
      navigate('/auth');
      return;
    }
    fetchAvailableCars();
  }, [near, account, navigate]);

  const fetchAvailableCars = async () => {
    setLoading(true);
    setError(null);
    try {
      const cars = await getAvailableCars();
      setAvailableCars(cars);
    } catch (e) {
      console.error('Error fetching available cars:', e);
      setError('Failed to fetch available cars. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const bookCarHandler = async (carId: string) => {
    try {
     // adjust with startDate and endDate from smart contract
      const now = new Date();
      const to = new Date(now.getTime() + 24 * 60 * 60 * 1000); // 24 hours from now
      
      await bookCar(carId, now, to);
      
      fetchAvailableCars();
    } catch (e) {
      console.error('Error booking car:', e);
      setError('Failed to book car. Please try again.');
    }
  };

  const logout = () => {
      onLogOut();
    }

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error}</p>;

  return (
    <div>
      <h1>Welcome to Your Car Sharing App</h1>
      
      <h2>Available Cars</h2>
      <h2>Available Cars</h2>
    {availableCars.length > 0 ? (
      <ul>
        {availableCars.map(car => (
          <li key={car.id}>
            Car ID: {car.id} - 
            Available: {car.available ? 'Yes' : 'No'} - 
            Rate: {car.hourlyRate} per hour
            <button onClick={() => bookCarHandler(car.id)}>Book Now</button>
          </li>
        ))}
            </ul>
        ) : (
            <p>No cars available at the moment.</p>
        )}

      <button onClick={logout}>Logout</button>
    </div>
  );
}

export default HomePage;