This is a project where I play around with and visualize an [Ant Colony Optimization Algorithm](https://en.wikipedia.org/wiki/Ant_colony_optimization_algorithms) **ACO**. 

A working demo should be deployed at [asks.no](https://asks.no/external/code/ants/index.html). Also check out [my website](https://asks.no/) btw.

## aco
In short, real world ants find a short path between food and nest by utilizing pheromones. This is a type of emergent behavioral; apparent intelligence from un-intelligent actors.

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
