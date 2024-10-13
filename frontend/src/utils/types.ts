import { Signer } from 'near-api-js';

// Extend the Signer from near-api-js if it's the one you're using
export abstract class NEARSigner extends Signer {
    abstract getAccountId(): Promise<string>;
}

export interface NEARSpecificSigner {
    getAccountId(): Promise<string>;
}