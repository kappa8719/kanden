#!/usr/bin/env python3
"""
Packet Differ - Compare two packets.json files and show differences
"""

import json
import sys
from dataclasses import dataclass
from typing import Dict, List, Set, Tuple
from urllib.error import HTTPError, URLError
from urllib.parse import urlparse
from urllib.request import Request, urlopen


@dataclass
class Packet:
    """Represents a network packet"""

    name: str
    phase: str
    side: str
    id: int

    def key(self) -> Tuple[str, str, str]:
        """Unique identifier for a packet (excluding ID)"""
        return (self.name, self.phase, self.side)

    def full_key(self) -> Tuple[str, str, str, int]:
        """Full identifier including ID"""
        return (self.name, self.phase, self.side, self.id)

    def __str__(self) -> str:
        return f"{self.phase}/{self.side}/{self.name} (ID: {self.id})"


class PacketDiffer:
    """Compares two packet definitions and reports differences"""

    def __init__(self, old_packets: List[Dict], new_packets: List[Dict]):
        self.old = self._parse_packets(old_packets)
        self.new = self._parse_packets(new_packets)

    def _parse_packets(self, packets: List[Dict]) -> Dict[Tuple, Packet]:
        """Convert list of packet dicts to keyed dictionary"""
        result = {}
        for p in packets:
            packet = Packet(p["name"], p["phase"], p["side"], p["id"])
            result[packet.key()] = packet
        return result

    def diff(self) -> Dict[str, List[Packet]]:
        """Calculate differences between old and new packets"""
        old_keys = set(self.old.keys())
        new_keys = set(self.new.keys())

        added = []
        removed = []
        changed = []

        # Find added packets
        for key in new_keys - old_keys:
            added.append(self.new[key])

        # Find removed packets
        for key in old_keys - new_keys:
            removed.append(self.old[key])

        # Find changed packets (same name/phase/side but different ID)
        for key in old_keys & new_keys:
            old_packet = self.old[key]
            new_packet = self.new[key]
            if old_packet.id != new_packet.id:
                changed.append((old_packet, new_packet))

        return {
            "added": sorted(added, key=lambda p: (p.phase, p.side, p.id)),
            "removed": sorted(removed, key=lambda p: (p.phase, p.side, p.id)),
            "changed": sorted(changed, key=lambda t: (t[0].phase, t[0].side, t[0].id)),
        }

    def print_report(self):
        """Print a human-readable diff report"""
        diff = self.diff()

        print("=" * 70)
        print("PACKET DIFFERENCE REPORT")
        print("=" * 70)
        print()

        # Summary
        total_changes = len(diff["added"]) + len(diff["removed"]) + len(diff["changed"])
        print(f"Summary:")
        print(f"  Added:   {len(diff['added'])} packets")
        print(f"  Removed: {len(diff['removed'])} packets")
        print(f"  Changed: {len(diff['changed'])} packets")
        print(f"  Total:   {total_changes} changes")
        print()

        if total_changes == 0:
            print("No differences found!")
            return

        # Added packets
        if diff["added"]:
            print("-" * 70)
            print("ADDED PACKETS:")
            print("-" * 70)
            for packet in diff["added"]:
                print(f"  [+] {packet}")
            print()

        # Removed packets
        if diff["removed"]:
            print("-" * 70)
            print("REMOVED PACKETS:")
            print("-" * 70)
            for packet in diff["removed"]:
                print(f"  [-] {packet}")
            print()

        # Changed packets
        if diff["changed"]:
            print("-" * 70)
            print("CHANGED PACKETS (ID changes):")
            print("-" * 70)
            for old, new in diff["changed"]:
                print(f"  [~] {old.phase}/{old.side}/{old.name}")
                print(f"      ID: {old.id} → {new.id}")
            print()


def is_url(path: str) -> bool:
    """Check if a path is a URL"""
    try:
        result = urlparse(path)
        return result.scheme in ("http", "https")
    except Exception:
        return False


def load_packets(source: str) -> List[Dict]:
    """Load packets from a JSON file or URL"""
    try:
        if is_url(source):
            print(f"  Fetching from URL...")
            # Add a User-Agent header to avoid being blocked
            req = Request(source, headers={"User-Agent": "PacketDiffer/1.0"})
            with urlopen(req, timeout=30) as response:
                data = response.read().decode("utf-8")
                return json.loads(data)
        else:
            with open(source, "r") as f:
                return json.load(f)
    except FileNotFoundError:
        print(f"Error: File '{source}' not found")
        sys.exit(1)
    except (URLError, HTTPError) as e:
        print(f"Error: Failed to fetch URL '{source}': {e}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON in '{source}': {e}")
        sys.exit(1)
    except Exception as e:
        print(f"Error loading '{source}': {e}")
        sys.exit(1)


def main():
    """Main entry point"""
    if len(sys.argv) != 3:
        print("Usage: python packet_differ.py <old_packets> <new_packets>")
        print()
        print("Arguments can be local files or HTTP/HTTPS URLs")
        print()
        print("Examples:")
        print("  python packet_differ.py packets_v1.json packets_v2.json")
        print(
            "  python packet_differ.py https://example.com/v1.json https://example.com/v2.json"
        )
        print("  python packet_differ.py packets_v1.json https://example.com/v2.json")
        sys.exit(1)

    old_source = sys.argv[1]
    new_source = sys.argv[2]

    print(f"Loading old packets from: {old_source}")
    old_packets = load_packets(old_source)

    print(f"Loading new packets from: {new_source}")
    new_packets = load_packets(new_source)

    print()

    differ = PacketDiffer(old_packets, new_packets)
    differ.print_report()


if __name__ == "__main__":
    main()
