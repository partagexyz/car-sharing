import { useEffect, useState, useMemo, useCallback } from 'react';
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
  const [redirectToProfile, setRedirectToProfile] = useState(false);
  const router = useRouter();

  const fetchOwnerCars = useCallback(async (accountId) => {
      const cars = await wallet.viewMethod({
        contractId: 'partage.testnet',
        method: 'list_owner_cars',
        args: { owner_id: accountId }
      });
      return { cars };
  }, []);

  const fetchUserBookings = useCallback(async (accountId) => {
      const bookings = await wallet.viewMethod({
        contractId: 'partage.testnet',
        method: 'list_user_bookings',
        args: { user_id: accountId }
      });
      return { bookings };
  }, []);

  useEffect(() => {
    console.log("Starting wallet startup...");
    wallet.startUp(async (accountId) => {
      console.log("Account ID received:", accountId);
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
                const userData = await (isOwner ? fetchOwnerCars(accountId) : fetchUserBookings(accountId));
                console.log("Fetched user data:", userData);
                setUser({ role: userRole, ...userData });
                setRedirectToProfile(true);
            } else {
              setUser(null);
            }
          } catch (error) {
            console.error('Error fetching user data:', error);
            setUser(null);
          } finally {
            console.log("Finished loading user data");
            setIsLoading(false);
            console.log("isLoading set to false");
          }
      } else {
        setUser(null);
        setIsLoading(false);
      }
    });
  }, [fetchOwnerCars, fetchUserBookings]);

  useEffect(() => {
    if (!isLoading && redirectToProfile) {
      console.log("Redirecting to /profile");
      router.push('/profile');
      setRedirectToProfile(false);
    }
  }, [isLoading, redirectToProfile, router]);

  const handleAccountCreated = () => {
    console.log("Account created!");
    setAccountCreated(true);
    console.log("Redirecting to /profile");
    router.push('/profile');
  };

  const nearContext = useMemo(() => ({
    wallet,
    signedAccountId,
    accountCreated,
    user
  }), [signedAccountId, accountCreated, user]);

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
        <Component {...pageProps} />
      )}
    </NearContext.Provider>
  );
}