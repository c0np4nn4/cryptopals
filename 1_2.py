from cryptopals import fixed_bytes_XOR

def main():
	one = "1c0111001f010100061a024b53535009181c"
	two = "686974207468652062756c6c277320657965"
	
	one_bytes = bytes.fromhex(one)
	two_bytes = bytes.fromhex(two)
	
	result = fixed_bytes_XOR(one_bytes, two_bytes)
	
	print(result.decode())

if __name__ == "__main__":
	main()
