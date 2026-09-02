# Architecture

Melly is a native shell runtime placed between a desktop source repository, Servo, an existing Wayland compositor, and selected Linux services.

```text
Desktop repository (HTML/CSS/JS + melly.toml)
                         |
                         v
                  Melly core runtime
                 /          |          \
                v           v           v
        engine boundary   deployment   capability bridge
                |                       |
                v                       v
             Servo                host capability router
                                         |
                           +-------------+-------------+
                           v             v             v
                       Sway IPC    generic Wayland   Linux services
```

## Stable boundaries

### Engine boundary

The future `MellyWebEngine` abstraction owns WebView creation, rendering, input delivery, resize/focus behavior, lifecycle, and local resource loading. Only this layer should track Servo embedding API changes.

### JavaScript capability API

Desktop code requests intent through namespaced operations such as `melly.apps.launch(...)` or `melly.windows.focus(...)`. API inputs, outputs, and errors must be host-neutral.

### Host integration

The host layer reports available capabilities and fulfils authorized semantic requests. Prefer focused providers such as windows, workspaces, outputs, and shortcuts over one interface that pretends every compositor supports the same features.

## Runtime flows

### Development reload

1. A file watcher observes the desktop working tree.
2. Changes are debounced and validated.
3. The affected document or WebView reloads.
4. The compositor and Melly process remain running.

State-preserving hot-module replacement is not required for the first implementation; predictable document reload is the baseline.

### Committed generation activation

1. The deployment controller notices that the configured branch head changed.
2. It materializes the commit as an isolated candidate generation.
3. Manifest, sources, and requested permissions are validated.
4. Candidate runtime state is initialized and health-checked.
5. A healthy candidate is switched in atomically; a failed candidate leaves the active generation untouched.

The runtime retains current, previous, known-good, and factory-safe recovery options.

## Dependency direction

Core coordination may depend on Melly-owned engine, bridge, host, and deployment contracts. Concrete Servo and compositor adapters implement those contracts. The contracts must not import implementation-specific types in the opposite direction.

## Security boundary

A desktop repository is executable software with a trusted visual position. Local rendering alone grants no shell execution, arbitrary filesystem access, unrestricted network access, or host control. The bridge checks the installed repository identity and declared permissions before routing a request. Host adapters receive only authorized requests.
