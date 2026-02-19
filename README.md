This is a project where I play around with and visualize an [Ant Colony Optimization Algorithm](https://en.wikipedia.org/wiki/Ant_colony_optimization_algorithms) **ACO**. 

A working demo should be deployed at [asks.no](https://asks.no/external/code/ants/index.html). Also check out [my website](https://asks.no/) btw.

## aco
In short, real world ants find a short path between food and nest by utilizing pheromones. This is a type of emergent behavior; apparent intelligence from un-intelligent actors.

The same rules can be simplified and replicated in a computer algorithm. 

## run
I develop with [Nix](https://nixos.org/learn/), in that case running is really easy.

### with nix
With flakes enabled, run `nix run` in repo.
- open `http://localhost:8080` in a browser
- that's it

### without nix
My advice would be to look in `./flake.nix` as a reference, specifically inside `packages` from:

```nix
devShell.${system} = pkgs.mkShell {
  packages = with pkgs; [
    #...
  ];
  #...
};
```

Here you will required tools like `cargo`, `nodejs` and `wasm-bindgen`. After installing these do `nu run.nu` to start a development server.

# stack
- OS environment: Nix
  - Rust environment: Cargo
  - Web environment: Npm

**Why did I chose to combine WebAssembly and JavaScript?**
The reason I did this was because I have a passion for Wasm, and believe in it's efforts to make low-level, compiled, cross-platform code. Especially the day we get a [stable Component-Model ABI](https://github.com/WebAssembly/component-model).

To run both on web and native OS, the full stack should have been Rust with available Wasm target. I could have used WGPU for graphics, and Egui for GUI. But because good tools are important for good workflows, I chose to write graphics in Three.js and GUI in html/js.

This leaves the stack with extra complexity, but not the full benefits. I am fine with this, happy to write Rust, and view this as a challenge.
