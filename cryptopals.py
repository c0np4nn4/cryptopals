import base64

"""
>> 1_1.py
@param  : bytes
@return : base64 encoded bytes
"""
def bytes_to_b64(input_bytes):
        output_bytes = base64.b64encode(input_bytes)
        return output_bytes
