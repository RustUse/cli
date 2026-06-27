#![allow(dead_code)]

pub(crate) const MAX_CRATES_IO_CATEGORIES: usize = 5;

/// Current crates.io category slug snapshot.
///
// Keep this list sorted because `is_valid_category_slug` uses binary search.
// For the latest category slugs, see <https://crates.io/category_slugs>
pub(crate) const VALID_CATEGORY_SLUGS: &[&str] = &[
    // Assistive technology that helps overcome disabilities and impairments to make software usable by as many people as possible.
    "accessibility",
    // Crates for aeronautics within the atmosphere and astronautics in outer space applications.
    "aerospace",
    // Crates related to multicopters, fixed wing, VTOL, and airships or balloons.
    "aerospace::drones",
    // Crates of protocol implementations for aerospace applications.
    "aerospace::protocols",
    // Crates related to simulations used in aerospace, including fluids and aerodynamics.
    "aerospace::simulation",
    // Protocol implementations for implications in space like CCSDS.
    "aerospace::space-protocols",
    // Crates related to unmanned aerial vehicles like multicopters, fixed wing, VTOL, airships, balloons, rovers, boats, and submersibles.
    "aerospace::unmanned-aerial-vehicles",
    // Rust implementations of core algorithms such as hashing, sorting, searching, and more.
    "algorithms",
    // Idiomatic wrappers of specific APIs for convenient access from Rust.
    "api-bindings",
    // Crates for machine learning, deep learning, large language models, AI agents, and related tooling.
    "artificial-intelligence",
    // Crates to help deal with events independently of the main program flow.
    "asynchronous",
    // Crates to help with the process of confirming identities.
    "authentication",
    // Crates related to the automotive industry, including vehicle control and diagnostics.
    "automotive",
    // Crates to store the results of previous computations in order to reuse the results.
    "caching",
    // Crates to help create command line interfaces.
    "command-line-interface",
    // Applications to run at the command line.
    "command-line-utilities",
    // Compiler implementations, including interpreters and transpilers.
    "compilers",
    // Algorithms for making data smaller.
    "compression",
    // Crates for comprehending the world from video or images.
    "computer-vision",
    // Crates for implementing concurrent and parallel computation.
    "concurrency",
    // Crates to facilitate configuration management for applications.
    "config",
    // Algorithms intended for securing data.
    "cryptography",
    // Crates for digital currencies, wallets, and distributed ledgers.
    "cryptography::cryptocurrencies",
    // Rust implementations of particular ways of organizing data suited for specific purposes.
    "data-structures",
    // Crates to interface with database management systems.
    "database",
    // Database management systems implemented in Rust.
    "database-implementations",
    // Crates to manage the inherent complexity of dealing with the fourth dimension.
    "date-and-time",
    // Crates that provide developer-facing features such as testing, debugging, linting, profiling, autocompletion, and formatting.
    "development-tools",
    // Utilities for build scripts and other build time steps.
    "development-tools::build-utils",
    // Subcommands that extend the capabilities of Cargo.
    "development-tools::cargo-plugins",
    // Crates to help figure out what is going on with code, such as logging, tracing, or assertions.
    "development-tools::debugging",
    // Crates to help interface with other languages.
    "development-tools::ffi",
    // Crates to help write procedural macros in Rust.
    "development-tools::procedural-macro-helpers",
    // Crates to help figure out the performance of code.
    "development-tools::profiling",
    // Crates to help verify the correctness of code.
    "development-tools::testing",
    // Crates to help with sending, receiving, formatting, and parsing email.
    "email",
    // Crates that are primarily useful on embedded devices or without an operating system.
    "embedded",
    // Emulators that allow one computer to behave like another.
    "emulators",
    // Encoding and/or decoding data from one data format to another.
    "encoding",
    // Direct Rust FFI bindings to libraries written in other languages.
    "external-ffi-bindings",
    // Crates for dealing with files and filesystems.
    "filesystem",
    // Crates for dealing with money, accounting, trading, investments, taxes, banking, and payment processing.
    "finance",
    // Crates that focus on some individual part of accelerating game development.
    "game-development",
    // Crates that try to provide a one-stop-shop for game development.
    "game-engines",
    // Applications for fun and entertainment.
    "games",
    // Crates for graphics libraries and applications, including raster and vector graphics primitives such as geometry, curves, and color.
    "graphics",
    // Crates to help create a graphical user interface.
    "gui",
    // Crates to interface with specific CPU or other hardware features.
    "hardware-support",
    // Crates to help develop software capable of adapting to various languages and regions.
    "internationalization",
    // Crates to help adapt internationalized software to specific languages and regions.
    "localization",
    // Crates with a mathematical aspect.
    "mathematics",
    // Crates to help with allocation, memory mapping, garbage collection, reference counting, or foreign memory interfaces.
    "memory-management",
    // Crates that provide audio, video, and image processing or rendering engines.
    "multimedia",
    // Crates that record, output, or process audio.
    "multimedia::audio",
    // Crates that encode or decode binary data in multimedia formats.
    "multimedia::encoding",
    // Crates that process or build images.
    "multimedia::images",
    // Crates that record, output, or process video.
    "multimedia::video",
    // Crates dealing with higher-level or lower-level network protocols.
    "network-programming",
    // Crates that are able to function without the Rust standard library.
    "no-std",
    // Crates that are able to function without the Rust alloc crate.
    "no-std::no-alloc",
    // Bindings to operating system-specific APIs.
    "os",
    // Bindings to Android-specific APIs.
    "os::android-apis",
    // Bindings to FreeBSD-specific APIs.
    "os::freebsd-apis",
    // Bindings to Linux-specific APIs.
    "os::linux-apis",
    // Bindings to macOS-specific APIs.
    "os::macos-apis",
    // Bindings to Unix-specific APIs.
    "os::unix-apis",
    // Bindings to Windows-specific APIs.
    "os::windows-apis",
    // Parsers implemented for particular formats or languages.
    "parser-implementations",
    // Crates to help create parsers of binary and text formats.
    "parsing",
    // Real-time or offline rendering of 2D or 3D graphics, usually with the help of a graphics card.
    "rendering",
    // Loading and parsing of data formats related to 2D or 3D rendering.
    "rendering::data-formats",
    // High-level solutions for rendering on the screen.
    "rendering::engine",
    // Crates that provide direct access to hardware or operating system rendering capabilities.
    "rendering::graphics-api",
    // Shared solutions for particular situations specific to programming in Rust.
    "rust-patterns",
    // Crates related to solving problems involving physics, chemistry, biology, geoscience, and other scientific fields.
    "science",
    // Crates for processing large-scale biological data.
    "science::bioinformatics",
    // Crates for processing genetic data, including sequences, abundance, variants, and analysis.
    "science::bioinformatics::genomics",
    // Crates for processing protein data, including sequences, abundance, and analysis.
    "science::bioinformatics::proteomics",
    // Crates for processing biological sequences, including alignment, assembly, and annotation.
    "science::bioinformatics::sequence-analysis",
    // Crates for computational modeling and simulation of biological systems.
    "science::computational-biology",
    // Crates for protein and biomolecular structure prediction, docking, model refinement, and physics-based biomolecular simulation.
    "science::computational-biology::structural-modeling",
    // Crates for network modeling, pathway and metabolic modeling, and whole-system simulations.
    "science::computational-biology::systems-biology",
    // Crates for computational methods in chemistry, including electronic-structure calculations, molecular simulation, and cheminformatics.
    "science::computational-chemistry",
    // Crates for molecular representations, descriptors, chemical graph algorithms, file format parsing, and QSAR tooling.
    "science::computational-chemistry::cheminformatics",
    // Crates for quantum chemistry and electronic-structure methods such as DFT, ab initio, and correlated techniques.
    "science::computational-chemistry::electronic-structure",
    // Crates for molecular dynamics, Monte Carlo, force fields, and statistical mechanics simulations.
    "science::computational-chemistry::molecular-simulation",
    // Processing of spatial information, maps, navigation data, and geographic information systems.
    "science::geo",
    // Crates for the study, characterization, and simulation of condensed matter and materials.
    "science::materials",
    // Crates for research tools and processing of data related to the brain and nervous system.
    "science::neuroscience",
    // Crates for quantum computing, including circuit construction, simulation, quantum algorithms, intermediate representations, and hardware backends.
    "science::quantum-computing",
    // Crates related to robotics.
    "science::robotics",
    // Crates related to cybersecurity, penetration testing, code review, vulnerability research, and reverse engineering.
    "security",
    // Crates used to model or construct models for some activity.
    "simulation",
    // Crates designed to combine templates with data to produce result documents.
    "template-engine",
    // Applications for editing text.
    "text-editors",
    // Crates to deal with the complexities of human language when expressed in textual form.
    "text-processing",
    // Crates to allow an application to format values for display to a user.
    "value-formatting",
    // Crates for the creation and management of virtual environments and resources, including containerization systems.
    "virtualization",
    // Ways to view data, such as plotting or graphing.
    "visualization",
    // Crates for use when targeting WebAssembly, or for manipulating WebAssembly.
    "wasm",
    // Crates to create applications for the web.
    "web-programming",
    // Crates to make HTTP network requests.
    "web-programming::http-client",
    // Crates to serve data over HTTP.
    "web-programming::http-server",
    // Crates to communicate over the WebSocket protocol.
    "web-programming::websocket",
];

pub(crate) fn is_valid_category_slug(slug: &str) -> bool {
    VALID_CATEGORY_SLUGS
        .binary_search_by(|candidate| candidate.cmp(&slug))
        .is_ok()
}
