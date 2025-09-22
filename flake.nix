{
  inputs = {
    nixpkgs.url = "github:joelkoen/nixpkgs";
  };

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      picolimbo = pkgs.rustPlatform.buildRustPackage {
        pname = "picolimbo";
        version = "v1.5.2+mc1.21.8";

        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
    in
    {
      packages.${system} = {
        default = picolimbo; inherit picolimbo;
      };
    };
}
