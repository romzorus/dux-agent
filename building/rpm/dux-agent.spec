Name:       dux-agent
Version:    PLACEHOLDER_FOR_VERSION
Release:    0
Summary:    Automation tool written in Rust - agent version
License:    Apache License, Version 2.0
URL:        https://gitlab.com/dux-tool/dux
Packager:   romzorus <romzorus591@gmail.com>


%description
An automation agent, regularly applying a configuration to localhost. This configuration can be fetched from multiple sources.

%install
mkdir -p %{buildroot}/usr/bin
cp dux-agent %{buildroot}/usr/bin/
mkdir -p %{buildroot}/etc/dux-agent
cp dux-agent.conf %{buildroot}/etc/dux-agent/

%files
/usr/bin/dux-agent
/etc/dux-agent/dux-agent.conf
