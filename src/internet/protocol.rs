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
use std::mem;

// From https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum StandardProtocol
{
	HOPOPT = 0,			// IPv6 Hop-by-Hop Option   Y   [RFC-ietf-6man-rfc2460bis-13]
	ICMP = 1,			// Internet Control Message      [RFC792]
	IGMP = 2,			// Internet Group Management      [RFC1112]
	GGP = 3,			// Gateway-to-Gateway      [RFC823]
	IPV4 = 4,			// IPv4 encapsulation      [RFC2003]
	ST = 5,				// Stream      [RFC1190][RFC1819]
	TCP = 6,			// Transmission Control      [RFC793]
	CBT = 7,			// CBT      [Tony_Ballardie]
	EGP = 8,			// Exterior Gateway Protocol      [RFC888][David_Mills]
	IGP = 9,			// any private interior gateway (used by Cisco for their IGRP)      [Internet_Assigned_Numbers_Authority]
	BBN_RCC_MON = 10,	// BBN RCC Monitoring      [Steve_Chipman]
	NVP_II = 11,		// Network Voice Protocol      [RFC741][Steve_Casner]
	PUP = 12,			// PUP      [Boggs, D., J. Shoch, E. Taft, and R. Metcalfe, "PUP: An Internetwork Architecture", XEROX Palo Alto Research Center, CSL-79-10, July 1979; also in IEEE Transactions on Communication, Volume COM-28, Number 4, April 1980.][[XEROX]]
	ARGUS = 13,			// (deprecated)   ARGUS      [Robert_W_Scheifler]
	EMCON = 14,			// EMCON      [<mystery contact>]
	XNET = 15,			// Cross Net Debugger      [Haverty, J., "XNET Formats for Internet Protocol Version 4", IEN 158, October 1980.][Jack_Haverty]
	CHAOS = 16,			// Chaos      [J_Noel_Chiappa]
	UDP = 17,			// User Datagram      [RFC768][Jon_Postel]
	MUX = 18,			// Multiplexing      [Cohen, D. and J. Postel, "Multiplexing Protocol", IEN 90, USC/Information Sciences Institute, May 1979.][Jon_Postel]
	DCN_MEAS = 19,		// DCN Measurement Subsystems      [David_Mills]
	HMP = 20,			// Host Monitoring      [RFC869][Bob_Hinden]
	PRM = 21,			// Packet Radio Measurement      [Zaw_Sing_Su]
	XNS_IDP = 22,		// XEROX NS IDP      ["The Ethernet, A Local Area Network: Data Link Layer and Physical Layer Specification", AA-K759B-TK, Digital Equipment Corporation, Maynard, MA. Also as: "The Ethernet - A Local Area Network", Version 1.0, Digital Equipment Corporation, Intel Corporation, Xerox Corporation, September 1980. And: "The Ethernet, A Local Area Network: Data Link Layer and Physical Layer Specifications", Digital, Intel and Xerox, November 1982. And: XEROX, "The Ethernet, A Local Area Network: Data Link Layer and Physical Layer Specification", X3T51/80-50, Xerox Corporation, Stamford, CT., October 1980.][[XEROX]]
	TRUNK_1 = 23,		// Trunk-1      [Barry_Boehm]
	TRUNK_2 = 24,		// Trunk-2      [Barry_Boehm]
	LEAF_1 = 25,		// Leaf-1      [Barry_Boehm]
	LEAF_2 = 26,		// Leaf-2      [Barry_Boehm]
	RDP = 27,			// Reliable Data Protocol      [RFC908][Bob_Hinden]
	IRTP = 28,			// Internet Reliable Transaction      [RFC938][Trudy_Miller]
	ISO_TP4 = 29,		// ISO Transport Protocol Class 4      [RFC905][<mystery contact>]
	NETBLT = 30,		// Bulk Data Transfer Protocol      [RFC969][David_Clark]
	MFE_NSP = 31,		// MFE Network Services Protocol      [Shuttleworth, B., "A Documentary of MFENet, a National Computer Network", UCRL-52317, Lawrence Livermore Labs, Livermore, California, June 1977.][Barry_Howard]
	MERIT_INP = 32,		// MERIT Internodal Protocol      [Hans_Werner_Braun]
	DCCP = 33,			// Datagram Congestion Control Protocol      [RFC4340]
	THREE_PC = 34,		// Third Party Connect Protocol      [Stuart_A_Friedberg]
	IDPR = 35,			// Inter-Domain Policy Routing Protocol      [Martha_Steenstrup]
	XTP = 36,			// XTP      [Greg_Chesson]
	DDP = 37,			// Datagram Delivery Protocol      [Wesley_Craig]
	IDPR_CMTP = 38,		// IDPR Control Message Transport Proto      [Martha_Steenstrup]
	TPPP = 39,			// TP++ Transport Protocol      [Dirk_Fromhein]
	IL = 40,			// IL Transport Protocol      [Dave_Presotto]
	IPV6 = 41,			// IPv6 encapsulation      [RFC2473]
	SDRP = 42,			// Source Demand Routing Protocol      [Deborah_Estrin]
	IPV6_ROUTE = 43,	// Routing Header for IPv6   Y   [Steve_Deering]
	IPV6_FRAG = 44,		// Fragment Header for IPv6   Y   [Steve_Deering]
	IDRP = 45,			// Inter-Domain Routing Protocol      [Sue_Hares]
	RSVP = 46,			// Reservation Protocol      [RFC2205][RFC3209][Bob_Braden]
	GRE = 47,			// Generic Routing Encapsulation      [RFC2784][Tony_Li]
	DSR = 48,			// Dynamic Source Routing Protocol      [RFC4728]
	BNA = 49,			// BNA      [Gary Salamon]
	ESP = 50,			// Encap Security Payload   Y   [RFC4303]
	AH = 51,			// Authentication Header   Y   [RFC4302]
	I_NLSP = 52,		// Integrated Net Layer Security TUBA      [K_Robert_Glenn]
	SWIPE = 53,			// (deprecated)   IP with Encryption      [John_Ioannidis]
	NARP = 54,			// NBMA Address Resolution Protocol      [RFC1735]
	MOBILE = 55,		// IP Mobility      [Charlie_Perkins]
	TLSP = 56,			// Transport Layer Security Protocol using Kryptonet key management      [Christer_Oberg]
	SKIP = 57,			// SKIP      [Tom_Markson]
	IPV6_ICMP = 58,		// ICMP for IPv6      [RFC-ietf-6man-rfc2460bis-13]
	IPV6_NO_NXT = 59,	// No Next Header for IPv6      [RFC-ietf-6man-rfc2460bis-13]
	IPV6_OPTS = 60,		// Destination Options for IPv6   Y   [RFC-ietf-6man-rfc2460bis-13]
	HOST_INTERNAL = 61,	// any host internal protocol      [Internet_Assigned_Numbers_Authority]
	CFTP = 62,			// CFTP      [Forsdick, H., "CFTP", Network Message, Bolt Beranek and Newman, January 1982.][Harry_Forsdick]
	LOCAL_NETWORK = 63,	// any local network      [Internet_Assigned_Numbers_Authority]
	SAT_EXPAK = 64,		// SATNET and Backroom EXPAK      [Steven_Blumenthal]
	KRYPTOLAN = 65,		// Kryptolan      [Paul Liu]
	RVD = 66,			// MIT Remote Virtual Disk Protocol      [Michael_Greenwald]
	IPPC = 67,			// Internet Pluribus Packet Core      [Steven_Blumenthal]
	ANY_DISTRIBUTED_FS = 68,	// any distributed file system      [Internet_Assigned_Numbers_Authority]
	SAT_MON = 69,		// SATNET Monitoring      [Steven_Blumenthal]
	VISA = 70,			// VISA Protocol      [Gene_Tsudik]
	IPCV = 71,			// Internet Packet Core Utility      [Steven_Blumenthal]
	CPNX = 72,			// Computer Protocol Network Executive      [David Mittnacht]
	CPHB = 73,			// Computer Protocol Heart Beat      [David Mittnacht]
	WSN = 74,			// Wang Span Network      [Victor Dafoulas]
	PVP = 75,			// Packet Video Protocol      [Steve_Casner]
	BR_SAT_MON = 76,	// Backroom SATNET Monitoring      [Steven_Blumenthal]
	SUN_ND = 77,		// SUN ND PROTOCOL-Temporary      [William_Melohn]
	WB_MON = 78,		// WIDEBAND Monitoring      [Steven_Blumenthal]
	WB_EXPAK = 79,		// WIDEBAND EXPAK      [Steven_Blumenthal]
	ISO_IP = 80,		// ISO Internet Protocol      [Marshall_T_Rose]
	VMTP = 81,			// VMTP      [Dave_Cheriton]
	SECURE_VMTP = 82,	// SECURE-VMTP      [Dave_Cheriton]
	VINES = 83,			// VINES      [Brian Horn]
	TTP = 84,			// Transaction Transport Protocol      [Jim_Stevens]
	//IPTM = 84,			// Internet Protocol Traffic Manager      [Jim_Stevens]	TODO: not sure why there are two 84's
	NSFNET_IGP = 85,	// NSFNET-IGP      [Hans_Werner_Braun]
	DGP = 86,			// Dissimilar Gateway Protocol      [M/A-COM Government Systems, "Dissimilar Gateway Protocol Specification, Draft Version", Contract no. CS901145, November 16, 1987.][Mike_Little]
	TCF = 87,			// TCF      [Guillermo_A_Loyola]
	EIGRP = 88,			// EIGRP      [RFC7868]
	OSPFIGP = 89,		// OSPFIGP      [RFC1583][RFC2328][RFC5340][John_Moy]
	SPRITE_RPC = 90,	// Sprite RPC Protocol      [Welch, B., "The Sprite Remote Procedure Call System", Technical Report, UCB/Computer Science Dept., 86/302, University of California at Berkeley, June 1986.][Bruce Willins]
	LARP = 91,			// Locus Address Resolution Protocol      [Brian Horn]
	MTP = 92,			// Multicast Transport Protocol      [Susie_Armstrong]
	AX25 = 93,			//  AX.25 Frames [Brian_Kantor]
	IPIP = 94,			// IP-within-IP Encapsulation Protocol      [John_Ioannidis]
	MICP = 95,			// (deprecated)   Mobile Internetworking Control Pro.      [John_Ioannidis]
	SCC_SP = 96,		// Semaphore Communications Sec. Pro.      [Howard_Hart]
	ETHERIP = 97,		// Ethernet-within-IP Encapsulation      [RFC3378]
	ENCAP = 98,			// Encapsulation Header      [RFC1241][Robert_Woodburn]
	PRIVATE_ENCRYPTION = 99,	// any private encryption scheme      [Internet_Assigned_Numbers_Authority]
	GMTP = 100,			// GMTP      [[RXB5]]
	IFMP = 101,			// Ipsilon Flow Management Protocol      [Bob_Hinden][November 1995, 1997.]
	PNNI = 102,			// PNNI over IP      [Ross_Callon]
	PIM = 103,			// Protocol Independent Multicast      [RFC7761][Dino_Farinacci]
	ARIS = 104,			// ARIS      [Nancy_Feldman]
	SCPS = 105,			// SCPS      [Robert_Durst]
	QNX = 106,			// QNX      [Michael_Hunter]
	AN = 107,			// Active Networks      [Bob_Braden]
	IP_COMP = 108,		// IP Payload Compression Protocol      [RFC2393]
	SNP = 109,			// Sitara Networks Protocol      [Manickam_R_Sridhar]
	COMPAQ_PEER = 110,	// Compaq Peer Protocol      [Victor_Volpe]
	IPX_IN_IP = 111,	// IPX in IP      [CJ_Lee]
	VRRP = 112,			// Virtual Router Redundancy Protocol      [RFC5798]
	PGM = 113,			// PGM Reliable Transport Protocol      [Tony_Speakman]
	ZERO_HOP = 114,		// any 0-hop protocol      [Internet_Assigned_Numbers_Authority]
	L2TP = 115,			// Layer Two Tunneling Protocol      [RFC3931][Bernard_Aboba]
	DDX = 116,			// D-II Data Exchange (DDX)      [John_Worley]
	IATP = 117,			// Interactive Agent Transfer Protocol      [John_Murphy]
	STP = 118,			// Schedule Transfer Protocol      [Jean_Michel_Pittet]
	SRP = 119,			// SpectraLink Radio Protocol      [Mark_Hamilton]
	UTI = 120,			// UTI      [Peter_Lothberg]
	SMP = 121,			// Simple Message Protocol      [Leif_Ekblad]
	SM = 122,			// (deprecated)   Simple Multicast Protocol      [Jon_Crowcroft][draft-perlman-simple-multicast]
	PTP = 123,			// Performance Transparency Protocol      [Michael_Welzl]
	ISIS = 124,			// over IPv4         [Tony_Przygienda]
	FIRE = 125,			// [Criag_Partridge]
	CRTP = 126,			// Combat Radio Transport Protocol      [Robert_Sautter]
	CRUDP = 127,		// Combat Radio User Datagram      [Robert_Sautter]
	SSCOPMCE = 128,		// [Kurt_Waber]
	IPLT = 129,			// [[Hollbach]]
	SPS = 130,			// Secure Packet Shield      [Bill_McIntosh]
	PIPE = 131,			// Private IP Encapsulation within IP      [Bernhard_Petri]
	SCTP = 132,			// Stream Control Transmission Protocol      [Randall_R_Stewart]
	FC = 133,			// Fibre Channel      [Murali_Rajagopal][RFC6172]
	RSVP_E2E_IGNORE = 134,	// [RFC3175]
	MOBILITY = 135,		// Mobility Header      Y   [RFC6275]
	UDP_LITE = 136,		// [RFC3828]
	MPLS_IN_IP = 137,	// [RFC4023]
	MANET = 138,		// MANET Protocols      [RFC5498]
	HIP = 139,			// Host Identity Protocol   Y   [RFC7401]
	SHIM6 = 140,		// Shim6 Protocol   Y   [RFC5533]
	WESP = 141,			// Wrapped Encapsulating Security Payload      [RFC5840]
	ROHC = 142,			// Robust Header Compression      [RFC5858]
	// 143-252 are unassigned
	EXPERIMENTAL1 = 253,// use for experimentation and testing   Y   [RFC3692]
	EXPERIMENTAL2 = 254,// use for experimentation and testing   Y   [RFC3692]
	RESERVED = 255,		// Reserved [Internet_Assigned_Numbers_Authority] People
}

// We have to use multiple enums because both C style and Rust style enums can't be used within one enum.
#[derive(Clone, Copy, Debug)]
pub enum Protocol
{
	/// RESERVED should not be used.
	Standard(StandardProtocol),

	/// Use values in [143, 252].
	Custom(u8),
}

impl Protocol 
{
    pub fn from_u8(value: u8) -> Self
    {
		if value >= 143 && value <= 252 {
			Protocol::Custom(value)
		} else {
			unsafe {Protocol::Standard(mem::transmute(value))}	// TODO: could get rid of unsafe using https://crates.io/crates/enum_primitive
		}
    }

    pub fn as_u8(self) -> u8
    {
		match self {
			Protocol::Standard(s) => s as u8,
			Protocol::Custom(v) => v,
		}
    }

    pub fn is_valid(self) -> bool
    {
		match self {
			Protocol::Standard(s) => (s as u8) != 255,
			Protocol::Custom(v) => v >= 143 && v <= 252,
		}
    }
} 
