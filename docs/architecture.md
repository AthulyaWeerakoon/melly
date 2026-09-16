# Architecture

## Product contract

Melly is a programmable Linux desktop environment whose customization contract is HTML, CSS, and JavaScript. Normal web-platform concepts remain normal web-platform concepts. `melly.*` describes Melly itself, including environment, granted permissions, configuration, and runtime status. `desktop.*` exposes the programmable scene and client surfaces. Permission-gated `system.*` exposes operating-system services. Servo is the web runtime and Smithay supplies the application-facing Wayland server machinery.

The contract is:

> Melly guarantees complete programmability of the interface surface it defines.

Every exposed feature must be expressible through the desktop source contract and follow a documented semantic guarantee. Host behavior without a reliable abstraction remains optional behind a capability, diagnostic-only, or outside the public API.

Desktop authors customize Melly-owned interface behavior through HTML, CSS, and JavaScript. Wayland protocol code, compositor IPC, D-Bus, shell commands, Rust, and compositor configuration are outside the authoring contract. Packaging and permission metadata covers installation and authority only; it is not a visual-layout or interaction language. The design target places approximately 90% of visible desktop UI policy inside this programmable scene; application-owned pixels and physical hardware state remain external constraints.

## Durable boundaries

```text
Desktop source
HTML + CSS + JavaScript
        |
        v
Melly public contract
DOM/web behavior + melly.* + desktop.* + system.*
        |
        v
Melly Core and scene
Servo layout + client surfaces + input routing + permissions + generations
        |
        v
Application-facing Wayland server
Smithay + xdg-shell + native Wayland client lifecycle
        |
        v
Display-host presentation
standard Wayland client + Sway/KWin/Labwc + operating system/hardware
```

The public contract remains stable across changes to standard Wayland protocols, display-host adapters, and other Linux subsystems.

### Runtime, command shell, and desktop host boundary

The runtime and native clients communicate through a per-user Unix-domain socket at `$XDG_RUNTIME_DIR/melly/runtime.sock`. The runtime owns the socket server, client identity, authorization, capability checks, command dispatch, events, and policy. The socket is ephemeral login-session state and is not stored in a repository, installation directory, persistent data directory, or shared temporary directory.

`rusty-melly` is the supported Rust client SDK for this boundary. The Melly command shell uses it as an ordinary runtime client to control Melly. Third-party native applications may use the same SDK without routing through the command shell. The desktop host is a distinct presentation process that embeds Servo, displays the desktop HTML/CSS/JavaScript, and supplies the three JavaScript namespaces. It may use the same SDK where a runtime client boundary is required, but it does not become the command shell.

Bypassing the command shell does not bypass Melly. Direct clients still negotiate the versioned protocol and pass runtime identity, capability, permission, and policy checks. The command shell receives no private protocol or implicit authority. Shared wire primitives belong to `melly-protocol`; client transport and ergonomics belong to `rusty-melly`; server and policy behavior belong to `melly-runtime`. These checks are design requirements, not features of the current connection-only SDK.

The desktop HTML/CSS/JavaScript does not access the Unix socket. Its native boundaries are `melly.*` for Melly self-description and configuration, `desktop.*` for scene interaction, and `system.*` for permission-gated operating-system services. Existing web concepts continue to use standard web APIs.

### Engine boundary

The future `MellyWebEngine` abstraction owns WebView creation, rendering, input delivery, resize/focus behavior, lifecycle, and local resource loading. Only this layer tracks Servo embedding API changes. Servo supplies the ordinary HTML/CSS/JavaScript environment. Melly extensions remain within that environment and do not form a parallel UI dialect.

The engine adapter may use available GPU rendering and compositing. Melly does not force a software renderer globally; an engine may select a software fallback when the graphics stack lacks a usable accelerated path. The desktop host and bridge coordinate surfaces and authorized operations without duplicating the engine's HTML/CSS layout or frame rendering work. Renderer evaluation covers ordinary browser output, the embedded engine, native client-surface interaction, and a software fallback at comparable viewport sizes.

### JavaScript and DOM boundary

Desktop code uses documented DOM elements, attributes, events, and `desktop.*` operations for scene and client-surface semantics. `melly.*` is reserved for Melly's own environment, granted permissions, configuration, and runtime state. Files, application inventory and launch, audio, power, devices, and similar operating-system services belong under `system.*` when no standard web API fits. All three namespaces remain semantic and host-neutral; they do not expose Sway containers, raw Wayland objects, D-Bus messages, commands, or backend wire formats.

The eventual versioned contract should distinguish:

- a small minimum environment that every supported Melly host must honor;
- optional capabilities that are advertised only when both the host can provide them and the installed desktop has permission;
- unsupported behavior, which fails explicitly or uses documented browser-preview behavior without performing a different native action.

Capability checks are not permission grants. Every supported backend implements the minimum Melly contract. Backends that cannot implement it are unsupported.

### Application-facing Wayland architecture

A rootless/nested Wayland compositor is the Melly application model. Native Wayland applications connect to Melly. The display host sees Melly's presentation surface or surfaces rather than managing each application as an independent host window.

A `ClientSurface` represents application-owned presentation content. It is the fundamental foreign-UI primitive exposed to the desktop scene. Smithay handles the underlying Wayland surfaces, subsurfaces, `xdg_toplevel` and popup lifecycle, configure cycles, buffers, seats, focus, clipboard, drag-and-drop, and input protocol state. Servo supplies the surrounding document, layout, CSS, JavaScript, DOM, and web storage.

The HTML bridge is the `<client-surface>` custom element. Servo layout assigns its geometry; Melly configures the corresponding Wayland client and presents the application buffer inside that region. A reference desktop may wrap it in conventional HTML window chrome, place it in a CSS grid, or use another visual model. Conventional windows, workspaces, tiling, floating, title bars, docks, and launchers are not mandatory Melly Core primitives.

