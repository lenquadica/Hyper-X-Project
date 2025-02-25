{ pkgs }: {
  deps = [
    pkgs.gh
    pkgs.openssh
    pkgs.docker-compose_1
    pkgs.docker_26
    pkgs.cowsay
  ];
}