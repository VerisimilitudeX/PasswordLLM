import json




while True:
    i = + 1
    gpu_model = input("GPU Model: ")
    hashrate = input("Enter your hashrate: ")
    gflop = input("Enter GPFLOPs")

    if gpu_model == "":
        break

    file_data = {
        "GPU Model": gpu_model,
        "gflop": gflop,
        "hashrate": hashrate
    }

    def write_json(new_data, filename='data.json'):
        with open(filename, 'r+') as file:
            # First we load existing data into a dict.
            file_data = json.load(file)
        # Join new_data with file_data inside emp_details
        file_data["GPUs"].append(new_data)
        # Sets file's current position at offset.
        file.seek(0)
        # convert
        json.dump(file_data, file, indent=4)


    write_json(file_data)
    print("Write complete")
    print(f"Times {i}\n")
