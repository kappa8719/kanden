#!/usr/bin/env python3
"""
Converts Fabric Yarn names to Mojang official mapping names.
Downloads and caches yarn, intermediary, and mojang mappings automatically.
"""

import argparse
import json
import os
import sys
import urllib.request
import zipfile
from pathlib import Path


class YarnToMojangConverter:
    def __init__(self, minecraft_version, cache_dir=".mapping_cache"):
        self.mc_version = minecraft_version
        self.cache_dir = Path(sys.path[0] + "/" + cache_dir)
        self.cache_dir.mkdir(exist_ok=True)

        # Mapping dictionaries
        self.yarn_to_intermediary = {}  # yarn_name -> intermediary_name
        self.intermediary_to_obf = {}  # intermediary_name -> obfuscated_name
        self.obf_to_mojang = {}  # obfuscated_name -> mojang_name

        # For fields and methods: (class, name, descriptor) -> name
        self.yarn_fields = {}
        self.yarn_methods = {}
        self.intermediary_fields = {}
        self.intermediary_methods = {}
        self.mojang_fields = {}
        self.mojang_methods = {}

    def download_file(self, url, dest):
        """Download a file if it doesn't exist."""
        if dest.exists():
            print(f"Using cached: {dest}")
            return
        print(f"Downloading: {url}")
        urllib.request.urlretrieve(url, dest)

    def download_mappings(self):
        """Download all required mapping files."""
        # Download Yarn mappings
        yarn_url = (
            f"https://github.com/FabricMC/yarn/archive/refs/heads/{self.mc_version}.zip"
        )
        yarn_zip = self.cache_dir / f"yarn-{self.mc_version}.zip"
        yarn_dir = self.cache_dir / f"yarn-{self.mc_version}"

        if not yarn_dir.exists():
            self.download_file(yarn_url, yarn_zip)
            print("Extracting Yarn...")
            with zipfile.ZipFile(yarn_zip, "r") as zip_ref:
                zip_ref.extractall(self.cache_dir)

        # Download Intermediary mappings
        intermediary_url = f"https://github.com/FabricMC/intermediary/raw/master/mappings/{self.mc_version}.tiny"
        intermediary_file = self.cache_dir / f"intermediary-{self.mc_version}.tiny"
        self.download_file(intermediary_url, intermediary_file)

        # Download Mojang mappings
        version_manifest_url = (
            "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"
        )
        manifest_file = self.cache_dir / "version_manifest.json"
        self.download_file(version_manifest_url, manifest_file)

        with open(manifest_file) as f:
            manifest = json.load(f)

        version_info = None
        for v in manifest["versions"]:
            if v["id"] == self.mc_version:
                version_info = v
                break

        if not version_info:
            raise ValueError(f"Version {self.mc_version} not found in manifest")

        version_json_file = self.cache_dir / f"{self.mc_version}.json"
        self.download_file(version_info["url"], version_json_file)

        with open(version_json_file) as f:
            version_data = json.load(f)

        if (
            "downloads" in version_data
            and "client_mappings" in version_data["downloads"]
        ):
            mojang_url = version_data["downloads"]["client_mappings"]["url"]
            mojang_file = self.cache_dir / f"mojang-{self.mc_version}.txt"
            self.download_file(mojang_url, mojang_file)
        else:
            raise ValueError(f"No client mappings found for {self.mc_version}")

        return yarn_dir, intermediary_file, mojang_file

    def load_yarn_mappings(self, yarn_dir):
        """Load Yarn mappings from .mapping files."""
        print("Loading Yarn mappings...")
        mappings_dir = yarn_dir / "mappings"

        if not mappings_dir.exists():
            raise ValueError(f"Mappings directory not found: {mappings_dir}")

        for mapping_file in mappings_dir.rglob("*.mapping"):
            with open(mapping_file, "r") as f:
                current_class_intermediary = None
                current_class_yarn = None

                for line in f:
                    line = line.strip()
                    if not line or line.startswith("#"):
                        continue

                    parts = line.split()
                    if not parts:
                        continue

                    if parts[0] == "CLASS":
                        # CLASS intermediary yarn
                        current_class_intermediary = parts[1]
                        current_class_yarn = parts[2] if len(parts) == 3 else parts[1]
                        self.yarn_to_intermediary[current_class_yarn] = (
                            current_class_intermediary
                        )

                    elif parts[0] == "FIELD" and current_class_yarn:
                        # FIELD intermediary yarn descriptor
                        intermediary_field = parts[1]
                        yarn_field = parts[2]
                        descriptor = parts[3] if len(parts) > 3 else ""
                        key = (current_class_yarn, yarn_field)
                        self.yarn_fields[key] = (
                            current_class_intermediary,
                            intermediary_field,
                            descriptor,
                        )

                    elif parts[0] == "METHOD" and current_class_yarn:
                        # METHOD intermediary yarn descriptor
                        intermediary_method = parts[1]
                        yarn_method = parts[2]
                        descriptor = parts[3] if len(parts) > 3 else ""
                        key = (current_class_yarn, yarn_method, descriptor)
                        self.yarn_methods[key] = (
                            current_class_intermediary,
                            intermediary_method,
                        )

    def load_intermediary_mappings(self, intermediary_file):
        """Load Intermediary mappings (Tiny v1 format)."""
        print("Loading Intermediary mappings...")
        with open(intermediary_file, "r") as f:
            lines = f.readlines()

        current_class = None

        for line in lines:
            line = line.strip()
            if not line:
                continue

            parts = line.split("\t")

            if parts[0] == "CLASS":
                # CLASS official intermediary obfuscated
                if len(parts) >= 3:
                    intermediary = parts[2]
                    obfuscated = parts[1]
                    self.intermediary_to_obf[intermediary] = obfuscated
                    current_class = intermediary

            elif parts[0] == "FIELD" and current_class:
                # FIELD descriptor intermediary obfuscated
                if len(parts) >= 4:
                    descriptor = parts[1]
                    intermediary = parts[3]
                    obfuscated = parts[2]
                    key = (current_class, intermediary, descriptor)
                    self.intermediary_fields[key] = obfuscated

            elif parts[0] == "METHOD" and current_class:
                # METHOD descriptor intermediary obfuscated
                if len(parts) >= 4:
                    descriptor = parts[1]
                    intermediary = parts[3]
                    obfuscated = parts[2]
                    key = (current_class, intermediary, descriptor)
                    self.intermediary_methods[key] = obfuscated

    def load_mojang_mappings(self, mojang_file):
        """Load Mojang ProGuard mappings."""
        print("Loading Mojang mappings...")
        with open(mojang_file, "r") as f:
            current_obf_class = None
            current_mojang_class = None

            for line in f:
                line = line.rstrip()
                if not line or line.startswith("#"):
                    continue

                # Class mapping: obfuscated -> mojang:
                if line.endswith(":") and not line.startswith("    "):
                    parts = line[:-1].split(" -> ")
                    if len(parts) == 2:
                        current_mojang_class = parts[0].replace(".", "/")
                        current_obf_class = parts[1].replace(".", "/")
                        self.obf_to_mojang[current_obf_class] = current_mojang_class

                # Field/Method mapping (indented)
                elif line.startswith("    ") and current_obf_class:
                    line = line.strip()
                    parts = line.split(" -> ")
                    if len(parts) == 2:
                        obf_name = parts[1]
                        left = parts[0]

                        # Check if it's a method (has parentheses)
                        if "(" in left:
                            # Method: returnType name(params) -> obfuscated
                            method_parts = left.split("(")
                            name_parts = method_parts[0].rsplit(" ", 1)
                            if len(name_parts) == 2:
                                mojang_name = name_parts[1]
                                key = (current_obf_class, obf_name)
                                self.mojang_methods[key] = mojang_name
                        else:
                            # Field: type name -> obfuscated
                            field_parts = left.rsplit(" ", 1)
                            if len(field_parts) == 2:
                                mojang_name = field_parts[1]
                                key = (current_obf_class, obf_name)
                                self.mojang_fields[key] = mojang_name

    def find_class(self, query):
        """Find class name(s) matching the query."""
        query = query.replace(".", "/")
        matches = []

        # Exact match
        if query in self.yarn_to_intermediary:
            matches.append(query)
        else:
            # Partial match - search by class name or full path
            for yarn_class in self.yarn_to_intermediary.keys():
                # Match by simple class name
                if query in yarn_class or yarn_class.endswith("/" + query):
                    matches.append(yarn_class)

        return matches

    def convert_class(self, yarn_class):
        """Convert Yarn class name to Mojang official."""
        intermediary = self.yarn_to_intermediary.get(yarn_class)
        if not intermediary:
            return None

        obfuscated = self.intermediary_to_obf.get(intermediary)
        if not obfuscated:
            return None

        return self.obf_to_mojang.get(obfuscated)

    def find_field(self, class_query, field_name):
        """Find field(s) matching the query."""
        classes = self.find_class(class_query)
        matches = []

        for yarn_class in classes:
            key = (yarn_class, field_name)
            if key in self.yarn_fields:
                matches.append((yarn_class, field_name))

        return matches

    def convert_field(self, yarn_class, yarn_field):
        """Convert Yarn field name to Mojang official."""
        key = (yarn_class, yarn_field)
        if key not in self.yarn_fields:
            return None

        intermediary_class, intermediary_field, descriptor = self.yarn_fields[key]

        # Get obfuscated field
        inter_key = (intermediary_class, intermediary_field, descriptor)
        obf_field = self.intermediary_fields.get(inter_key)
        if not obf_field:
            return None

        # Get obfuscated class
        obf_class = self.intermediary_to_obf.get(intermediary_class)
        if not obf_class:
            return None

        # Get mojang name
        mojang_key = (obf_class, obf_field)
        return self.mojang_fields.get(mojang_key)

    def find_method(self, class_query, method_name, descriptor=""):
        """Find method(s) matching the query."""
        classes = self.find_class(class_query)
        matches = []

        for yarn_class in classes:
            # Try with exact descriptor if provided
            if descriptor:
                key = (yarn_class, method_name, descriptor)
                if key in self.yarn_methods:
                    matches.append((yarn_class, method_name, descriptor))
            else:
                # Find all methods with this name in this class
                for key in self.yarn_methods.keys():
                    if key[0] == yarn_class and key[1] == method_name:
                        matches.append(key)

        return matches

    def convert_method(self, yarn_class, yarn_method, descriptor=""):
        """Convert Yarn method name to Mojang official."""
        key = (yarn_class, yarn_method, descriptor)
        if key not in self.yarn_methods:
            return None

        intermediary_class, intermediary_method = self.yarn_methods[key]

        # Get obfuscated method
        inter_key = (intermediary_class, intermediary_method, descriptor)
        obf_method = self.intermediary_methods.get(inter_key)
        if not obf_method:
            return None

        # Get obfuscated class
        obf_class = self.intermediary_to_obf.get(intermediary_class)
        if not obf_class:
            return None

        # Get mojang name
        mojang_key = (obf_class, obf_method)
        return self.mojang_methods.get(mojang_key)

    def setup(self):
        """Download and load all mappings."""
        yarn_dir, intermediary_file, mojang_file = self.download_mappings()
        self.load_yarn_mappings(yarn_dir)
        self.load_intermediary_mappings(intermediary_file)
        self.load_mojang_mappings(mojang_file)
        print("\nMappings loaded successfully!")
        print(f"  Yarn classes: {len(self.yarn_to_intermediary)}")
        print(f"  Yarn fields: {len(self.yarn_fields)}")
        print(f"  Yarn methods: {len(self.yarn_methods)}")


