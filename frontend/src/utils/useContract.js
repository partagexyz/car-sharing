import { useState, useEffect, useContext } from 'react';
import { NearContext } from '@/utils/near';
import { CarSharingContract } from '../config';

const CONTRACT = CarSharingContract;

export const useContract = () => {
  const { signedAccountId, wallet } = useContext(NearContext);
  const [greeting, setGreeting] = useState('loading...');
  const [loggedIn, setLoggedIn] = useState(false);
  const [showSpinner, setShowSpinner] = useState(false);

  useEffect(() => {
    if (!wallet) return;

    wallet.viewMethod({ contractId: CONTRACT, method: 'get_greeting' }).then(
      greeting => setGreeting(greeting)
    );
  }, [wallet]);

  useEffect(() => {
    setLoggedIn(!!signedAccountId);
  }, [signedAccountId]);

  const updateGreeting = async (newGreeting) => {
    setShowSpinner(true);
    await wallet.callMethod({ contractId: CONTRACT, method: 'set_greeting', args: { greeting: newGreeting } });
    const greeting = await wallet.viewMethod({ contractId: CONTRACT, method: 'get_greeting' });
    setGreeting(greeting);
    setShowSpinner(false);
  };

  return {
    greeting,
    loggedIn,
    showSpinner,
    updateGreeting,
  };
};