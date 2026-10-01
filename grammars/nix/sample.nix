# A sample.
{ pkgs ? import <nixpkgs> { }, lib ? pkgs.lib, enableTests ? true }:

let
  version = "1.2.3";
  limit = 10;
  inherit (lib) optional mkIf;

  area = shape:
    if shape.kind == "circle" then 3.14 * shape.radius * shape.radius
    else if shape ? width then shape.width * shape.height
    else 0;

  shapes = [
    { kind = "circle"; radius = 1; }
    { kind = "rect"; width = 2; height = 3; }
  ];
in
pkgs.stdenv.mkDerivation rec {
  pname = "sample";
  inherit version;

  src = ./.;
  nativeBuildInputs = with pkgs; [ cmake ninja ] ++ optional enableTests pkgs.gtest;

  cmakeFlags = [ "-DLIMIT=${toString limit}" ];

  buildPhase = ''
    echo "building ${pname}-${version}"
    make -j$NIX_BUILD_CORES
  '';

  passthru = {
    areas = map area shapes;
    small = builtins.filter (a: a < limit) (map area shapes);
  };

  meta = {
    description = "A sample";
    license = lib.licenses.mit;
    platforms = lib.platforms.unix;
  };
}
