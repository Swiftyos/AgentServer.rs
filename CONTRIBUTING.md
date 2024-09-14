# Contributing Guide

Welcome to the Scalable Graph Execution System project! We value efficient, high-quality contributions that help us move fast without compromising on correctness or reliability. This guide focuses on development processes, tools, and best practices to ensure you can deliver bug-free and correct code quickly.

---

## Table of Contents

- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
  - [Branching Strategy](#branching-strategy)
  - [Issue Tracking](#issue-tracking)
- [Tools and Environment Setup](#tools-and-environment-setup)
  - [Essential Tools](#essential-tools)
  - [Optimizing Development Speed](#optimizing-development-speed)
- [Coding Standards](#coding-standards)
  - [Code Style](#code-style)
  - [Error Handling](#error-handling)
- [Testing Guidelines](#testing-guidelines)
  - [Writing Effective Tests](#writing-effective-tests)
  - [Test-Driven Development](#test-driven-development)
  - [Continuous Testing](#continuous-testing)
- [Ensuring Code Correctness](#ensuring-code-correctness)
  - [Static Analysis and Linting](#static-analysis-and-linting)
  - [Automated Code Formatting](#automated-code-formatting)
  - [Peer Review](#peer-review)
- [Performance Optimization](#performance-optimization)
  - [Benchmarking](#benchmarking)
  - [Profiling Tools](#profiling-tools)
- [Documentation Best Practices](#documentation-best-practices)
- [Submitting Your Contribution](#submitting-your-contribution)
- [Maintaining High Velocity](#maintaining-high-velocity)
- [Additional Resources](#additional-resources)
- [Contact](#contact)

---

## Getting Started

Before diving in, ensure you have a solid understanding of the project's architecture and goals:

- **Read the [README](README.md)**: Familiarize yourself with the system overview and architecture.
- **Understand the Key Components**: Know how the services interact and the technologies used.
- **Set Up Your Environment**: Follow the setup instructions to get the project running locally.

---

## Development Workflow

Our development workflow is designed to maximize speed and efficiency while maintaining code quality.

### Branching Strategy

- **Main Branch**: Contains stable, production-ready code.
- **Development Branch**: Use the `develop` branch for integrating features before they are merged into `main`.
- **Feature Branches**: Create a feature branch from `develop` for your work:

  ```
  git checkout -b feature/short-description develop
  ```

### Issue Tracking

- **Use GitHub Issues**: Before starting, make sure there is an issue assigned to you.
- **Define the Scope**: Clearly understand the task to avoid scope creep.
- **Break Down Tasks**: If possible, split large tasks into smaller, manageable sub-tasks.

---

## Tools and Environment Setup

Leveraging the right tools is crucial for rapid and efficient development.

### Essential Tools

- **Rust Toolchain**: Install via [rustup](https://rustup.rs).
- **Editor/IDE**: Use an IDE with Rust support (e.g., VSCode with rust-analyzer).
- **Clippy**: Linting tool to catch common mistakes:

  ```bash
  rustup component add clippy
  ```

- **Rustfmt**: Automated code formatting:

  ```bash
  rustup component add rustfmt
  ```

- **Cargo-edit**: Manage dependencies easily:

  ```bash
  cargo install cargo-edit
  ```

- **Cargo-watch**: Automatically run tests or rebuild on code changes:

  ```bash
  cargo install cargo-watch
  ```

- **Cargo-nextest**: Fast, parallel test runner:

  ```bash
  cargo install cargo-nextest
  ```

- **Cargo-audit**: Check for security vulnerabilities:

  ```bash
  cargo install cargo-audit
  ```

- **Insta**: Snapshot testing for Rust:

  ```bash
  cargo install cargo-insta
  ```

### Optimizing Development Speed

- **Hot Reloading**: Use `cargo-watch` to automatically compile and run tests when files change:

  ```bash
  cargo watch -x 'test --all --no-fail-fast'
  ```

- **IDE Integration**: Ensure your IDE is set up for Rust development with features like auto-completion, inline error checking, and code navigation.

- **Pre-Commit Hooks**: Use Git hooks to automate checks before committing:

  - **Formatting and Linting**: Run `cargo fmt` and `cargo clippy`.
  - **Tests**: Run a quick test suite to catch immediate issues.

---

## Coding Standards

Adhering to consistent coding standards speeds up development and reduces errors.

### Code Style

- **Follow Rust Best Practices**: Use idiomatic Rust patterns.
- **Consistent Formatting**: Use `rustfmt` to automatically format your code.
- **Meaningful Names**: Use clear and descriptive names for variables, functions, and modules.
- **Avoid Code Smells**: Refactor code that is complex or difficult to understand.

### Error Handling

- **Use `anyhow` for Application Errors**: Simplifies error handling in application code.
- **Use `thiserror` for Library Errors**: Provides clear and maintainable error types in libraries.
- **Contextual Errors**: Add context to errors using `.context()` to make debugging easier.

---

## Testing Guidelines

Writing effective tests ensures your code works as intended and reduces bugs.

### Writing Effective Tests

- **Unit Tests**: Test individual functions and methods.
- **Integration Tests**: Test the interaction between different parts of the system.
- **Edge Cases**: Don't forget to test edge cases and potential error conditions.
- **Use Mocks and Fakes**: When testing components that interact with external systems, use mocks to simulate behavior.

### Test-Driven Development

- **Write Tests First**: Start by writing a failing test that defines the desired functionality.
- **Implement Code**: Write the minimal code required to pass the test.
- **Refactor**: Improve the code while ensuring tests still pass.

### Continuous Testing

- **Automate Testing**: Use `cargo-watch` to run tests automatically on code changes.
- **Fast Feedback Loop**: Keep tests fast to maintain development velocity.
- **Use `cargo-nextest`**: For running tests in parallel, speeding up the test suite.

---

## Ensuring Code Correctness

Ensuring that your code is correct from the start saves time on debugging and rework.

### Static Analysis and Linting

- **Clippy**: Run Clippy to catch common mistakes and improve code quality.

  ```bash
  cargo clippy --all -- -D warnings
  ```

- **Fix Warnings Immediately**: Treat warnings as errors to keep the codebase clean.

### Automated Code Formatting

- **Rustfmt**: Run Rustfmt before committing to ensure consistent code style.

  ```bash
  cargo fmt --all
  ```

- **Editor Integration**: Configure your editor to format on save.

### Peer Review

- **Code Reviews**: Before merging, have at least one other team member review your code.
- **Review for Correctness and Style**: Focus on logic errors, potential bugs, and adherence to coding standards.
- **Be Constructive**: Provide helpful feedback and be open to suggestions.

---

## Performance Optimization

Efficient code is critical for high-performance systems.

### Benchmarking

- **Use Criterion.rs**: For accurate and reliable benchmarks.

  ```rust
  use criterion::{criterion_group, criterion_main, Criterion};

  fn bench_my_function(c: &mut Criterion) {
      c.bench_function("my_function", |b| b.iter(|| my_function()));
  }

  criterion_group!(benches, bench_my_function);
  criterion_main!(benches);
  ```

- **Benchmark Early**: Identify performance bottlenecks during development, not after deployment.

### Profiling Tools

- **Cargo Flamegraph**: Visualize where your code spends time.

  ```bash
  cargo install flamegraph
  cargo flamegraph --bin my_binary
  ```

- **Heap Profiling with `dhat`**: Detect memory leaks and optimize memory usage.

---

## Documentation Best Practices

Clear documentation accelerates development and onboarding.

- **Inline Documentation**: Use `///` comments for public APIs.
- **Examples**: Provide code examples where applicable.
- **Update Documentation**: Keep documentation up-to-date with code changes.
- **Use `cargo doc`**: Generate documentation and review it for completeness.

  ```bash
  cargo doc --open
  ```

---

## Submitting Your Contribution

- **Ensure All Tests Pass**: Run the full test suite before submitting.

  ```bash
  cargo test --all
  ```

- **Verify Linting and Formatting**:

  ```bash
  cargo fmt --all -- --check
  cargo clippy --all -- -D warnings
  ```

- **Update Documentation**: If your changes affect public APIs or functionality, update the relevant documentation.

- **Create a Pull Request**:

  - **Descriptive Title**: Summarize the changes.
  - **Detailed Description**: Explain what changes were made and why.
  - **Reference Issues**: Link to any related issues.

- **Respond to Feedback Promptly**: Address review comments to keep the process moving.

---

## Maintaining High Velocity

To develop quickly without sacrificing quality:

- **Stay Focused**: Work on one task at a time to completion.
- **Avoid Over-Engineering**: Implement what's needed for the task, avoiding unnecessary complexity.
- **Leverage Existing Code**: Reuse utilities and functions from the common crate.
- **Automate Repetitive Tasks**: Use scripts or tools to handle repetitive work.

---

## Additional Resources

- **Rust Documentation**: [The Rust Programming Language](https://doc.rust-lang.org/book/)
- **Tokio Tutorial**: [Asynchronous Programming in Rust](https://tokio.rs/tokio/tutorial)
- **Effective Testing**: [Rust Testing Book](https://rust-lang.github.io/rustc-guide/how-to-test.html)
- **Performance Tips**: [Rust Performance Book](https://nnethercote.github.io/perf-book/)

---

## Contact

If you have questions or need assistance:

- **Slack**: Join our [Slack Workspace](https://join.slack.com/t/projectworkspace/shared_invite/...) and reach out in the `#development` channel.
- **Email**: Contact the maintainers at `maintainer@example.com`.

---

Thank you for contributing to the Scalable Graph Execution System! Your expertise and efforts are crucial to the project's success. Let's build something amazing together—fast and right the first time.