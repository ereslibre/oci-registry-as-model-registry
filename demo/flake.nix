{
  description = "Nixity";

  inputs = {
    devenv.url = "github:cachix/devenv";
    nixities.url = "github:ereslibre/nixities";
    systems.url = "github:nix-systems/default";
  };

  outputs = {
    self,
    devenv,
    nixities,
    systems,
    ...
  } @ inputs: let
    forEachSystem = nixities.nixpkgs.lib.genAttrs (import systems);
  in {
    devShells = forEachSystem (system: let
      pkgs = import nixities.nixpkgs {inherit system;};
    in {
      # The default devShell
      default = devenv.lib.mkShell {
        inherit pkgs;
        inputs.nixpkgs = nixities.nixpkgs;
        modules = [
          ({pkgs, ...}: {
            languages.go.enable = true;
          })
        ];
      };
    });
  };
}
