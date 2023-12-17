[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/__xb4cFP)

# Go Project

## Go linter

This project follows the [Go linter](https://pkg.go.dev/golang.org/x/lint/golint) setup for linting Go code.

### Installation

To install Go linter and its dependencies, use the following commands:

```bash
# Install Go linter globally (if not installed)
go install golang.org/x/lint/golint@latest 
```

### Usage

To run Go linter and lint the project, use the following command:

```bash
# Run Go linter
golint ./...
```

## Coding Standard

This project adheres to the [Effective Go](https://golang.org/doc/effective_go.html) guidelines for maintaining a consistent coding style.

## Installation and Execution

1. Ensure you have Go installed. If not, download and install it from [Go official website](https://golang.org/dl/).
2. Open a terminal or command prompt.
3. Navigate to the project directory.
4. (Optional but recommended) Add the "bin" directory to your PATH:

    On Unix/Linux/macOS, add the following line to your shell profile file (e.g., `~/.bashrc`, `~/.zshrc`):

    ```bash
    export PATH=$PATH:$(pwd)/bin
    ```

    On Windows, you can add the "bin" directory to the system PATH via the System Properties > Advanced > Environment Variables.

5. Execute the program:

    ```bash
    go run riceCooker.go
    ```

## Creating a Module for Testing

This project uses Go modules to manage dependencies and facilitate testing. To create a module for testing, follow these steps:

1. Open a terminal or command prompt.
2. Navigate to the project directory.

    ```bash
    cd path/to/your/project
    ```

3. Run the following command to initialize a Go module:

    ```bash
    go mod init rice_cooker
    ```

   This command creates a `go.mod` file to manage the module and its dependencies.

## Running Tests

To run tests for the project, execute the following command:

```bash
go test 
```