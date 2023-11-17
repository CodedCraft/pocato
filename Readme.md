# Pocato - Your CLI Task Manager


<!-- Logo ----------------------------------------------------------------------------------------->
<p align="center">
  <img src="assets/pocato_logo.png" alt="Tocapo Logo" width="250" />
</p>
<!-- Logo ----------------------------------------------------------------------------------------->


## Table of Contents

  - [Introduction](#introduction)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Feedback](#feedback)
  - [Tech Stack](#tech-stack)
  - [Roadmap](#roadmap)
  - [License](#license)


## Introduction

Welcome to Pocato, a simple command-line interface tool designed to help you manage your tasks 
and stay organized. The current release is a Minimum Viable Product (MVP) and consists of just the 
core task manager application (todo app). The vision is to tightly integrate this task manager with 
the Pomodoro Technique and a calendar, creating a comprehensive productivity ecosystem. There are 
big plans for the future, including web and desktop applications, so stay tuned for exciting 
updates!

## Features

- **Task Creation:** Easily create tasks with a title.
- **Task List:** View a list of all your tasks, including their status.
- **Task Status:** Mark task as started (but not completed), blocked, cancelled or paused.
- **Task Completion**: Mark task as complete.
- **Task Deletion:** Remove tasks that are no longer needed.

## Installation (Linux)

To use Pocato, you need to have rust and cargo installed on your system.
(If not, I recommend install both with [rustup](https://rustup.rs/))
1. Clone the repository and cd into the root of the project:
   ```bash
    git clone https://github.com/CodedCraft/pocato.git
    cd pocato
    ```
2. Build with cargo:
   ```bash
   cargo build --bin "pct" --release
    ```
3. Move to compiled app into the bin folder:
    ```bash
    sudo mv target/release/pct /usr/bin/
    ```
3. Run tocapo with the 'help' command to learn all commands (or read [Usage](#Usage) section below):
    ```bash
    pct help
    ```
## Usage
Here are all commands to get you started:

 | Command + <argument>         | Description                  |
 |------------------------------+------------------------------|
 | `pct add <Enter task name>`  | Create a new task.           |
 | `pct show`                   | Show all tasks the database. |
 | `pct show <Enter task ID>`   | Show a specific task.        |
 | `pct start <Enter task ID>`  | Start a specific task.       |
 | `pct block <Enter task ID>`  | Block a specific task.       |
 | `pct cancel <Enter task ID>` | Cancel a specific task.      |
 | `pct pause <Enter task ID>`  | Pause a specific task.       |
 | `pct finish <Enter task ID>` | Check off a specific task.   |
 | `pct delete <Enter task ID>` | Delete a specific task.      |
 | `pct help`                   | Show the help menu.          |
 |------------------------------+------------------------------|

## Feedback
I value your feedback! If you encounter any issues, have suggestions, or want to report a bug, 
please feel free to create an issue in the GitHub repository.

## Tech Stack

The CLI Task Manager App is built using the following technologies and tools:

- **Rust Programming Language:** Rust was chosen for its strong emphasis on safety, speed, and 
performance, making it an excellent choice for building a reliable task manager.

- **Clap for CLI Parsing:** Currently, Clap library is used for command-line parsing.
Clap simplifies command-line argument parsing and provides a great user experience. Custom solutions 
may be explored in the future for better integration.

- **SQLite as Database:** The application currently stores task data in an SQLite database. This 
choice was made for its simplicity and portability. Plans include migrating to PostgreSQL in the 
future to support a distributed task management system.

Stay tuned for updates as I continue to enhance and optimize the technology stack to provide you 
with a better task management experience.

## Roadmap

While this MVP CLI version provides only the essential task management functionality, exciting plans 
are in store for the future:

- **Web Application:** A user-friendly web application with an intuitive interface is in the works.
- **Desktop Application:** A desktop version is planned for even more convenience.
- **Sync and Backup:** Stay organized across devices with task synchronization and data backup.
- **Customization:** Customize your task manager with themes and categories.

Keep an eye on our GitHub repository and website for updates and release announcements!

<!-- ## Contributing -->
<!---->
<!-- If you're interested in contributing to the project or have ideas for new features, please check our  -->
<!-- [Contribution Guidelines](CONTRIBUTING.md). -->

## License

At the moment the project is licensed under CreativeCommons Attribution-NonCommercial-NoDerivs 4.0 
International license (CC BY-NC-ND). This project will be relicensed in the future under a different 
license. CC BY-NC-ND was chosen to keep all options for relicensing open. For details, please see 
the [LICENSE](LICENSE) file.

---

Thank you for choosing Tocapo! I hope it helps you stay organized and productive. If you have any 
questions, feel free to reach out in the GitHub discussion tab.

Happy task management! 🚀📋
