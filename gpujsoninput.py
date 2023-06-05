import json

ALGO = input("Enter Algorithm: ").upper()

while True:
    # Prompt the user for the GPU data
    gpu_model = input("GPU Model: ").upper()

    # Check if the user entered an empty string for the GPU model
    # If they did, exit the loop
    if gpu_model == "":
        break

    while True:
        hashrate = str(input("Enter your hashrate: ")).upper()
        unacceptable_formats = ['/', '\\', 'S']
        for char in range(0, len(hashrate)):
            if hashrate[char] in unacceptable_formats:
                hashrate[char].replace(hashrate[char], '')

            if hashrate[char].isalpha():  # if char in input is alpha
                unit = hashrate[char].upper()  # put alaphabet into array

        if unit is None:
            print("Please enter a valid hashrate with the unit of the hashrate.")
            continue
        acceptable_formats = ['H', 'KH', 'MH', 'GH', 'TH']  # list of acceptable hashes
        count = 0
        for values in acceptable_formats:
            if unit == values:
                count++
        print(count)
        if count != 1:
            # format_input is not in acceptable_formats
            print("Incorrect Input")
            continue
            # try again

        gflop = input("Enter GPFLOPs: ")
        if gflop != "":
            break

    # Create a dictionary with the data and append it to the list
    gpu_dict = {
        f"{gpu_model}": {
            "GFLOPS-64": gflop,
            f"{ALGO} HASHRATE": (hashrate + unit + "/s")}
    }


def write_json(data, filename='data.json'):
    # Read the file
    with open(filename, 'r') as file:
        file_data = json.load(file)

    # Check if the array exists in the file
    found = False
    for i, subarr in enumerate(file_data):
        if subarr[0] == data[0]:
            found = True
            break

    if found:
        # Array exists, so modify the dictionary contained in the second element of the subarray
        file_data[i][1].update(data[1])
    else:
        # Array does not exist, so add it to the file
        file_data.append(data)
    # Write the modified data to the file
    with open(filename, 'w') as file:
        json.dump(file_data, file, indent=4)


# Write the data to the JSON file
write_json(gpu_dict)
print("Data saved to data.json")
