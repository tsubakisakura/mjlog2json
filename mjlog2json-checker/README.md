# mjlog2json-checker

Verify that mjlog2json conversion matches the official xml and json.

# Usage

1. Download official xml and json to same folder.
2. Run ```cargo run --release -p mjlog2json-checker async <<folder_name>>```
3. Check the difference between ```actual.txt``` and ```expected.txt``` using a diff tool.
