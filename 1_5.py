from binascii import hexlify 
from cryptopals import fixed_bytes_XOR, repeating_key_XOR

def main():

    '''
    #    At first, I tried to XOR msg and repeated_key in hex values
    #    BUT, result was not same with the one on web page. (cryptopals.com/sets/1/challenges/5)
    #    So, I should have to implement this in just bytes.

    msg =  "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbali"
    msg_bytes = hexlify(msg.encode())

    key = "ICE"
    key_byte = hexlify(key.encode())
    # 1. make the len(key_byte) equal to the len(msg_bytes)
    repeating_key  = key_byte * int(len(msg_bytes) / len(key_byte))
    if len(repeating_key) != len(msg_bytes):
        repeating_key += key_byte[:len(msg_bytes) % len(key_byte)]

    print(len(repeating_key))
    print(msg_bytes)

    result = fixed_bytes_XOR(msg_bytes, repeating_key)
    print(result)
    '''

    
    msg = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal" 
    key = "ICE"
    
    msg_bytes = msg.encode()
    key_byte = key.encode()

    result = repeating_key_XOR(msg_bytes, key_byte)
    print(result)


if __name__ == "__main__":
    main()
