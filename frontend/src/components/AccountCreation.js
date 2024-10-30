// this component will display options for creating a user or owner account
import React, { useState, useContext } from 'react';
import { useRouter } from 'next/router';
import { NearContext } from '@/utils/near';

const AccountCreation = ({ setAccountCreated }) => {
    const { wallet, signedAccountId } = useContext(NearContext);
    const router = useRouter();
    const [error, setError] = useState('');
    const [message, setMessage] = useState('');
    const [accountType, setAccountType] = useState('user');
    const [name, setName] = useState('');
    const [license, setLicense] = useState('');

    // function to handle form submission
    const handleSubmit = async (event) => {
      event.preventDefault();
      setError('');
      try {
        // Check that signedAccountId is defined
        if (!signedAccountId) {
          throw new Error("Account ID not defined. Please sign in.");
        }

        // check if the account is already an owner or a user
        const is_owner = await wallet.viewMethod({
          contractId: 'partage.testnet',
          method: 'is_owner',
          args: { account_id: signedAccountId }
        });

        const is_user = await wallet.viewMethod({
          contractId: 'partage.testnet',
          method: 'is_user',
          args: { account_id: signedAccountId }
        });

        // if account exists, redirect to user profile
        if (is_owner || is_user) {
          setMessage("Account already exists.");
          setTimeout(() => router.push('/profile'), 1000);
          return;
        }

        // if account doesnt exist, procees with account creation
        const contractId = 'partage.testnet';
        // map account type to the correct method and parameter
        const method = accountType === 'user' ? 'create_user_account' : 'create_owner_account';
        const args = accountType === 'user' 
          ? { user_id: signedAccountId, name, driving_license: license } 
          : { owner_id: signedAccountId, name };

        // call the smart contract method
        const result = await wallet.callMethod(contractId, method, args);

        if (result.success) {
          // update accountCreated state in parent component
          setAccountCreated(true);
          setMessage("Account created!");
          // Navigate to UserProfile after account creation
          setTimeout(() => router.push('/profile'), 1000);
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
              Account Name: {signedAccountId}
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