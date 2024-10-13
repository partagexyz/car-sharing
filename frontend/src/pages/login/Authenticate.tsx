import React from 'react';
import { ClientLogin } from '@calimero-is-near/calimero-p2p-sdk';
import { useNavigate } from 'react-router-dom';
import { setStorageAppEndpointKey, setStorageApplicationId } from '../../utils/storage';
import { getNodeUrl, getStorageApplicationId } from '../../utils/node';
import * as nearAPI from 'near-api-js';

const NEAR_CONFIG = {
  networkId: 'testnet',
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  contractName: 'your-contract-name.testnet', // Replace with your actual contract name
};

function Authenticate() {
  const navigate = useNavigate();
  const [near, setNear] = React.useState<nearAPI.Near | null>(null);
  const [account, setAccount] = React.useState<nearAPI.Account | null>(null);

  React.useEffect(() => {
    const initContract = async () => {
      const near = await nearAPI.connect(Object.assign({
        deps: { keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore() }
      }, NEAR_CONFIG));
      setNear(near);
      // If there's already a signed-in account, redirect to home
      if (near && near.connection.signer) {
        const signer = near.connection.signer as unknown as (nearAPI.Signer & { getAccountId: () => Promise<string> });
        const accountId = await signer.getAccountId();
        if (accountId) {
          navigate('/home');
        }
      }
    };
    initContract();
  }, [navigate]);

  const onSetupClick = () => {
    setStorageAppEndpointKey('');
    setStorageApplicationId('');
    navigate('/');
  };

  const onLogout = () => {
    if (near) {
      (near as any).signOut();
      // clear local state or storage if needed
      setAccount(null);
      navigate('/'); // Redirect to a default page
    }
  };

  const onLogin = () => {
    if (near) {
        (near as any).requestSignIn(NEAR_CONFIG.contractName);
    }
  };

  return (
    <div>
      <h2>Login with NEAR</h2>
      {account ? (
        <div>
          <p>Welcome, {account.accountId}</p>
          <button onClick={onSetupClick}>Return to Setup</button>
          <button onClick={onLogout}>Logout</button>
        </div>
      ) : (
        <button onClick={onLogin}>Sign In</button>
      )}
      {/* You can keep the ClientLogin component if you still need other functionalities */}
      <ClientLogin
        getNodeUrl={getNodeUrl}
        getApplicationId={getStorageApplicationId}
        sucessRedirect={() => navigate('/home')}
      />
    </div>
  );
}

export default Authenticate;