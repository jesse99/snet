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

//! IPv4 and IPv6 ethernet type numbers.

// From https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml#ieee-802-numbers-1
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum EtherType
{
	EthernetLength(u16),	// wired ethernet length field (802.3)
	// There's also an Experimental ether_type but that seems to have gone away as of IEEE 802.3x-1997.
	IPv4,
	ARP,
	RARP,
	SNMP,
	IPv6,
	LocalExperimental0,
	LocalExperimental1,
	Reserved,
	// TODO: lot's more, notably:
	// 34958,888E,-,-,IEEE Std 802.1X - Port-based network access control,[IEEE]
	// 34984,88A8,-,-,IEEE Std 802.1Q - Service VLAN tag identifier (S-Tag),[IEEE]
	// 35020,88CC,-,-,IEEE Std 802.1AB - Link Layer Discovery Protocol (LLDP),[IEEE]
	// 35061,88F5,-,-,IEEE Std 802.1Q  - Multiple VLAN Registration Protocol (MVRP),[IEEE]
	// 35062,88F6,-,-,IEEE Std 802.1Q - Multiple Multicast Registration Protocol (MMRP),[IEEE]
	// 35085,890D,-,-,IEEE Std 802.11 - Fast Roaming Remote Request (802.11r),[IEEE]
}

impl EtherType 
{
    pub fn from_u16(value: u16) -> Self
    {
		match value {
			0x0000...0x05DC => EtherType::EthernetLength(value),
			0x0800 => EtherType::IPv4,
			0x0806 => EtherType::ARP,
			0x8035 => EtherType::RARP,
			0x814C => EtherType::SNMP,
			0x86DD => EtherType::IPv6,
			0x88B5 => EtherType::LocalExperimental0,
			0x88B6 => EtherType::LocalExperimental1,
			0xFFFF => EtherType::Reserved,
			_ => panic!("{} isn't a supported ether_type", value),
		}
    }

    pub fn as_u16(self) -> u16
    {
		match self {
			EtherType::EthernetLength(v) => v,
			EtherType::IPv4 => 0x0800,
			EtherType::ARP => 0x0806,
			EtherType::RARP => 0x8035,
			EtherType::SNMP => 0x814C,
			EtherType::IPv6 => 0x86DD,
			EtherType::LocalExperimental0 => 0x88B5,
			EtherType::LocalExperimental1 => 0x88B6,
			EtherType::Reserved => 0xFFFF,
		}
    }

    pub fn is_valid(self) -> bool
    {
		match self {
			EtherType::EthernetLength(v) => v <= 0x05DC,
			EtherType::Reserved => false,
			_ => true,
		}
    }
} 
