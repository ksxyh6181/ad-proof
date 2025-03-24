from nacl.signing import SigningKey
 
import base58
 
def generate_solana_wallet_nacl():
    signing=SigningKey.generate()
    public_key=base58.b58encode(signing.verify_key.encode()).decode('utf-8') 
    private_key=base58.b58encode(signing._signing_key).decode('utf-8')
    print("public:{}".format(public_key))
    print("private_key:{}".format(private_key))

if __name__ == "__main__":
    generate_solana_wallet_nacl()