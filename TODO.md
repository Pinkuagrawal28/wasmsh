# TODO List for WASM Tools

The following tools were identified but not built due to limitations within the current WebAssembly environment or the scope of this project. They are listed here for future consideration if WASM capabilities evolve or project requirements change.

## Difficulty Levels:

*   **Very High:** Fundamentally incompatible with WASM sandbox model, or requires an entire interpreter/system.
*   **High:** Requires significant WASI extensions, complex logic, or might only be partially implementable.
*   **Medium:** Requires some WASI extensions or careful handling of WASM limitations, moderate complexity.
*   **Low:** Could be implemented with current WASM/WASI capabilities, straightforward logic.

## Skipped Tools by Difficulty:

### Very High Difficulty

-   `cd`
    -   **Reason:** Requires direct shell integration to change the parent shell's directory, not feasible for a standalone WASM executable.
    -   **Notes:** Would need to be a `wasmsh` shell built-in or require `wasmsh` to interpret its output to change its own directory.
-   `ps`
    -   **Reason:** Requires access to host system process information, which is not available to a WASM module.
    -   **Notes:** Fundamentally incompatible with WASM's sandboxed nature.
-   `top`
    -   **Reason:** Requires real-time access to host system process and resource information, and advanced interactive terminal control, not feasible for a WASM module.
    -   **Notes:** Fundamentally incompatible with WASM's sandboxed nature.
-   `free`
    -   **Reason:** Requires access to host system memory statistics, which is not available to a WASM module.
    -   **Notes:** Fundamentally incompatible with WASM's sandboxed nature.
-   `shutdown`
    -   **Reason:** Requires system-level privileges to halt or reboot the host system, not possible from a WASM sandbox.
    -   **Notes:** Fundamentally incompatible with WASM's sandboxed nature.
-   `systemctl`
    -   **Reason:** Requires interaction with the host system's service manager (systemd), not possible from a WASM sandbox.
    -   **Notes:** Linux-specific and fundamentally incompatible with WASM's sandboxed nature.
-   `ifconfig` (or `ip`)
    -   **Reason:** Requires low-level access to network interface configuration, not available to a WASM module.
    -   **Notes:** Fundamentally incompatible with WASM's sandboxed nature.
-   `ssh`
    -   **Reason:** Requires complex network protocols, cryptography, and interactive terminal sessions, not feasible for a WASM module.
    -   **Notes:** Requires direct TCP/IP socket access and a full SSH client implementation, which is not typically available or portable in WASM.
-   `scp`
    -   **Reason:** Relies on SSH protocol for secure file transfer, subject to the same limitations as `ssh`.
    -   **Notes:** Same as `ssh`.
-   `chown`
    -   **Reason:** Requires system-level privileges to change file ownership on the host system, not possible from a WASM sandbox.
    -   **Notes:** Fundamentally incompatible with WASM's sandboxed nature.
-   `chgrp`
    -   **Reason:** Requires system-level privileges to change file group ownership on the host system, not possible from a WASM sandbox.
    -   **Notes:** Fundamentally incompatible with WASM's sandboxed nature.
-   `sudo`
    -   **Reason:** Requires privilege escalation and user management on the host system, not possible from a WASM sandbox.
    -   **Notes:** Fundamentally incompatible with WASM's sandboxed nature.
-   `awk`
    -   **Reason:** A powerful pattern scanning and processing language; a full interpreter implementation is a very large project.
    -   **Notes:** Implementing a full `awk` interpreter is a massive undertaking, essentially building a language runtime within WASM.

### High Difficulty

-   `less`
    -   **Reason:** Requires advanced interactive terminal control and continuous display updates, which are complex for a standalone WASM module.
    -   **Notes:** Would need a robust terminal emulation library and careful handling of input/output streams. Might be partially implementable as a simple pager.
-   `man`
    -   **Reason:** Requires access to a manual page database and interactive display, which is complex to implement in a WASM sandbox.
    -   **Notes:** Would need to either embed a large dataset of man pages or have a mechanism to fetch them, plus interactive display.
-   `sed`
    -   **Reason:** A complex stream editor with extensive regular expression and transformation capabilities; a full implementation is a significant undertaking.
    -   **Notes:** Requires a full regular expression engine and complex text manipulation logic. A simplified version (e.g., basic find/replace) might be feasible.
-   `ping`
    -   **Reason:** Requires raw network socket access for ICMP, which is typically restricted in WASM environments.
    -   **Notes:** Could potentially be implemented using WASI networking extensions for TCP/UDP, but true ICMP `ping` is unlikely.