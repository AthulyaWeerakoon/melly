# Architecture

## Product contract

Melly is a programmable Linux desktop environment whose customization contract is HTML, CSS, and JavaScript. Normal web-platform concepts remain normal web-platform concepts; Melly adds semantic elements, attributes, events, and `melly.*` JavaScript operations only where desktop behavior has no suitable web equivalent.

The contract is:

> Melly guarantees complete programmability of the interface surface it defines.

Every exposed feature must be expressible through the desktop source contract and follow a documented semantic guarantee. Host behavior without a reliable abstraction remains optional behind a capability, diagnostic-only, or outside the public API.

Desktop authors customize Melly-owned interface behavior through HTML, CSS, and JavaScript. Wayland protocol code, compositor IPC, D-Bus, shell commands, Rust, and compositor configuration are outside the authoring contract. Packaging and permission metadata covers installation and authority only; it is not a visual-layout or interaction language.

## Durable boundaries

```text
Desktop source
HTML + CSS + JavaScript
        |
        v
Melly public contract
DOM/web behavior + Melly semantics + melly.* + capabilities
        |
        v
Melly runtime internals
Servo boundary + Wayland mediation + permissions + generations + recovery
        |
        v
Host integration
standard Wayland + host adapters + selected Linux services
        |
        v
Host compositor / operating system / hardware
```

The public contract remains stable across changes between standard Wayland protocols, an application-facing proxy, host adapters, and other Linux subsystems.

### Runtime and shell process boundary

The runtime and shell communicate through a per-user Unix-domain socket at `$XDG_RUNTIME_DIR/melly/runtime.sock`. The runtime owns the socket server, client identity, authorization, capability checks, command dispatch, events, and policy. The socket is ephemeral login-session state and is not stored in a repository, installation directory, persistent data directory, or shared temporary directory.

`rusty-melly` is the supported Rust client SDK for this boundary. The reference `melly-shell` uses it as an ordinary runtime client. Third-party native applications may use the same SDK to communicate directly with the runtime without routing through the shell process.

Bypassing the shell does not bypass Melly. Direct clients still negotiate the versioned protocol and pass runtime identity, capability, permission, and policy checks. The shell receives no private protocol or implicit authority. Shared wire primitives belong to `melly-protocol`; client transport and ergonomics belong to `rusty-melly`; server and policy behavior belong to `melly-runtime`.

The desktop HTML/CSS/JavaScript does not access the Unix socket. Its native boundary remains the permission-checked `melly.*` bridge exposed by the shell environment.

### Engine boundary

The future `MellyWebEngine` abstraction owns WebView creation, rendering, input delivery, resize/focus behavior, lifecycle, and local resource loading. Only this layer tracks Servo embedding API changes. Servo supplies the ordinary HTML/CSS/JavaScript environment. Melly extensions remain within that environment and do not form a parallel UI dialect.

### JavaScript and DOM boundary

Desktop code requests native intent through host-neutral `melly.*` operations and documented Melly-specific DOM elements, attributes, and events. Inputs, outputs, errors, object identities, and event names describe Melly domain concepts and do not expose Sway containers, raw Wayland objects, or another backend's wire format.

The eventual versioned contract should distinguish:

- a small minimum environment that every supported Melly host must honor;
- optional capabilities that are advertised only when both the host can provide them and the installed desktop has permission;
- unsupported behavior, which fails explicitly or uses documented browser-preview behavior without performing a different native action.

Capability checks are not permission grants. Every supported backend implements the minimum Melly contract. Backends that cannot implement it are unsupported.

### Application-facing Wayland direction

A rootless Wayland proxy/compositor is the current direction for Melly-managed application-window lifecycle, HTML-controlled chrome, application-surface placement, and input routing. Native Wayland applications connect to Melly, and Melly creates ordinary windows under an existing host compositor.

A Melly window domain object associates application state, a client surface, HTML decoration, and a host toplevel. Smithay is a candidate for Wayland server machinery. Servo is the intended web engine.

These are implementation choices to validate, not promises to desktop authors. Prototype evidence may refine the topology, dependency choices, buffer path, and WebView strategy without weakening or renaming the public customization contract.

### Managed and host-managed applications

Melly distinguishes compatibility from managed support:

