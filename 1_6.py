import base64
from binascii import hexlify, unhexlify
from cryptopals import int_to_byte, character_scoring, single_byte_XOR

def get_hamming_distance(one_bytes, two_bytes):
    #print(one_bytes, two_bytes)
    distance = int(one_bytes.decode(), 16) ^ int(two_bytes.decode(), 16)
    #print(bin(distance))
    score = 0
    for b in bin(distance):
        if b == '1':
            score += 1

    return score 




def main():
    f = open("./6.txt", "r")
    msg = ""

    for line in f:
        msg += (line.strip())
    msg_hex_bytes = hexlify(base64.b64decode(msg))
    
    m = msg_hex_bytes

    hamming_distance = [65536, 65536]

    for size in range(2, 40, 2):
        score = 0

        for i in range(5):
            score += get_hamming_distance(m[i * size : (i + 1) * size], m[(i + 1) * size : (i + 2) * size] ) / size
        hamming_distance.append(score)




    msgs = []
    for _ in range(3):
        key_size = hamming_distance.index(min(hamming_distance))
        hamming_distance[key_size] = 65536
        print(key_size)

        for i in range(key_size):
            msg = b''
            for j in range(int(len(m) / key_size) - 1):
                msg += m[i + key_size * 2 * j : i + key_size * 2 * j + 1]
            msgs.append(msg)

    '''
    -------------------------------------------------------------------------------------------------------------------
    1, 4, 7, 
    '''

        for i in range(key_size):
            scores = []
            results = []
            for j in range(0, 127):
                key_byte = int_to_byte(str(j).encode())
                result = single_byte_XOR(msgs[i], key_byte)
                scores.append(character_scoring(result))
                results.append(result)

            print(unhexlify(results[scores.index(max(scores))]))

        


if __name__ == "__main__":
    main()
