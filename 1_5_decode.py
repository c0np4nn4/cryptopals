from binascii import hexlify, unhexlify
from cryptopals import fixed_bytes_XOR, repeating_key_XOR

def main():

    #msg = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal" 
    msg_bytes = b'\x0b67\'*+.cb,.ii*#i:*<c$ -b=c4<*&"c$\'\'e\'*(+/ C\ne.,e*1$3:e>+ \'c\x0ci+ (1e(c&0.\'(/'
    key = "ICE"
    
    #msg_bytes = msg.encode()
    key_byte = key.encode()

    result = repeating_key_XOR(msg_bytes, key_byte)
    print(unhexlify(result.decode()))


if __name__ == "__main__":
    main()
