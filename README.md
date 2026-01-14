# MyPC Information Tool
![111](https://github.com/user-attachments/assets/0bc87049-eceb-46eb-8152-53c90f2d6cba)

**MyPC** is a Rust utility designed to collect and display information about your computer's hardware components. The program retrieves data about the CPU, motherboard, GPU, RAM, and disks.

## Features

The program displays the following information:

- **CPU**:
  - Processor Name
  - Maximum Clock Speed
  
- **Motherboard**:
  - Manufacturer
  - Model
  - Serial Number

- **GPU**:
  - GPU Name
  - Video Memory Size

- **RAM**:
  - Manufacturer
  - Speed
  - Total Memory
  
- **Disks**:
  - Disk Model
  - Disk Size (in gigabytes)

## Installation and Running

### Requirements

- **Rust**: Make sure you have [Rust](https://www.rust-lang.org/) installed (you can install it via [rustup](https://rustup.rs/)).

### Steps to Run

1. Clone the repository to your computer:

    ```bash
    git clone https://github.com/Junsious/MyPC.git
    ```

2. Navigate to the project directory:

    ```bash
    cd MyPC
    ```

3. Build and run the program:

    ```bash
    cargo run
    ```

## How It Works

The program uses `powershell` to extract system information via WMI (Windows Management Instrumentation) commands. Data about the CPU, motherboard, GPU, RAM, and disks are processed and formatted before being displayed on the screen.


## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Contributors

If you would like to make changes or improvements to the project, feel free to create pull requests or open issues.

---

Feel free to reach out if you have any questions or suggestions!
