from binascii import hexlify, unhexlify
import base64
from cryptopals import single_byte_XOR, character_scoring, int_to_byte, repeating_key_XOR

def get_hamming_distance(one_bytes, two_bytes):
	distance = 0
	for x, y in zip(one_bytes, two_bytes):
		
		diff = x ^ y
		distance += sum([1 for bit in bin(diff) if bit == '1'])
	return distance


def main():
	f = open("./6.txt", "r")
	lines = f.readlines()
	msg_bytes = base64.b64decode(''.join(lines))
	m = msg_bytes
	

	hamming_distances = [65535,65535]	

	for size in range(2, 40):
		score = 0
		count = 0
		for i in range(len(m)//size):
			chunk = []
			chunk.append(m[(i+0) * size : (i+1) * size])
			chunk.append(m[(i+1) * size : (i+2) * size])
			dist = get_hamming_distance(chunk[0], chunk[1])
			score += (dist / size)
			del chunk[1]
			del chunk[0]
			count += 1
		hamming_distances.append(score / count)

	
	key_size = hamming_distances.index(min(hamming_distances))
	print("KEYSIZE: {}".format(key_size))

	
	transposed_msg_bytes = [b'' for i in range(key_size)]
	print(len(transposed_msg_bytes))
	for offset in range(key_size):
		for i in range(0, len(msg_bytes), key_size ):
			index = offset + i 
			transposed_msg_bytes[offset] += msg_bytes[index:index+1]


	for i in range(key_size):
		print(len(transposed_msg_bytes[i]))

	
	key = ""
	
	for index in range(len(transposed_msg_bytes)):
		score = []
		results = []
		t = transposed_msg_bytes
		for i in range(128):
			num_bytes = str(i).encode()
			key_byte = int_to_byte(num_bytes)
			result = single_byte_XOR(t[index], key_byte)

			results.append(result)
			score.append(character_scoring(result))
		max_index_num = score.index(max(score))
		key += chr(max_index_num)

	key_bytes = key.encode()	

	result = repeating_key_XOR(msg_bytes, key_bytes)
	print(unhexlify(result))

if __name__ == "__main__":
	main()