- A **Melly-managed** application passes through the validated Melly application boundary and may receive the documented Melly window model, HTML chrome, events, and controls.
- A **host-managed** application connects directly to the host path. It remains usable, but Melly does not claim window mediation or other managed features for it.

Legacy X11 applications are host-managed from the first version. On the Sway reference host, Sway's XWayland support runs them directly. Melly does not proxy X11 or implement XWayland in the first version.

Application types, Wayland protocols, surface roles, and runtime conditions that the current Melly version cannot mediate safely use host-managed routing when authorized and supported by the host. Each routing decision is recorded in the structured compatibility log with the application identity when known, detected protocol or case, bypass reason, and relevant missing capability.

Host-managed applications are compatible but outside the full Melly support contract. They are absent from managed collections unless a future, explicitly limited diagnostic model includes them. Partial behavior is not represented as the full contract.

The fallback does not bypass launch authorization, permissions, sandbox policy, or host security. It also never means falling back from a failed JavaScript API call to arbitrary shell execution or compositor IPC.

### Authority boundaries

- Applications remain authoritative for their content and application-specific state.
- Melly is authoritative for Melly-owned interface surfaces, exposed domain objects, permissions, capabilities, desktop-source generations, and recovery policy.
- The host compositor remains authoritative for physical/global placement, global focus, actual output arrangement, system composition, physical input policy, and all host-managed applications unless a supported protocol explicitly delegates a semantic operation.
- The kernel and hardware remain authoritative for GPUs, displays, and input devices.

## Promise discipline

Before adding a public feature, document and test:

1. the semantic behavior Melly owns;
2. the minimum result a desktop author can rely on;
3. permission and capability requirements;
4. behavior when the operation is unavailable or fails;
5. whether browser preview can simulate it clearly without pretending a native action occurred;
6. how backend-specific data is prevented from leaking into the contract.

Known host limitations appear as explicit state, structured errors, absent capabilities, or logged host-managed routing decisions. Baseline X11 compatibility uses the host's XWayland path and does not include Melly-managed X11 windows. Uniform minimize/maximize behavior, client-decoration suppression, output control, managed XWayland integration, zero-copy composition, and other unproven integrations remain outside the contract.

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

Core coordination may depend on Melly-owned engine, bridge, Wayland, host, deployment, and IPC contracts. Concrete Servo, proxy, protocol, and compositor adapters implement those contracts. Public contracts must not import implementation-specific types in the opposite direction.

The shell depends on `rusty-melly`, not on runtime internals. The runtime and client SDK depend on shared `melly-protocol` wire primitives. `rusty-melly` does not depend on the reference shell, compositor adapters, or rendering-engine adapters.

## Security boundary

A desktop repository is executable software with a trusted visual position. Local rendering alone grants no shell execution, arbitrary filesystem access, unrestricted network access, or host control. The bridge checks installed repository identity and approved permissions before routing a request. Host adapters receive only authorized semantic requests.

Desktop JavaScript has no ambient authority to modify source files, arbitrary host files, processes, services, compositor state, sockets, or machine configuration. It may alter its DOM and in-memory state. Every native effect outside the renderer sandbox is expressed as a documented `melly.*` operation and must pass capability, permission, identity, and policy checks.

The directory containing the manifest entry document is the desktop resource root. Documents, modules, workers, stylesheets, templates, fonts, images, media, and other interface resources must canonically resolve to files inside that root. Candidate validation and runtime loading reject absolute filesystem paths, parent traversal, symlink escapes, redirects or encoded variants that leave the root, and time-of-check/time-of-use path substitutions. The final loader must use race-resistant path resolution rather than trusting string-prefix checks.

Network access is independent authority. A desktop with explicit network permission may issue AJAX requests to approved localhost or outbound endpoints for data, while remote scripts, modules, styles, markup, fonts, images, templates, and other interface assets remain prohibited. Responses are handled as untrusted data and do not expand filesystem or native authority.

A localhost helper is a separate application boundary. It authenticates and authorizes requests independently and acts with its own operating-system identity and permissions when changing desktop or other files. Melly does not lend, proxy, or delegate runtime privileges through a localhost request, and localhost is not implicitly trusted.

Git is the source and transaction history, not the security boundary. Validation, permission approval, isolation, health checks, atomic activation, and external recovery enforce deployment safety.
