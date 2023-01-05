# RealPass Initial Research

## Quick important information

Passwords are most commonly cracked using GPUs
Data for password cracking is hard to come by, but I think we are able just to use cryptocurrency mining data instead, which is in abundance.
FLOPS (Floating Point Operations per second) is the number of calculations a device can make in a second, also known as math problems.
HPS (Hashes per second) is the number of guesses (password guesses) a device can make on a specific algorithm per second
Passwords hashed in specific algorithms must be decrypted using the same algorithm it was hashed in

We may be able to create a database of popular graphics cards and their FLOPS then find out how many FLOPS it takes for every hash in every algorithm.

## Most common password hashing algorithms in order of most to least common
|ALGO|DESCRIPTION|
|:-|:-|
SHA-1 SHA-256, SHA384|Difficult to Crack
MD5|Easier to crack
BCrypt|Best for security, Hard to crack
Argon2|Hard to crack

If we are able to use GPU mining data, it will be substantially easier to implement a lot of the advanced features.

### Nvidia GPUs
#
Nvidia GPUs are the most common GPUs used for mining, and are the most common GPUs used for password cracking. They are also the most common GPUs used for gaming, so they are the most common GPUs in general.

<!--GFLOPS for Nvidia GPUs-->
1 GFLOP is approx 127.451434323 MH for MD5 

    127.4514343231001509813789632612

|GPU|GFLOPS (FP64)|
|:-|:-|
|RTX 3090 Ti|625|
|RTX 3090 |556|
|RTX 3080|465|
|RTX 3070 Ti| 339|
|RTX 3070 |317|
|RTX 2070 Super|283|
|RTX 2080 Ti |420|
|RTX 3060 Ti |253|
|RTX 2060 Super |224|
|RTX 2060 |201|
|RTX 3060 |199|
|RTX 1660 Ti |169|
|RTX 1660 Super |157|
|RTX 4090 |1290|
|RTX 3080 Ti|532|
<!---
can u sort the GFLOPS in this array?
--->
## Converting FLOPS of GPUs

FLOPS is the number of floating-point calculations a device can make in a second, also known as math problems. We can calculate the FLOPS by using the following formula:
GFLOPS = (FLOPS * 1000000000) / 2
TFLOPS = (FLOPS * 10000000000) / 2

## Calculating HPS of GPUs

HPS is the number of guesses (password guesses) a device can make on a specific algorithm per second. We can calculate the HPS of a GPU by using the following formula:

HPS = FLOPS / (Hashes per second of algorithm)

## Calculating time to crack

Time to crack = (Number of hashes in database) / (HPS of GPU)

## Calculating time to crack with multiple GPUs

Time to crack = (Number of hashes in database) / (HPS of GPUs)

### FP64

(SMs * GHz) * 4 = FP64 Performance in GFLOPS
SMs = Stream Multiprocessors
GHz = Frequency of individual CUDA Cores
FP64 = (Double) Precision Floating-Point Format

An example would be,
3090Ti has 84 Stream Multiprocessors
3090Ti has a boost clock of 1.86GHz
(84 * 1.86) * 4 = 624.96 (FP64) GFLOPS
Real-world result: 625.0 GFLOPS

### FP32

Double Precision Flops Rating = Clock frequency x CUDA Cores x 2 x Clock cycles
X = Clock Cycles
I’m unsure of what the relevance of Clock Cycles are in this case but it doesn’t seem to be relevant as I get the correct answer without it.
((GHz * Core) * 2) * X

An example would be,
1.86 (Boost clock speed)   *10752 (Core count) = 19,998.72
19,998.72* 2 = 39,997.44 TFLOPS

Real-world result:  40.00 TFLOPS

I believe we should use FP64 as it’s more precise in the number of decimals it has… Though this is irrelevant if we are just randomly guessing. Eh, I found this article saying most hashing algorithms do not use floating-point operations and only use integers, so the data above may be irrelevant. Maybe we have to use IPS (Instructions per second)?

    "NVIDIA believes in modern games relying heavily on integer math. With the new architecture, we still have the 64 FP32 cores, but another 64 cores are now designated as “FP32 and INT32”, making half the cores capable of doing either floating-point or integer calculations. Since password recovery relies on integer math, we can effectively utilize half the Ampere cores. You can read a tech article about the new Ampere architecture at Engadget."

I believe we need to calculate the amount of instructions to complete one round of the hashing algorithm, for a hashing algo like SHA256, on average it takes 64 - 80 Clock Cycles to complete on round.

    To estimate the performance of MD5-crypt on a given architecture, we first define
    a simple model that is based on the number of arithmetic instructions needed to
    complete one round of the password hashing scheme. Since it is not very hard to
    estimate the instruction throughput of a hardware platform, this model can be
    used to compare the performance of the hashing scheme on different architectures
    and determines the maxim speedup that can be achieved. To define this model,
    all arithmetic and logic operations are taken into account

## Real-world HASHCAT 3090-Ti benchmarks algorithm’s results

MD5 - 65079.1 MH/s
MD5  (pass + salt) - 66252.7 MH/s
MD5 (salt + pass) - 37131.6 MH/s
SHA-1 -  22757.6 MH/s
SHA-256 (pass + salt) - 9746.6 MH/s

And a lot more I did not include because there was so much.
Data collected from Hashcat v6.1.1 benchmark on the Nvidia RTX 3090 · GitHub
Sources:
<https://www.ru.nl/publish/pages/769526/thesis.pdf>
<https://www.hivesystems.io/blog/are-your-passwords-in-the-green>
<https://developer.okta.com/blog/2019/07/29/hashing-techniques-for-password-storage>
<https://www.tomshardware.com/news/eight-rtx-4090s-can-break-passwords-in-under-an-hour>
<https://www.azcalculator.com/calc/GPU-gflops-calculator.php>
<https://gist.github.com/Chick3nman/e4fcee00cb6d82874dace72106d73fef>