def main():
    parser = argparse.ArgumentParser(
        description="Convert Yarn names to Mojang official names",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Convert a class
  python script.py 1.21.4 class net/minecraft/MinecraftVersion

  # Convert a field
  python script.py 1.21.4 field net/minecraft/MinecraftVersion LOGGER

  # Convert a method
  python script.py 1.21.4 method net/minecraft/MinecraftVersion create ()Lnet/minecraft/class_6489;
        """,
    )

    parser.add_argument("version", help="Minecraft version (e.g., 1.21.4)")
    parser.add_argument(
        "type", choices=["class", "field", "method"], help="Type of mapping to convert"
    )
    parser.add_argument(
        "class_name",
        help="Yarn class name (supports partial: MinecraftVersion, minecraft/MinecraftVersion, net.minecraft.MinecraftVersion)",
    )
    parser.add_argument("member_name", nargs="?", help="Field or method name")
    parser.add_argument(
        "descriptor", nargs="?", default="", help="Method descriptor (optional)"
    )

    args = parser.parse_args()

    converter = YarnToMojangConverter(args.version)
    converter.setup()

    print("\n" + "=" * 60)

    if args.type == "class":
        matches = converter.find_class(args.class_name)

        if not matches:
            print(f"Could not find any class matching: {args.class_name}")
        elif len(matches) == 1:
            yarn_class = matches[0]
            result = converter.convert_class(yarn_class)
            if result:
                print(f"Yarn:   {yarn_class}")
                print(f"Mojang: {result}")
            else:
                print(f"Could not convert class: {yarn_class}")
        else:
            print(f"Multiple classes found matching '{args.class_name}':")
            for i, yarn_class in enumerate(matches, 1):
                result = converter.convert_class(yarn_class)
                print(f"\n{i}. Yarn:   {yarn_class}")
                if result:
                    print(f"   Mojang: {result}")

    elif args.type == "field":
        if not args.member_name:
            print("Error: Field name required")
            return

        matches = converter.find_field(args.class_name, args.member_name)

        if not matches:
            print(
                f"Could not find field '{args.member_name}' in any class matching: {args.class_name}"
            )
        elif len(matches) == 1:
            yarn_class, field_name = matches[0]
            result = converter.convert_field(yarn_class, field_name)
            if result:
                print(f"Yarn:   {yarn_class}.{field_name}")
                print(f"Mojang: {result}")
            else:
                print(f"Could not convert field: {yarn_class}.{field_name}")
        else:
            print(f"Multiple fields found:")
            for i, (yarn_class, field_name) in enumerate(matches, 1):
                result = converter.convert_field(yarn_class, field_name)
                print(f"\n{i}. Yarn:   {yarn_class}.{field_name}")
                if result:
                    print(f"   Mojang: {result}")

    elif args.type == "method":
        if not args.member_name:
            print("Error: Method name required")
            return

        matches = converter.find_method(
            args.class_name, args.member_name, args.descriptor
        )

        if not matches:
            print(
                f"Could not find method '{args.member_name}' in any class matching: {args.class_name}"
            )
        elif len(matches) == 1:
            yarn_class, method_name, desc = matches[0]
            result = converter.convert_method(yarn_class, method_name, desc)
            if result:
                print(f"Yarn:   {yarn_class}.{method_name}{desc}")
                print(f"Mojang: {result}")
            else:
                print(f"Could not convert method: {yarn_class}.{method_name}{desc}")
        else:
            print(f"Multiple methods found:")
            for i, (yarn_class, method_name, desc) in enumerate(matches, 1):
                result = converter.convert_method(yarn_class, method_name, desc)
                print(f"\n{i}. Yarn:   {yarn_class}.{method_name}{desc}")
                if result:
                    print(f"   Mojang: {result}")

    print("=" * 60)


if __name__ == "__main__":
    main()
