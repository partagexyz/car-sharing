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
    wallet.startUp(async (accountId) => {
        setSignedAccountId(accountId);
        if (accountId) {
            try {
                const isOwner = await wallet.viewMethod({
                    contractId: 'partage.testnet',
                    method: 'is_owner',
                    args: { account_id: accountId }
                });
                
                const isUser = await wallet.viewMethod({
                    contractId: 'partage.testnet',
                    method: 'is_user',
                    args: { account_id: accountId }
                });

                if (isOwner || isUser) {
                    const userRole = isOwner ? 'owner' : 'user';
                    const userData = await (isOwner ? 
                        fetchOwnerCars(accountId) : 
                        fetchUserBookings(accountId)
                    );

                    setUser({ role: userRole, ...userData });
                } else {
                    setUser(null);
                }
            } catch (error) {
                console.error('Error fetching user data:', error);
                setUser(null);
            } finally {
                setIsLoading(false);
            }
        } else {
            setUser(null);
            setIsLoading(false);
        }
    });
}, []);

  const fetchOwnerCars = async (accountId) => {
      const cars = await wallet.viewMethod({
        contractId: 'partage.testnet',
        method: 'list_owner_cars',
        args: { owner_id: accountId }
      });
      return { cars };
  };

  const fetchUserBookings = async (accountId) => {
      const bookings = await wallet.viewMethod({
        contractId: 'partage.testnet',
        method: 'list_user_bookings',
        args: { user_id: accountId }
      });
      return { bookings };
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