// this component will display options for creating a user or owner account
import React, { useState, useContext } from 'react';
import { useRouter } from 'next/router';
import { NearContext } from '@/wallets/near';

const AccountCreation = ({ setAccountCreated }) => {
    const { wallet } = useContext(NearContext);
    const router = useRouter();
    const [error, setError] = useState('');
    const [accountType, setAccountType] = useState('user');
    const [name, setName] = useState('');
    const [license, setLicense] = useState('');
    const [isOwner, setIsOwner] = useState(false);

    const handleSubmit = async (event) => {
        event.preventDefault();
        setError('');
        try {
            let result;
            const accountId = wallet.getAccountId();
            
            if (accountType === 'user') {
                // Call the create_user_account method in smart contract
                result = await wallet.callMethod('create_user_account', {
                    user_id: accountId,
                    name,
                    driving_license: license,
                });
            } else {
                // Call the create_owner_account method in smart contract
                result = await wallet.callMethod('create_owner_account', {
                    owner_id: accountId,
                    name,
                });
            }

            if (result.success) {
                setAccountCreated(true);
                // Navigate to UserProfile after account creation
                router.push('/userprofile');
            } else {
                throw new Error(result.error);
            }
        } catch (error) {
            setError(error.message);
        }
    };

    return (
        <div>
          <h2>Create Account</h2>
          <form onSubmit={handleSubmit}>
            <label>
              Account Name: {wallet.getAccountId}
            </label>
            <br />
            <label>
              Name:
              <input 
                type="text" 
                value={name} 
                onChange={(e) => setName(e.target.value)} 
                required 
              />
            </label>
            <br />
            <p>Account Type:</p>
            <label>
              <input 
                type="radio" 
                value="user" 
                checked={accountType === 'user'} 
                onChange={() => setAccountType('user')}
              /> 
              User
            </label>
            <label>
              <input 
                type="radio" 
                value="owner" 
                checked={accountType === 'owner'} 
                onChange={() => setAccountType('owner')}
              /> 
              Owner
            </label>
            <br />
            {accountType === 'user' && (
              <label>
                Driving License:
                <input 
                  type="text" 
                  value={license} 
                  onChange={(e) => setLicense(e.target.value)} 
                  required 
                />
              </label>
            )}
            <br />
            <button type="submit">Create Account</button>
          </form>
          {error && <p style={{ color: 'red' }}>{error}</p>}
        </div>
    );
};

export default AccountCreation;