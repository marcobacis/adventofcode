from collections import namedtuple
from math import prod

Packet = namedtuple("Packet", ("version", "type_id", "value", "sub_packets"))

def hex_to_int(hex_str):
    return bin(int(hex_str, 16))[2:].zfill(len(hex_str) * 4)

def get_literal_value(transmission, i):
    bits = ""
    while True:
        is_last_group = transmission[i] == '0'
        bits += transmission[(i+1):(i+5)]
        i += 5
        if is_last_group:
            break
    return int(bits, 2), i

def get_packets_by_length(transmission, i):
    packets = []
    sub_packets_length = int(transmission[i : (i+15)], 2)
    i += 15
    length_used = 0
    while length_used < sub_packets_length:
        sub_packet, new_i = decode(transmission, i)
        length_used += new_i - i
        i = new_i
        packets.append(sub_packet)    
    return packets, i

def get_packets_by_number(transmission, i):
    num_packets = int(transmission[i : (i + 11)], 2)
    i += 11
    packets = []
    for _ in range(num_packets):
        sub_packet, new_i = decode(transmission, i)
        i = new_i
        packets.append(sub_packet)
    return packets, i

def decode(message, i=0):
    version = int(message[i : (i + 3)], 2)
    type_id = int(message[(i + 3) : (i + 6)], 2)
    i += 6
    if type_id == 4:
        value, i = get_literal_value(message, i)
        return Packet(version, type_id, value, []), i
    length_type_id = message[i]
    i += 1
    if length_type_id == '0':
        sub_packets, i = get_packets_by_length(message, i)
    else:
        sub_packets, i = get_packets_by_number(message, i)
    return Packet(version, type_id, compute_value(type_id, sub_packets), sub_packets), i

def compute_value(type_id, sub_packets):
    values = [p.value for p in sub_packets]
    if len(values) == 1:
        return values[0]
    if type_id == 0:
        return sum(values)
    if type_id == 1:
        return prod(values)
    if type_id == 2:
        return min(values)
    if type_id == 3:
        return max(values)
    if type_id == 5:
        return 1 if values[0] > values[1] else 0
    if type_id == 6:
        return 1 if values[0] < values[1] else 0
    if type_id == 7:
        return 1 if values[0] == values[1] else 0
    return 0

def sum_versions(packet):
    total = packet.version
    for sub_packet in packet.sub_packets:
        total += sum_versions(sub_packet)
    return total

def get_data(filename):
    with open(filename, 'r') as f:
        return [hex_to_int(line) for line in f.read().splitlines()]

examples = get_data('example.txt')
for example in examples:
    packet, _ = decode(example)
    print("Part 1: " + str(sum_versions(packet)) + ", Part 2: " + str(packet.value))

challenge = get_data('input.txt')[0]
challenge,_ = decode(challenge)
print(sum_versions(challenge))

print(challenge.value)