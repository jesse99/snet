// Copyright (C) 2017 Jesse Jones
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software Foundation,
// Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

//! IPv4 and IPv6 protocol numbers.

// From https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml
// It'd be nice to use an enum for these but then we couldn't support custom protocols
// (we can't mix C and Rust style enum values).
pub const HOPOPT: u8 = 0;	// IPv6 Hop-by-Hop Option   Y   [RFC-ietf-6man-rfc2460bis-13]
pub const ICMP: u8 = 1;	// Internet Control Message      [RFC792]
pub const IGMP: u8 = 2;	// Internet Group Management      [RFC1112]
pub const GGP: u8 = 3;	// Gateway-to-Gateway      [RFC823]
pub const IPV4: u8 = 4;	// IPv4 encapsulation      [RFC2003]
pub const ST: u8 = 5;	// Stream      [RFC1190][RFC1819]
pub const TCP: u8 = 6;	// Transmission Control      [RFC793]
pub const CBT: u8 = 7;	// CBT      [Tony_Ballardie]
pub const EGP: u8 = 8;	// Exterior Gateway Protocol      [RFC888][David_Mills]
pub const IGP: u8 = 9;	// any private interior gateway (used by Cisco for their IGRP)      [Internet_Assigned_Numbers_Authority]
pub const BBN_RCC_MON: u8 = 10;	// BBN RCC Monitoring      [Steve_Chipman]
pub const NVP_II: u8 = 11;	// Network Voice Protocol      [RFC741][Steve_Casner]
pub const PUP: u8 = 12;	// PUP      [Boggs, D., J. Shoch, E. Taft, and R. Metcalfe, "PUP: An Internetwork Architecture", XEROX Palo Alto Research Center, CSL-79-10, July 1979; also in IEEE Transactions on Communication, Volume COM-28, Number 4, April 1980.][[XEROX]]
pub const ARGUS: u8 = 13;	// (deprecated)   ARGUS      [Robert_W_Scheifler]
pub const EMCON: u8 = 14;	// EMCON      [<mystery contact>]
pub const XNET: u8 = 15;	// Cross Net Debugger      [Haverty, J., "XNET Formats for Internet Protocol Version 4", IEN 158, October 1980.][Jack_Haverty]
pub const CHAOS: u8 = 16;	// Chaos      [J_Noel_Chiappa]
pub const UDP: u8 = 17;	// User Datagram      [RFC768][Jon_Postel]
pub const MUX: u8 = 18;	// Multiplexing      [Cohen, D. and J. Postel, "Multiplexing Protocol", IEN 90, USC/Information Sciences Institute, May 1979.][Jon_Postel]
pub const DCN_MEAS: u8 = 19;	// DCN Measurement Subsystems      [David_Mills]
pub const HMP: u8 = 20;	// Host Monitoring      [RFC869][Bob_Hinden]
pub const PRM: u8 = 21;	// Packet Radio Measurement      [Zaw_Sing_Su]
pub const XNS_IDP: u8 = 22;	// XEROX NS IDP      ["The Ethernet, A Local Area Network: Data Link Layer and Physical Layer Specification", AA-K759B-TK, Digital Equipment Corporation, Maynard, MA. Also as: "The Ethernet - A Local Area Network", Version 1.0, Digital Equipment Corporation, Intel Corporation, Xerox Corporation, September 1980. And: "The Ethernet, A Local Area Network: Data Link Layer and Physical Layer Specifications", Digital, Intel and Xerox, November 1982. And: XEROX, "The Ethernet, A Local Area Network: Data Link Layer and Physical Layer Specification", X3T51/80-50, Xerox Corporation, Stamford, CT., October 1980.][[XEROX]]
pub const TRUNK_1: u8 = 23;	// Trunk-1      [Barry_Boehm]
pub const TRUNK_2: u8 = 24;	// Trunk-2      [Barry_Boehm]
pub const LEAF_1: u8 = 25;	// Leaf-1      [Barry_Boehm]
pub const LEAF_2: u8 = 26;	// Leaf-2      [Barry_Boehm]
pub const RDP: u8 = 27;	// Reliable Data Protocol      [RFC908][Bob_Hinden]
pub const IRTP: u8 = 28;	// Internet Reliable Transaction      [RFC938][Trudy_Miller]
pub const ISO_TP4: u8 = 29;	// ISO Transport Protocol Class 4      [RFC905][<mystery contact>]
pub const NETBLT: u8 = 30;	// Bulk Data Transfer Protocol      [RFC969][David_Clark]
pub const MFE_NSP: u8 = 31;	// MFE Network Services Protocol      [Shuttleworth, B., "A Documentary of MFENet, a National Computer Network", UCRL-52317, Lawrence Livermore Labs, Livermore, California, June 1977.][Barry_Howard]
pub const MERIT_INP: u8 = 32;	// MERIT Internodal Protocol      [Hans_Werner_Braun]
pub const DCCP: u8 = 33;	// Datagram Congestion Control Protocol      [RFC4340]
pub const THREE_PC: u8 = 34;	// Third Party Connect Protocol      [Stuart_A_Friedberg]
pub const IDPR: u8 = 35;	// Inter-Domain Policy Routing Protocol      [Martha_Steenstrup]
pub const XTP: u8 = 36;	// XTP      [Greg_Chesson]
pub const DDP: u8 = 37;	// Datagram Delivery Protocol      [Wesley_Craig]
pub const IDPR_CMTP: u8 = 38;	// IDPR Control Message Transport Proto      [Martha_Steenstrup]
pub const TPPP: u8 = 39;      // TP++ Transport Protocol      [Dirk_Fromhein]
pub const IL: u8 = 40;	// IL Transport Protocol      [Dave_Presotto]
pub const IPV6: u8 = 41;	// IPv6 encapsulation      [RFC2473]
pub const SDRP: u8 = 42;	// Source Demand Routing Protocol      [Deborah_Estrin]
pub const IPV6_ROUTE: u8 = 43;	// Routing Header for IPv6   Y   [Steve_Deering]
pub const IPV6_FRAG: u8 = 44;	// Fragment Header for IPv6   Y   [Steve_Deering]
pub const IDRP: u8 = 45;	// Inter-Domain Routing Protocol      [Sue_Hares]
pub const RSVP: u8 = 46;	// Reservation Protocol      [RFC2205][RFC3209][Bob_Braden]
pub const GRE: u8 = 47;	// Generic Routing Encapsulation      [RFC2784][Tony_Li]
pub const DSR: u8 = 48;	// Dynamic Source Routing Protocol      [RFC4728]
pub const BNA: u8 = 49;	// BNA      [Gary Salamon]
pub const ESP: u8 = 50;	// Encap Security Payload   Y   [RFC4303]
pub const AH: u8 = 51;	// Authentication Header   Y   [RFC4302]
pub const I_NLSP: u8 = 52;	// Integrated Net Layer Security TUBA      [K_Robert_Glenn]
pub const SWIPE: u8 = 53;	// (deprecated)   IP with Encryption      [John_Ioannidis]
pub const NARP: u8 = 54;	// NBMA Address Resolution Protocol      [RFC1735]
pub const MOBILE: u8 = 55;	// IP Mobility      [Charlie_Perkins]
pub const TLSP: u8 = 56;	// Transport Layer Security Protocol using Kryptonet key management      [Christer_Oberg]
pub const SKIP: u8 = 57;	// SKIP      [Tom_Markson]
pub const IPV6_ICMP: u8 = 58;	// ICMP for IPv6      [RFC-ietf-6man-rfc2460bis-13]
pub const IPV6_NO_NXT: u8 = 59;	// No Next Header for IPv6      [RFC-ietf-6man-rfc2460bis-13]
pub const IPV6_OPTS: u8 = 60;	// Destination Options for IPv6   Y   [RFC-ietf-6man-rfc2460bis-13]
pub const HOST_INTERNAL: u8 = 61;	// any host internal protocol      [Internet_Assigned_Numbers_Authority]
pub const CFTP: u8 = 62;	// CFTP      [Forsdick, H., "CFTP", Network Message, Bolt Beranek and Newman, January 1982.][Harry_Forsdick]
pub const LOCAL_NETWORK: u8 = 63;	// any local network      [Internet_Assigned_Numbers_Authority]
pub const SAT_EXPAK: u8 = 64;	// SATNET and Backroom EXPAK      [Steven_Blumenthal]
pub const KRYPTOLAN: u8 = 65;	// Kryptolan      [Paul Liu]
pub const RVD: u8 = 66;	// MIT Remote Virtual Disk Protocol      [Michael_Greenwald]
pub const IPPC: u8 = 67;	// Internet Pluribus Packet Core      [Steven_Blumenthal]
pub const ANY_DISTRIBUTED_FS: u8 = 68;	// any distributed file system      [Internet_Assigned_Numbers_Authority]
pub const SAT_MON: u8 = 69;	// SATNET Monitoring      [Steven_Blumenthal]
pub const VISA: u8 = 70;	// VISA Protocol      [Gene_Tsudik]
pub const IPCV: u8 = 71;	// Internet Packet Core Utility      [Steven_Blumenthal]
pub const CPNX: u8 = 72;	// Computer Protocol Network Executive      [David Mittnacht]
pub const CPHB: u8 = 73;	// Computer Protocol Heart Beat      [David Mittnacht]
pub const WSN: u8 = 74;	// Wang Span Network      [Victor Dafoulas]
pub const PVP: u8 = 75;	// Packet Video Protocol      [Steve_Casner]
pub const BR_SAT_MON: u8 = 76;	// Backroom SATNET Monitoring      [Steven_Blumenthal]
pub const SUN_ND: u8 = 77;	// SUN ND PROTOCOL-Temporary      [William_Melohn]
pub const WB_MON: u8 = 78;	// WIDEBAND Monitoring      [Steven_Blumenthal]
pub const WB_EXPAK: u8 = 79;	// WIDEBAND EXPAK      [Steven_Blumenthal]
pub const ISO_IP: u8 = 80;	// ISO Internet Protocol      [Marshall_T_Rose]
pub const VMTP: u8 = 81;	// VMTP      [Dave_Cheriton]
pub const SECURE_VMTP: u8 = 82;	// SECURE-VMTP      [Dave_Cheriton]
pub const VINES: u8 = 83;	// VINES      [Brian Horn]
pub const TTP: u8 = 84;	// Transaction Transport Protocol      [Jim_Stevens]
pub const IPTM: u8 = 84;	// Internet Protocol Traffic Manager      [Jim_Stevens]
pub const NSFNET_IGP: u8 = 85;	// NSFNET-IGP      [Hans_Werner_Braun]
pub const DGP: u8 = 86;	// Dissimilar Gateway Protocol      [M/A-COM Government Systems, "Dissimilar Gateway Protocol Specification, Draft Version", Contract no. CS901145, November 16, 1987.][Mike_Little]
pub const TCF: u8 = 87;	// TCF      [Guillermo_A_Loyola]
pub const EIGRP: u8 = 88;	// EIGRP      [RFC7868]
pub const OSPFIGP: u8 = 89;	// OSPFIGP      [RFC1583][RFC2328][RFC5340][John_Moy]
pub const SPRITE_RPC: u8 = 90;	// Sprite RPC Protocol      [Welch, B., "The Sprite Remote Procedure Call System", Technical Report, UCB/Computer Science Dept., 86/302, University of California at Berkeley, June 1986.][Bruce Willins]
pub const LARP: u8 = 91;	// Locus Address Resolution Protocol      [Brian Horn]
pub const MTP: u8 = 92;	// Multicast Transport Protocol      [Susie_Armstrong]
pub const AX25: u8 = 93;    //  AX.25 Frames [Brian_Kantor]
pub const IPIP: u8 = 94;	// IP-within-IP Encapsulation Protocol      [John_Ioannidis]
pub const MICP: u8 = 95;	// (deprecated)   Mobile Internetworking Control Pro.      [John_Ioannidis]
pub const SCC_SP: u8 = 96;	// Semaphore Communications Sec. Pro.      [Howard_Hart]
pub const ETHERIP: u8 = 97;	// Ethernet-within-IP Encapsulation      [RFC3378]
pub const ENCAP: u8 = 98;	// Encapsulation Header      [RFC1241][Robert_Woodburn]
pub const PRIVATE_ENCRYPTION: u8 = 99;	// any private encryption scheme      [Internet_Assigned_Numbers_Authority]
pub const GMTP: u8 = 100;	// GMTP      [[RXB5]]
pub const IFMP: u8 = 101;	// Ipsilon Flow Management Protocol      [Bob_Hinden][November 1995, 1997.]
pub const PNNI: u8 = 102;	// PNNI over IP      [Ross_Callon]
pub const PIM: u8 = 103;	// Protocol Independent Multicast      [RFC7761][Dino_Farinacci]
pub const ARIS: u8 = 104;	// ARIS      [Nancy_Feldman]
pub const SCPS: u8 = 105;	// SCPS      [Robert_Durst]
pub const QNX: u8 = 106;	// QNX      [Michael_Hunter]
pub const AN: u8 = 107;     // Active Networks      [Bob_Braden]
pub const IP_COMP: u8 = 108;	// IP Payload Compression Protocol      [RFC2393]
pub const SNP: u8 = 109;	// Sitara Networks Protocol      [Manickam_R_Sridhar]
pub const COMPAQ_PEER: u8 = 110;	// Compaq Peer Protocol      [Victor_Volpe]
pub const IPX_IN_IP: u8 = 111;	// IPX in IP      [CJ_Lee]
pub const VRRP: u8 = 112;	// Virtual Router Redundancy Protocol      [RFC5798]
pub const PGM: u8 = 113;	// PGM Reliable Transport Protocol      [Tony_Speakman]
pub const ZERO_HOP: u8 = 114;	// any 0-hop protocol      [Internet_Assigned_Numbers_Authority]
pub const L2TP: u8 = 115;	// Layer Two Tunneling Protocol      [RFC3931][Bernard_Aboba]
pub const DDX: u8 = 116;	// D-II Data Exchange (DDX)      [John_Worley]
pub const IATP: u8 = 117;	// Interactive Agent Transfer Protocol      [John_Murphy]
pub const STP: u8 = 118;	// Schedule Transfer Protocol      [Jean_Michel_Pittet]
pub const SRP: u8 = 119;	// SpectraLink Radio Protocol      [Mark_Hamilton]
pub const UTI: u8 = 120;	// UTI      [Peter_Lothberg]
pub const SMP: u8 = 121;	// Simple Message Protocol      [Leif_Ekblad]
pub const SM: u8 = 122;	// (deprecated)   Simple Multicast Protocol      [Jon_Crowcroft][draft-perlman-simple-multicast]
pub const PTP: u8 = 123;	// Performance Transparency Protocol      [Michael_Welzl]
pub const ISIS: u8 = 124;	// over IPv4         [Tony_Przygienda]
pub const FIRE: u8 = 125;	// [Criag_Partridge]
pub const CRTP: u8 = 126;	// Combat Radio Transport Protocol      [Robert_Sautter]
pub const CRUDP: u8 = 127;	// Combat Radio User Datagram      [Robert_Sautter]
pub const SSCOPMCE: u8 = 128;	// [Kurt_Waber]
pub const IPLT: u8 = 129;	// [[Hollbach]]
pub const SPS: u8 = 130;	// Secure Packet Shield      [Bill_McIntosh]
pub const PIPE: u8 = 131;	// Private IP Encapsulation within IP      [Bernhard_Petri]
pub const SCTP: u8 = 132;	// Stream Control Transmission Protocol      [Randall_R_Stewart]
pub const FC: u8 = 133;	// Fibre Channel      [Murali_Rajagopal][RFC6172]
pub const RSVP_E2E_IGNORE: u8 = 134;	// [RFC3175]
pub const MOBILITY: u8 = 135;	// Mobility Header      Y   [RFC6275]
pub const UDP_LITE: u8 = 136;	// [RFC3828]
pub const MPLS_IN_IP: u8 = 137;	// [RFC4023]
pub const MANET: u8 = 138;	// MANET Protocols      [RFC5498]
pub const HIP: u8 = 139;	// Host Identity Protocol   Y   [RFC7401]
pub const SHIM6: u8 = 140;	// Shim6 Protocol   Y   [RFC5533]
pub const WESP: u8 = 141;	// Wrapped Encapsulating Security Payload      [RFC5840]
pub const ROHC: u8 = 142;	// Robust Header Compression      [RFC5858]
// 143-252 are unassigned
pub const EXPERIMENTAL1: u8 = 253;	// use for experimentation and testing   Y   [RFC3692]
pub const EXPERIMENTAL2: u8 = 254;	// use for experimentation and testing   Y   [RFC3692]
pub const RESERVED: u8 = 255;	// Reserved [Internet_Assigned_Numbers_Authority] People
