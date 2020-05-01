from binascii import hexlify 
from cryptopals import fixed_bytes_XOR
def main():

    '''
    #    At first, I tried to XOR msg and repeated_key in hex values
    #    BUT, result was not same with the one on web page. (cryptopals.com/sets/1/challenges/5)
    #    So, I should have to implement this in just bytes.

    msg =  "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbali"
    msg_bytes = hexlify(msg.encode())

    key = "ICE"
    key_bytes = hexlify(key.encode())
    # 1. make the len(key_bytes) equal to the len(msg_bytes)
    repeating_key  = key_bytes * int(len(msg_bytes) / len(key_bytes))
    if len(repeating_key) != len(msg_bytes):
        repeating_key += key_bytes[:len(msg_bytes) % len(key_bytes)]

    print(len(repeating_key))
    print(msg_bytes)

    result = fixed_bytes_XOR(msg_bytes, repeating_key)
    print(result)
    '''

    
    msg = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal" 
    key = "YELLO MACHINE"
    
    msg_bytes = msg.encode()
    key_bytes = key.encode()
   
    print("[+] msg : " + str(msg_bytes))
    print("[+] original key : " + str(key_bytes))


    repeating_key = key_bytes * int(len(msg_bytes) / len(key_bytes))
    if len(repeating_key) != len(msg_bytes):
        key_str = key_bytes.decode()
        repeating_key += key_str[:len(msg_bytes) % len(key_bytes)].encode()

    print("[+] Key : " + str(repeating_key))
    
    print("[+] msg_len : " + str(len(msg_bytes)))
    print("[+] key_len : " + str(len(repeating_key)))

    result_bytes = fixed_bytes_XOR(msg_bytes, repeating_key)
    print(result_bytes)
if __name__ == "__main__":
    main()