### Application support boundary

Melly supports native Wayland applications that connect through its application-facing server and can participate in the Melly scene. Unsupported protocols fail explicitly; Melly does not silently route their clients directly to the display host.

X11 and host-owned XWayland bypass are not supported. A future X11 compatibility design would require a Melly-owned XWayland path whose surfaces enter the same client-surface model. It is outside the current contract and implementation scope.

### Scene and input routing

Servo layout participates directly in desktop composition policy. For each client-surface element, Melly reads the laid-out region, configures the Wayland client to the corresponding size, clips and composes its submitted buffer into the scene, and updates it when layout changes.

Pointer and keyboard input are routed according to the scene hit test. Input over ordinary HTML is delivered to Servo and the DOM. Input over a client-surface region is translated into the application-facing Wayland seat and delivered to that client. Focus, popups, cursor state, clipboard, and drag-and-drop remain coordinated Core responsibilities rather than desktop-author configuration.

### Web storage and source identity

Standard `sessionStorage`, `localStorage`, `IndexedDB`, and `navigator.storage` semantics are the default desktop persistence interface where Servo supports them. Persistent storage is tied to stable installed desktop identity so it survives reloads, restarts, reboots, and Git generation swaps. It lives outside individual source generations.

Browser storage holds private UI and application state; it is not the authority for live client surfaces, outputs, installed applications, devices, or user files. Git stores desktop source. The normal user filesystem remains separate and requires an explicit permission-checked native API.

### Authority boundaries

- Applications remain authoritative for their content and application-specific state.
- Melly is authoritative for Melly-owned interface surfaces, exposed domain objects, permissions, capabilities, desktop-source generations, and recovery policy.
- The display host remains authoritative for physical outputs, actual output arrangement, device hotplug, GPU/display ownership, DRM/KMS, refresh, hardware presentation, and physical input devices.
- The kernel and hardware remain authoritative for GPUs, displays, and input devices.

## Promise discipline

Before adding a public feature, document and test:

1. the semantic behavior Melly owns;
2. the minimum result a desktop author can rely on;
3. permission and capability requirements;
4. behavior when the operation is unavailable or fails;
5. whether browser preview can simulate it clearly without pretending a native action occurred;
6. how backend-specific data is prevented from leaking into the contract.

Known limitations appear as explicit state, structured errors, or absent capabilities. X11 compatibility, managed XWayland integration, output control, zero-copy composition, and other unimplemented integrations remain outside the contract. Client-requested maximize and fullscreen state remain protocol realities, while their visual presentation is desktop policy expressed through HTML/CSS/JavaScript where the protocol permits.

## Runtime flows

### Development reload

1. A file watcher observes the desktop working tree.
2. Changes are debounced and validated.
3. The affected document or WebView reloads.
4. The host compositor, client applications, and Melly process remain running wherever the validated architecture permits.

State-preserving hot-module replacement is not required initially; predictable document reload is the baseline. Runtime implementation changes still require rebuilding Melly, but desktop HTML/CSS/JavaScript changes do not.

### Committed generation activation

1. The deployment controller notices that the configured branch head changed.
2. It materializes the commit as an isolated candidate generation.
3. Metadata, local sources, requested permissions, and prohibited dependencies are validated.
4. Candidate runtime state is initialized and health-checked.
5. A healthy candidate is switched in atomically; a failed candidate leaves the active generation untouched.

The runtime retains current, previous, known-good, and factory-safe recovery options. Emergency recovery must exist outside user-controlled HTML.

## Dependency direction

Core coordination may depend on Melly-owned engine, bridge, Wayland, host, deployment, and IPC contracts. Concrete Servo, Wayland-server, protocol, and display-host adapters implement those contracts. Public contracts must not import implementation-specific types in the opposite direction.

The command shell depends on `rusty-melly`, not on runtime internals. The runtime and client SDK depend on shared `melly-protocol` wire primitives. `rusty-melly` does not depend on the command shell, desktop host, compositor adapters, or rendering-engine adapters.

## Security boundary

A desktop repository is executable software with a trusted visual position. Local rendering alone grants no shell execution, arbitrary filesystem access, unrestricted network access, or host control. The bridge checks installed repository identity and approved permissions before routing a request. Host adapters receive only authorized semantic requests.

Desktop JavaScript has no ambient authority to modify source files, arbitrary host files, processes, services, compositor state, sockets, or machine configuration. It may alter its DOM and in-memory state. Every native effect outside the renderer sandbox is expressed through a documented API in the appropriate namespace and must pass capability, fine-grained manifest permission, identity, and policy checks.

The directory containing the manifest entry document is the desktop resource root. Documents, modules, workers, stylesheets, templates, fonts, images, media, and other interface resources must canonically resolve to files inside that root. Candidate validation and runtime loading reject absolute filesystem paths, parent traversal, symlink escapes, redirects or encoded variants that leave the root, and time-of-check/time-of-use path substitutions. The final loader must use race-resistant path resolution rather than trusting string-prefix checks.

Network access is independent authority. A desktop with explicit network permission may issue AJAX requests to approved localhost or outbound endpoints for data, while remote scripts, modules, styles, markup, fonts, images, templates, and other interface assets remain prohibited. Responses are handled as untrusted data and do not expand filesystem or native authority.

A localhost helper is a separate application boundary. It authenticates and authorizes requests independently and acts with its own operating-system identity and permissions when changing desktop or other files. Melly does not lend, proxy, or delegate runtime privileges through a localhost request, and localhost is not implicitly trusted.

Git is the source and transaction history, not the security boundary. Validation, permission approval, isolation, health checks, atomic activation, and external recovery enforce deployment safety.
