const bs58 = require('bs58');
const crypto = require('crypto');
const elliptic = require('elliptic');
const EC = elliptic.ec;
const ec = new EC('secp256k1');
const secp256k1 = new EC('secp256k1');

function deriveEpsilon(accountId, walletId) {
    // Implementation for deriving epsilon. 
    // Replace this with the actual implementation.
    // This is just a placeholder as the specific method of derivation is not provided.
    return crypto.createHash('sha256').update(accountId + walletId).digest();
}

function deriveKey(publicKey, epsilon) {
    // Decode base58 public key
    const publicKeyBuffer = bs58.decode(publicKey);

    // Derive a key using HMAC and the provided epsilon
    const key = crypto.createHmac('sha256', epsilon).update(publicKeyBuffer).digest();

    // Convert key to a point on the curve
    const keyPoint = secp256k1.keyFromPublic("022e0ab4358780d72a4cac22ab75b257757663e5b204d96ccdce62f780536dac6a", 'hex').getPublic();

    // const EC = require('elliptic').ec
    // const pk = ec.keyFromPublic()

    return keyPoint;
}

// Example usage
const accountId = 'mydev.near'; // Replace with actual account ID
const walletId = "m/44'/397'/0'/0'/1"; // Replace with actual wallet ID
const publicKey = 'JCt63KiFDFm3c2ZVQaV45ySNAdCKbpd8ErPuUVyr2jiV'; // Replace with actual public key


const epsilon = deriveEpsilon(accountId, walletId);
const pk = deriveKey(publicKey, epsilon);

console.log(epsilon);
