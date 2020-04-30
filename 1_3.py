from cryptopals import int_to_byte, single_byte_XOR, character_scoring
from binascii import unhexlify, b2a_hex


def main():
	msg = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
	msg_bytes = bytes.fromhex(msg)
	score = []
	results = []
	for i in range(128):
		num_bytes = str(i).encode()
		key_byte = int_to_byte(num_bytes)
		result = single_byte_XOR(msg_bytes, key_byte)
		results.append(result)
		score.append(character_scoring(result))
	max_index_num = score.index(max(score))
	print(unhexlify(results[max_index_num].decode()))
	


if __name__ == "__main__":
	main()
