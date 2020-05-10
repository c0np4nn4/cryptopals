import base64
from Crypto.Cipher import AES
from cryptopals import AES_ECB

def main():
	f = open("./7.txt", "r")
	lines = f.readlines()
	msg = base64.b64decode(''.join(lines))
	key = b'YELLOW SUBMARINE'

	print(AES_ECB(msg, key))	


if __name__ == "__main__":
	main()
