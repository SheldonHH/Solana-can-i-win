import struct

max_choosable_integer = 10
desired_total = 20

data = struct.pack("<ii", max_choosable_integer, desired_total)
with open("instruction_data.bin", "wb") as f:
    f.write(data)
