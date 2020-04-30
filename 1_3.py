from cryptopals import fixed_bytes_XOR
from binascii import unhexlify, b2a_hex


def main():
	msg = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
	msg_bytes = bytes.fromhex(msg)
	score = []
	results = []
	for i in range(128):
		num_bytes = str(i).encode()
		key_byte = single_byte_XOR_key_generator(num_bytes)
		result = single_byte_XOR(msg_bytes, key_byte)
		results.append(result)
		score.append(character_scoring(result))
	max_index_num = score.index(max(score))
	print(unhexlify(results[max_index_num].decode()))
	
		
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


def single_byte_XOR_key_generator(num_bytes):
	key = hex(int(num_bytes.decode()))
	if len(key) % 2:
		key = "0" + key[2:]
	else:
		key = key[2:]
	key = bytes.fromhex(key)
	return key


def single_byte_XOR(msg_bytes, key_byte):	
		key_bytes = b''
		for j in range(int(len(msg_bytes))):
			key_bytes += key_byte
		result = fixed_bytes_XOR(msg_bytes, key_bytes)
		return result






if __name__ == "__main__":
	main()
