# snet

network simulator based on score and written in rust

## Organization

The core directories are of the network stack. The stack is composed of layers and is based on the [Internet protocol suite](https://en.wikipedia.org/wiki/Internet_protocol_suite) instead of the [OSI model](https://en.wikipedia.org/wiki/OSI_model):

* The *physical* layer is repsonsible for handing the transmission of bits on a wire or over the air, e.g. simulating antennae and radio wave propagation.
* The *link* layer is responsible for data that travels a single hop, e.g. ARP, MAC, tunnels.
* The *internet* layer is responsible for routing frames towards an endpoint, e.g. IP and IGMP.
* The *transport* layer is responsible for endpoint to endpoint communication, e.g. TCP and UDP.
* The *application* layer handles process to process communication, e.g. DNS, HTTP, NTP, SSH, RTP, etc.

The other directories are:

* The *common* directory contains types that are used across layers, e.g. IP and MAC address types.
