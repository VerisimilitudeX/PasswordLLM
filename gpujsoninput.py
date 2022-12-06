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

        if format_input in acceptable_formats or format_input == '/s'.join(acceptable_formats):
            # format_input is in acceptable_formats
            break  # successful program
        else:
            # format_input is not in acceptable_formats
            print("Incorrect Input")  # try again

    gflop = input("Enter GPFLOPs: ")

    # Create a dictionary with the data and append it to the list
    gpu_dict = {
        f"{gpu_model}": {
            "GFLOPS-64": gflop,
            f"{ALGO} HASHRATE": hashrate + f"/S"}
    }

gpu_data.append(gpu_dict)


def write_json(data, filename='data.json'):
    with open(filename, 'a+') as file:
        if 'GPUs' not in file:
            data.update({"GPUs", })
    json.dump(data, file, indent=4)


# Write the data to the JSON file
write_json(gpu_data)

print("Data saved to data.json")
