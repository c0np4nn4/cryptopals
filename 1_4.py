from cryptopals import int_to_byte, single_byte_XOR, character_scoring
from binascii import unhexlify, b2a_hex


def main():
    f = open("./4.txt", "r")
    msgs = []
    answers = []
    for line in f:
        msgs.append(line.strip())
   

    score = []
    results = []
    for i in range(len(msgs)):
        msg = msgs[i]
        msg_bytes = bytes.fromhex(msg)
        for i in range(128):
            num_bytes = str(i).encode()
            key_byte = int_to_byte(num_bytes)
            result = single_byte_XOR(msg_bytes, key_byte)
    
            results.append(result)
            score.append(character_scoring(result))
        max_index_num = score.index(max(score))
        answers.append(results[max_index_num])
    
    score = []
    for i in range(len(answers)):
        score.append(character_scoring(answers[i]))
    max_index_num = score.index(max(score))
    
    result = unhexlify(answers[max_index_num].decode())
    print(result)
    

if __name__ == "__main__":
        main()
