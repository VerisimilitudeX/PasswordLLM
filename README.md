# PasswordLLM (formerly RealPass)
Is your password **_strong_** or just **_wrong_**? A free and open source program to test your password against realistic and real-world scenarios.

PasswordLLM is a free and open source password testing tool that uses realistic and real-world scenarios to evaluate the strength of your passwords.

## Features

- Uses advanced algorithms and attack simulations to estimate how long it would take an attacker to guess your password.
- Includes a dictionary of common passwords to check against.
- Works offline with limited functionality
- Checks lists of breached passwords

## Installation

To install PasswordLLM, follow these steps:

1. Download the latest release from the [PasswordLLM releases page](https://github.com/VerisimilitudeX/PasswordLLM/releases).
2. Extract the downloaded archive.
3. Run the run.bat file in order to launch the Powershell script with the correct permissions, the Powershell script will install the binary file.

## Usage

To use PasswordLLM, run the Powershell installer script by right clicking it and pressing "Run with Powershell" Or for an offline-mode, just run the "PasswordGPT-64x.exe" file.

PasswordGPT will then evaluate the password and display the estimated time it would take an attacker to guess it. It will also provide suggestions for improving the strength of your password.

## Build from source

### Dependencies
1. [Nvidia Cuda Toolkit](https://developer.nvidia.com/cuda-downloads)
2. Create a system enviornment variable to `C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.3\lib\x64` after completing step 1
## License
MIT

###### Note: This software is a joint collaboration between @LimesKey & @VerisimilitudeX.
