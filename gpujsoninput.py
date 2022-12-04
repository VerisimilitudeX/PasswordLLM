import json

# Create an empty list to hold the GPU data
gpu_data = []

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
        format_input = []
        for letter in range(0, len(hashrate)):
            if hashrate[letter].isalpha():  # if char in input is alpha
                format_input.append(hashrate[letter])  # put char into array
        format_input = "".join(format_input)  # join every element of char into 1 array string

        print(format_input)

        acceptable_formats = ['H', 'KH', 'MH', 'GH', 'TH']  # list of acceptable hashes

        for x in acceptable_formats:  # for every element in array
            if format_input == acceptable_formats or '/s'.join(
                    acceptable_formats):  # if the format_input matches any one of the ones in acceptable_formats or
                # acceptable_formats with a "/s" added onto it
                break  # sucessful program
            else:
                print("Incorrect Input")  # try again

    gflop = input("Enter GPFLOPs: ")

    # Create a dictionary with the data and append it to the list
    gpu_dict = "GPUs", {
        f"{gpu_model}": {
            "GFLOPS-64": gflop,
            f"{ALGO} HASHRATE": hashrate + f"H/S"}
    }

    gpu_data.append(gpu_dict)


def write_json(data, filename='data.json'):
    with open(filename, 'w') as file:
        json.dump(data, file, indent=4)


# Write the data to the JSON file
write_json(gpu_data)

print("Data saved to data.json")
