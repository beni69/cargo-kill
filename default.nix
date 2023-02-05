{ lib
, rustPlatform
, fetchFromGitHub
}:

rustPlatform.buildRustPackage rec {
  pname = "cargo-kill";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "beni69";
    repo = pname;
    rev = "80fb70f19f77469544a66cc0d1c98040495fc16a";
    hash = "sha256-Apz31058sZmFUXSaAyTpfe7Sl3u3c6v6dMIZuwleDjQ=";
  };

  cargoHash = "sha256-xeHg1kgC08eM8dDRwg+IJYs3qlIezsLNQj1kgZVVaws=";

  meta = with lib; {
    description = "find and remove `target` directories and save some disk space";
    homepage = "https://github.com/beni69/cargo-kill";
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ ];
  };
}
