from binascii import unhexlify
from cryptopals import PKCS7_padding

def main():
    msg_bytes = b"YELLOW SUBMARINES"
    block_size = 8
    print(PKCS7_padding(msg_bytes, block_size))


if __name__ == "__main__":
    main()
