import {
    getApplicationId,
    getJWTObject,
    getStorageAppEndpointKey,
    getStorageContextId,
    setStorageAppEndpointKey,
    setStorageApplicationId,
    setStorageContextId,
  } from './storage';
  
  // This might need import from 'near-api-js' if you're using NEAR specific functions
  import * as nearAPI from 'near-api-js';
  
  export function getNodeUrl(): string {
    let storageKey = getStorageAppEndpointKey();
    if (!storageKey) {
      // Assuming you're using environment variables for different environments
      let envKey: string = process.env.REACT_APP_API_URL ?? 'default-near-node-url';
      setStorageAppEndpointKey(envKey);
      return envKey;
    }
    return storageKey;
  }
  
  export function getContextId(): string {
    let storageContextId = getStorageContextId();
  
    if (!storageContextId) {
      // Here you might want to fetch or generate context based on user session or similar
      let jwtToken = getJWTObject();
      let envKey: string = jwtToken?.context_id ?? '';
      setStorageContextId(envKey);
      return envKey;
    }
  
    return storageContextId;
  }
  
  export function getNearEnvironment(): string {
    return process.env.REACT_APP_NEAR_ENV ?? 'testnet';
  }
  
  export function getStorageApplicationId(): string {
    let storageApplicationId = getApplicationId();
  
    if (!storageApplicationId) {
      let envKey: string = process.env.REACT_APP_APPLICATION_ID ?? '';
      setStorageApplicationId(envKey);
      return envKey;
    }
  
    return storageApplicationId;
  }
  
  // Additional functions specific to your car-sharing app
  export async function getCurrentUser(): Promise<nearAPI.Account | null> {
    const near = await nearAPI.connect({
      networkId: getNearEnvironment(),
      nodeUrl: getNodeUrl(),
      walletUrl: 'your-wallet-url', // adjust as needed
      //explorerUrl: 'your-explorer-url', // Example, adjust as needed
      deps: { keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore() }
    } as nearAPI.ConnectConfig);

    const accountId = await (near.connection.signer as nearAPI.Signer & { getAccountId: () => Promise<string> })?.getAccountId?.();
    if (accountId) {
      return new nearAPI.Account(near.connection, accountId);
    }
    return null;
  }
  
  // Example of a function for fetching car data from a smart contract
  export async function getAvailableCars(): Promise<any> {
    const account = await getCurrentUser();
    if (!account) {
      throw new Error('User not signed in or no account found');
    }
    try {
      // Assuming your contract has a method called 'get_available_cars'
      const cars = await account.viewFunction({
        contractId: 'your-car-sharing-contract.testnet', // Replace with your actual contract name
        methodName: 'get_available_cars',
        args: {}
    });
      return cars;
    } catch (e) {
      console.error('Failed to fetch available cars:', e);
      return [];
    }
  }
  
  export async function bookCar(carId: string, from: Date, to: Date): Promise<boolean> {
    const account = await getCurrentUser();
    if (!account) {
      throw new Error('User not signed in or no account found');
    }
  
    try {
      // link to method 'book_car' in contract
      const result = await account.functionCall({
        contractId: 'car_sharing.testnet', // Replace with contract Id
        methodName: 'book_car',
        args: { 
            car_id: carId,
            user_id: account.accountId,
            start_time: from.getTime() / 1000, // verify if conversion to seconds is correct
            end_time: to.getTime() / 1000, // same
            deposit: '1' // adjust based on contract's expectations
        },
        gas: BigInt('300000000000000'), // Adjust as needed
        attachedDeposit: BigInt('1000000000000000000000000') // 1 NEAR in yoctoNEAR
      });
      //safely checking the status
      return result.transaction_outcome?.outcome?.status !== 'Failure';
    } catch (e) {
      console.error('Failed to book car:', e);
      return false;
    }
  }