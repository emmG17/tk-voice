# TK Voice - Rust CLI Tool

**TK Voice** is a Rust command-line interface (CLI) tool that leverages TikTok's text-to-speech API to convert text into speech. With this tool, you can create audio files using various voices provided by TikTok's API.

## Prerequisites

Before you begin using **TK Voice**, ensure that you have the following prerequisites installed on your system:

1. Rust: You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

2. Git: If you haven't already, download and install Git from the [official website](https://git-scm.com/).

## Installation

To get started with **TK Voice**, follow these steps:

1. Clone the repository:

   ```shell
   git clone https://github.com/yourusername/tk-voice.git
   ```

2. Change to the project directory:

   ```shell
   cd tk-voice
   ```

3. Build the CLI tool:

   ```shell
   cargo build --release
   ```

## Usage

After building the **TK Voice** CLI tool, you can use it to convert text into speech using different voices. Here's how to use it:

```shell
./target/release/tk-voice <voice> <text> <session_id> <output_file_name>
```

- `<voice>`: Specify the voice to use (e.g., "default_voice").
- `<text>`: Provide the text you want to convert to speech.
- `<session_id>`: Your TikTok session ID.
- `<output_file_name>`: Specify the name of the output audio file (without the file extension).

Example:

```shell
./target/release/tk-voice default_voice "Hello, world!" your_session_id output
```

This command will generate an MP3 file named `output.mp3` containing the speech generated from the provided text using the specified voice.

## Important Notes

- Ensure you have a stable internet connection when using this tool, as it relies on TikTok's API to generate speech.

- You need a valid TikTok session ID to access the API. Replace `your_session_id` with your actual session ID.

- Keep in mind that this tool interacts with the TikTok API, which may change over time. Make sure you comply with TikTok's terms of service and usage policies.

## License

This project is licensed under the MIT License. Refer to the [LICENSE](LICENSE) file for details.

---

Please consider contributing to this project or reporting any issues on the [GitHub repository](https://github.com/yourusername/tk-voice). Enjoy using the **TK Voice** Rust CLI tool!
