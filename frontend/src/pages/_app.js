import { useEffect, useState, useMemo } from 'react';
import '@/styles/globals.css';
import { Navigation } from '@/components/navigation';
import { Wallet, NearContext } from '@/utils/near';
import { NetworkId } from '@/config';
import AccountCreation from '@/components/AccountCreation';
import { useRouter } from 'next/router';

const wallet = new Wallet({ networkId: NetworkId });

export default function MyApp({ Component, pageProps }) {
  const [signedAccountId, setSignedAccountId] = useState('');
  const [user, setUser] = useState(null);
  const [isLoading, setIsLoading] = useState(true);
  const [accountCreated, setAccountCreated] = useState(null);
  const router = useRouter();

  useEffect(() => {
    wallet.startUp((accountId) => {
      setSignedAccountId(accountId);
      if (accountId) {
        Promise.all([
          wallet.viewMethod({
            contractId: 'partage.testnet',
            method: 'is_owner',
            args: { account_id: accountId }
          }),
          wallet.viewMethod({
            contractId: 'partage.testnet',
            method: 'is_user',
            args: { account_id: accountId }
          })
        ]).then(([isOwner, isUser]) => {
          setAccountCreated(isOwner || isUser);
          if (isOwner) {
            fetchOwnerCars(accountId);
          } else if (isUser) {
            fetchUserBookings(accountId);
          }
        }).catch(() => {
          setAccountCreated(false);
          console.error('Failed to check account status');
        });
      } else {
        setAccountCreated(false);
      }
    });
  }, []);

  const fetchOwnerCars = async (accountId) => {
    try {
      const cars = await wallet.viewMethod({
        contractId: 'partage.testnet',
        method: 'list_owner_cars',
        args: { owner_id: accountId }
      });
      setUser({ role: 'owner', cars });
    } catch (error) {
      console.error('Error fetching owner cars:', error);
      setUser(null);
    } finally {
      setIsLoading(false);
    }
  };

  const fetchUserBookings = async (accountId) => {
    try {
      const bookings = await wallet.viewMethod({
        contractId: 'partage.testnet',
        method: 'list_user_bookings',
        args: { user_id: accountId }
      });
      setUser({ role: 'user', bookings });
    } catch (error) {
      console.error('Error fetching user bookings:', error);
      setUser(null);
    } finally {
      setIsLoading(false);
    }
  };

  const handleAccountCreated = () => {
    setAccountCreated(true);
    // Navigate to User Profile after account creation
    router.push('/userprofile');
  };

  const nearContext = useMemo(() => ({
    wallet,
    signedAccountId,
    accountCreated
  }), [signedAccountId, accountCreated]);


  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <NearContext.Provider value={{ wallet, signedAccountId, nearContext, user }}>
      <Navigation />
      { isLoading ? (
        <div>Loading...</div>
      ) : !signedAccountId ? (
        <div>Please sign in to continue</div>
      ) : accountCreated === null ? (
        <div>Loading...</div>
      ) : accountCreated === false ? (
        <AccountCreation 
          setAccountCreated={setAccountCreated} 
          onAccountCreated={handleAccountCreated}
        />
      ) : (
        // return null for the user to be redirected to the user profile
        null
      )}
    </NearContext.Provider>
  );
}