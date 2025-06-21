#!/bin/bash

mkdir -p /opt/beerpm
mkdir -p /opt/beerpm/Packages
mkdir -p /opt/beerpm/Binaries
mkdir -p /opt/beerpm/Formulaes

touch /opt/beerpm/beer.config.toml

echo "[info]" > /opt/beerpm/info.toml
installed_count=$(ls /opt/beerpm/Formulaes | wc -l)
disk_usage=$(du -sh /opt/beerpm/Packages | awk '{print $1}')
echo "installed_count = $installed_count" >> /opt/beerpm/info.toml
echo "packages_disk_usage = '$disk_usage'" >> /opt/beerpm/info.toml