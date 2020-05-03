from cryptopals import AES_ECB_decrypt
from binascii import unhexlify

def detect_AES_ECB(msg_bytes):
	block_size = 16
	chunks = [msg_bytes[i : i + block_size] for i in range(0, len(msg_bytes), block_size)]
	
	is_it_AES_ECB = len(chunks) - len(set(chunks))
	return is_it_AES_ECB

def main():
	f = open("./8.txt", "r")
	lines = f.readlines()
	msgs = [bytes.fromhex(lines[i].strip()) for i in range(len(lines))]
	
	ECB_list = []
	
	for i in range(len(msgs)):
		if detect_AES_ECB(msgs[i]):
			print("ECB index : {}".format(i + 1))
	



if __name__ == "__main__":
	main()


