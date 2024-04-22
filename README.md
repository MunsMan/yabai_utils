# Yabai Utils

**Yabai Utils** is a tool designed to streamline and simplify interactions with the Yabai window manager. It provides a set of utilities that abstract complex command invocations into simpler, more manageable functions. The primary goal of this project is to enhance productivity by reducing the complexity involved in managing window configurations and navigation.

## Motivation

The motivation behind **Yabai Utils** stems from the desire to make [Yabai](https://github.com/koekeishiya/yabai) macOS most powerful window management more accessible. The goal is to abstract complex commands behind a simple interface for easier [skhd](https://github.com/koekeishiya/skhd) integration. With **Yabai Utils**, users can efficiently manage their workspace with minimal effort and learning curve.

## Features

Currently, **Yabai Utils** focuses on enhancing window navigation, including support for floating windows. It simplifies the commands necessary to interact with the Yabai window manager, making it more accessible and easier to use for everyday tasks.

### Current Capabilities:

- **Window Focusing**: Simplify the focusing of windows, including floating ones, with easy-to-use commands.
- **Space Focusing**: Focus on any space by passing a direction (`left`, `right`) or just an index.
  - When using directions, cycling is through all spaces is the default behavior
  - When using indexes, you always go to your desired space. If it doesn't exist, yabai-utils will create it for you.

## Getting Started

**Yabai Utils** is designed to be easy to install and use, especially with support for Nix, providing a reproducible development and deployment environment. This makes it easy for users to get up and running without worrying about dependency conflicts.

### Prerequisites

- macOS operating system
- Yabai window manager installed
- Nix package manager (for using the Nix flake)

### Installation

#### Using Nix Flake

If you are using Nix, you can easily integrate **Yabai Utils** into your environment with:

```sh
nix develop github:munsman/yabai-utils
```

#### Manual Installation

Clone this repository to your local machine using:

```sh
git clone https://github.com/munsman/yabai-utils.git
cd yabai-utils
cargo build --release
```

Follow the setup instructions to integrate **Yabai Utils** with your Yabai installation.

### Usage

To use the simplified window navigation commands, run:

```sh
# Example command to navigate through windows
yabai-utils window focus <direction>
```

direction = `up`, `down`, `left`, `right`

## Roadmap

**Yabai Utils** is actively being developed with the following features planned for future releases:

- **Cycling Windows**: Easily cycle through windows within the current space.
- **Managing Spaces**: Commands to manage and cycle through Yabai spaces efficiently.
- **Focus Consistency**: Enhancements to focus management to only consider visible windows and automatically remove empty spaces.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you would like to contribute:

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request
