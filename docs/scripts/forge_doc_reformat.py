"""
This Python script is designed to modify the Markdown files generated by the 'forge doc' command.
The 'forge doc --out docs/docs/contracts' command doesn't return the docs in the right format for Docusaurus.
Therefore, run this script after running the `forge doc --out docs/docs/contracts` & before publishing the doc to make it compatible with Docusaurus.
"""

import os
import re

# Function to replace GitHub source URLs with the 'master' branch
def replace_git_sources(md_content):
    def replace_source(match):
        original_url = match.group(0)
        # Capture the commit hash or tag using a group in the regular expression
        commit_hash = match.group(1)
        replaced_url = original_url.replace(f'/blob/{commit_hash}/', '/blob/master/')
        return replaced_url

    # Updated regular expression to capture the commit hash or tag
    return re.sub(r'https://github.com/.+?/blob/([a-f0-9]+?)/.+?$', replace_source, md_content, flags=re.MULTILINE)

# Function to parse the Inherits line in the Markdown content
def parse_inherits(md_content):
    inherits_match = re.search(r'\*\*Inherits:\*\*\s*([\s\S]*?)\n', md_content)
    if inherits_match:
        inheritances = inherits_match.group(1).strip()
        return inheritances
    return None

# Function to replace paths inside brackets with empty brackets
def replace_paths_with_empty_brackets(line,file_path):
    def replace_path(match):
        default_path = 'docs/docs/src'
        path = default_path + match.group(0)[1:-1]  # Remove parentheses
        start = "docs/docs/src/src/X/X/"
        relative_path = os.path.relpath(path, start)
        print(f"path: {path}")
        print(f"line: {line}")
        print(f"file_path: {file_path}")
        print(f"Original Path: {relative_path}")
        if '/src/gateway/strategy/' in file_path:
            return '(../' + relative_path + ')'
        else:
            return '(' + relative_path + ')'
    return re.sub(r'\([^)]+\)', replace_path, line)

# Function to process a single Markdown file
def process_md_file(file_path):
    with open(file_path, 'r') as file:
        md_content = file.read()

    # Parse Inherits line
    inherits = parse_inherits(md_content)
    print(f"file_path: ",file_path);
    # Modify the Inherits line
    if inherits:
        modified_inherits = replace_paths_with_empty_brackets(inherits,file_path)
        print(f"modified_inherits: {modified_inherits}")
        md_content = md_content.replace(inherits, modified_inherits)

    md_content = replace_git_sources(md_content)

    # Write the modified content back to the original file
    with open(file_path, 'w') as file:
        file.write(md_content)

    print(f"File modified: {file_path}")

    # Check if the file name is README.md or SUMMARY.md and delete it
    file_name = os.path.basename(file_path)
    if file_name.lower() in ['readme.md', 'summary.md']:
        os.remove(file_path)
        print(f"Deleted file: {file_path}")

# Main function to process all Markdown files in a directory and its subdirectories
def process_all_md_files(directory):
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.md'):
                file_path = os.path.join(root, file)
                process_md_file(file_path)

if __name__ == "__main__":
    # Get the absolute path of the directory containing the script
    script_directory = os.path.dirname(os.path.abspath(__file__))

    # Specify the directory path relative to the script
    directory_path = os.path.join(script_directory, '../docs/contracts/src')

    # Process all Markdown files in the specified directory and its subdirectories
    process_all_md_files(directory_path)

    # Specific file path to process
    # file_to_process = "/Users/nakul/Desktop/Interlay_Work/bob/docs/scripts/../docs/contracts/src/src/gateway/strategy/AvalonStrategy.sol/contract.AvalonLstStrategy.md"
    # process_md_file(file_to_process)

    print("All Markdown files in the directory processed.")
