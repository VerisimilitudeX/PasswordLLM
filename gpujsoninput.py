import json

# Create an empty list to hold the GPU data
gpu_data = []

while True:
    # Prompt the user for the GPU data
    gpu_model = input("GPU Model: ")

    # Check if the user entered an empty string for the GPU model
    # If they did, exit the loop
    if gpu_model == "":
        break

    hashrate = input("Enter your hashrate: ")
    gflop = input("Enter GPFLOPs: ")

    # Create a dictionary with the data and append it to the list
    gpu = {
        "GPU Model": gpu_model,
        "gflop": gflop,
        "hashrate": hashrate
    }
    gpu_data.append(gpu)


def write_json(data, filename='data.json'):
    with open(filename, 'w') as file:
        json.dump(data, file, indent=4)


# Write the data to the JSON file
write_json(gpu_data)

print("Data saved to data.json")
