import base64
from binascii import hexlify

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

