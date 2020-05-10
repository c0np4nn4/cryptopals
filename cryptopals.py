import base64
from binascii import hexlify, unhexlify
from Crypto.Cipher import AES

"""
{ 1_1.py }
@ name   : bytes_to_b64
@ param  : bytes
@ return : base64 encoded bytes
"""
def bytes_to_b64(input_bytes):
        output_bytes = base64.b64encode(input_bytes)
        return output_bytes


"""
{ 1_2.py }
@ name   : fixed_bytes_XOR
@ param  : bytes, bytes
@ return : bytes
"""
def fixed_bytes_XOR(one_bytes, two_bytes):
	output_bytes = bytes( [x ^ y for x, y in zip(one_bytes, two_bytes)] )
	output_bytes = hexlify(output_bytes)
	return output_bytes


"""
{ 1_3.py }
@ name   : int_to_byte
@ param  : int
@ return : byte
"""

def int_to_byte(num_bytes):
    key = hex(int(num_bytes.decode()))
    if len(key) % 2:
        key = "0" + key[2:]
    else:
        key = key[2:]
    key = bytes.fromhex(key)
    return key


"""
{ 1_3.py }
@ name   : character_scoring
@ param  : bytes
@ return : float
"""
def character_scoring(input_bytes):
	character_frequency = {
		'e':12.702, 't':9.356, 'a':8.167, 'o':7.507, 'i':6.966,
		'n':6.749, 's':6.327, 'h':6.094, 'r':5.987, 'd':4.253,
		'l':4.025, 'u':2.758, 'w':2.560, 'm':2.406, 'f':2.228,
		'c':2.202, 'g':2.015, 'y':1.994, 'p':1.929, 'b':1.492,
		'k':1.292, 'v':0.978, 'j':0.153, 'x':0.150, 'q':0.095,
		'z':0.077, ' ':30.0 , 
		'E':12.702,'T':9.356, 'A':8.167, 'O':7.507, 'I':6.966,
		'N':6.749, 'S':6.327, 'H':6.094, 'R':5.987, 'D':4.253,
		'L':4.025, 'U':2.758, 'W':2.560, 'M':2.406, 'F':2.228,
		'C':2.202, 'G':2.015, 'Y':1.994, 'P':1.929, 'B':1.492,
		'K':1.292, 'v':0.978, 'J':0.153, 'X':0.150, 'Q':0.095,
		'Z':0.077
	}

	score=0
	input_bytes = unhexlify(input_bytes)
	for byte in input_bytes:
		score += character_frequency.get(chr(byte), 0)
	return score


"""
{ 1_3.py }
@ name   : single_byte_XOR
@ param  : bytes, bytes
@ return : bytes
"""
def single_byte_XOR(msg_bytes, key_byte):	
		key_bytes = b''
		for j in range(int(len(msg_bytes))):
			key_bytes += key_byte
		result = fixed_bytes_XOR(msg_bytes, key_bytes)
		return result
        
"""
{ 1_5.py }
@ name   : repeating_key_XOR
@ param  : bytes, byte
@ return : bytes
"""
def repeating_key_XOR(msg_bytes, key_byte):
    repeating_key = key_byte * int(len(msg_bytes) / len(key_byte))
    if len(repeating_key) != len(msg_bytes):
        key_str = key_byte.decode()
        repeating_key += key_str[:len(msg_bytes) % len(key_byte)].encode()

    result_bytes = fixed_bytes_XOR(msg_bytes, repeating_key)
    return result_bytes 


"""
{ 1_6.py }
@ name   : get_hamming_distance
@ param  : bytes, bytes
@ return : int
"""

def get_hamming_distance(one_bytes, two_bytes):
        distance = 0
        for x, y in zip(one_bytes, two_bytes):

                diff = x ^ y
                distance += sum([1 for bit in bin(diff) if bit == '1'])
        return distance

"""
{ 1_7.py }
@ name   : AES_ECB
@ param  : bytes, bytes
@ return : bytes
"""
def AES_ECB(msg_bytes, key_bytes):
        aes = AES.new(key_bytes, AES.MODE_ECB)
        print("====================")
        print("     AES_ECB        ")
        print(" 1. Encrypt")
        print(" 2. Decrypt")
        print("====================")
        select = input(" Select >> ")
        if select == "1":
            result = aes.encrypt(msg_bytes)
        elif select == "2":
            result = aes.decrypt(msg_bytes)
        else:
            print("wrong")
        return result


"""
{ 1_9.py }
@ name   : PKCS7_padding
@ param  : bytes, int
@ return : bytes
"""
def PKCS7_padding(msg_bytes, block_size):
    o_len = len(msg_bytes)
    pad_count = block_size - (o_len % block_size)
    pad = hex(pad_count)[2:]
    if len(pad) < 2:
        pad = "0" + pad
    pad = bytes.fromhex(pad)
    result = msg_bytes
    print(type(result))
    for _ in range(pad_count):
        result += pad
    return result
