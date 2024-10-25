import { useEffect, useState, useMemo } from 'react';
import '@/styles/globals.css';
import { Navigation } from '@/components/navigation';
import { Wallet, NearContext } from '@/wallets/near';
import { NetworkId } from '@/config';
import AccountCreation from '@/components/AccountCreation';
import UserProfile from '@/components/UserProfile';
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
        fetchUserData(accountId);
      wallet.viewMethod({
        contractId: 'your-contract-id',
        method: 'is_account_created',
        args: { account_id: accountId }
      }).then((exists) => setAccountCreated(exists))
        .catch(() => setAccountCreated(false));
    } else {
      setAccountCreated(false);
    }
    });
  }, []);

  const fetchUserData = async (accountId) => {
    try {
      const contract = await wallet.getContract();
      const userData = await contract.getUser(accountId);
      setUser(userData);
    } catch (error) {
      console.error('Error fetching user data:', error);
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
    <NearContext.Provider value={{ wallet, signedAccountId, nearContext }}>
      <Navigation />
      {!signedAccountId ? (
        <div>Please sign in to continue</div>
      ) : accountCreated === null ? (
        <div>Loading...</div>
      ) : accountCreated === false ? (
        <AccountCreation 
          setAccountCreated={setAccountCreated} 
          onAccountCreated={handleAccountCreated}
        />
      ) : (
        <UserProfile user={{ id: signedAccountId, role: 'user' }} />
      )}
    </NearContext.Provider>
  );
}