import React, { useEffect, useState } from 'react';
import { SetupModal } from '@calimero-is-near/calimero-p2p-sdk';
import { useNavigate } from 'react-router-dom';
import { getNodeUrl, getStorageApplicationId } from '../../utils/node';
import { setStorageAppEndpointKey, setStorageApplicationId } from '../../utils/storage';
import * as nearAPI from 'near-api-js';

const NEAR_CONFIG = {
  networkId: 'testnet',
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
  explorerUrl: 'https://explorer.testnet.near.org',
  contractName: 'your-car-sharing-contract.testnet', // Replace with actual contract name
};

function SetupPage() {
  const navigate = useNavigate();
  const [near, setNear] = useState<nearAPI.Near | null>(null);
  const [account, setAccount] = useState<nearAPI.Account | null>(null);
  const [setupCompleted, setSetupCompleted] = useState(false);

  useEffect(() => {
    const initContract = async () => {
      const near = await nearAPI.connect(Object.assign({
        deps: { keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore() }
      }, NEAR_CONFIG));
      setNear(near);

      // Check if there's an already signed-in account
      if (near && near.connection.signer) {
        const signer = near.connection.signer as unknown as (nearAPI.Signer & { getAccountId: () => Promise<string> });
        const accountId = await signer.getAccountId();
        if (accountId) {
          const account = new nearAPI.Account(near.connection, accountId);
          setAccount(account);
          navigate('/auth');
        }
      }
    };

    initContract();
  }, [navigate]);

  const setupSuccess = () => {
    setSetupCompleted(true);
    setTimeout(() => navigate('/auth'), 1000);
  };

  return (
    <div>
      <h2>Setup Your Car-Sharing Application</h2>
      <p>Please complete the setup to continue with NEAR integration.</p>
      
      {setupCompleted ? (
        <p>Setup completed. Redirecting to login...</p>
      ) : (
        <SetupModal
          successRoute={setupSuccess}
          getNodeUrl={getNodeUrl}
          setNodeUrl={setStorageAppEndpointKey}
          setApplicationId={setStorageApplicationId}
          getApplicationId={getStorageApplicationId}
        />
      )}
    </div>
  );
}

export default SetupPage;