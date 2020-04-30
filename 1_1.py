from cryptopals import bytes_to_b64
import base64

def main():
	msg_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
	msg_bytes = bytes.fromhex(msg_hex)
	
	result = bytes_to_b64(msg_bytes)
	
	print(result.decode())	

if __name__ == "__main__":
	main() 
