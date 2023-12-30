{
  description = "Rust nixity";

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
      pkgs = import nixities.nixpkgs {
        inherit system;
        config = {
          allowUnfree = true;
          cudaSupport = true;
        };
      };
    in {
      # The default devShell
      default = devenv.lib.mkShell {
        inherit pkgs;
        inputs.nixpkgs = nixities.nixpkgs;
        modules = [
          ({
            pkgs,
            lib,
            ...
          }: {
            languages.c.enable = false;
            languages.rust = {
              enable = true;
            };
            packages = (with pkgs; [
              gcc11
              gcc11Stdenv
              cudaPackages.cuda_nvcc
              cudatoolkit
              openssl
              pkg-config
            ]);
            pre-commit.hooks = {
              rustfmt.enable = true;
              clippy.enable = true;
            };
          })
        ];
      };
    });
  };
}
