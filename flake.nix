{
  description = "Benchmark for serde-query";

  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = [
          pkgs.nixpkgs-fmt
          pkgs.nil
          pkgs.rustup

          pkgs.hyperfine
          pkgs.jq
        ];
      };
    };
}
