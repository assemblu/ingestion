#!/usr/bin/env python3
"""
Fix inconsistent blank lines in C++ source files for better navigation.
Ensures consistent spacing between logical sections.
"""

import re
import sys
from pathlib import Path

def fix_blank_lines(content):
    """Fix blank line inconsistencies in C++ code."""
    lines = content.split('\n')
    result = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # If line is only whitespace, make it completely empty
        if stripped == '':
            result.append('')
        else:
            result.append(line)
        
        # Look ahead to determine if we need spacing
        if i < len(lines) - 1:
            next_line = lines[i + 1].strip()
            
            # Add blank line after these patterns (if not already present)
            needs_blank_after = any([
                # After include blocks
                stripped.startswith('#include') and not next_line.startswith('#include'),
                # After namespace/using declarations  
                stripped.startswith('using ') and next_line and not next_line.startswith('using'),
                # After global variables
                (stripped.endswith(';') and not stripped.startswith('//') and 
                 i > 0 and not lines[i-1].strip().startswith('#include') and
                 next_line and next_line.startswith(('void ', 'int ', 'bool ', 'auto ', 'template'))),
                # Before function definitions
                (next_line.startswith(('void ', 'int ', 'bool ', 'auto ', 'template')) and 
                 not stripped.startswith(('void ', 'int ', 'bool ', 'auto ', 'template')) and
                 stripped != ''),
                # After closing braces of functions (not inside blocks)
                (stripped == '}' and i > 0 and 
                 count_indent(line) == 0 and next_line and next_line != ''),
                # Before major comments that start sections
                (next_line.startswith('// ') and len(next_line) > 20 and 
                 not stripped.startswith('//') and stripped != ''),
                # After try-catch blocks
                stripped.startswith('} catch') and next_line and not next_line.startswith('}'),
                # Before while/for/if at top level (main control flow)
                (next_line.strip().startswith(('while (', 'for (', 'if (')) and 
                 count_indent(lines[i + 1]) < 8 and stripped != '' and not stripped.endswith('{')),
            ])
            
            # Check if there's already a blank line
            has_blank = (i < len(lines) - 2 and lines[i + 1].strip() == '')
            
            if needs_blank_after and not has_blank and next_line:
                result.append('')
        
        i += 1
    
    # Clean up multiple consecutive blank lines and ensure blank lines are truly empty
    final = []
    blank_count = 0
    for line in result:
        if line.strip() == '':
            blank_count += 1
            if blank_count <= 1:  # Allow max 1 blank line
                final.append('')  # Ensure blank line is completely empty
        else:
            blank_count = 0
            final.append(line)
    
    return '\n'.join(final)

def count_indent(line):
    """Count leading spaces/tabs."""
    count = 0
    for char in line:
        if char == ' ':
            count += 1
        elif char == '\t':
            count += 4
        else:
            break
    return count

def process_file(filepath):
    """Process a single file."""
    path = Path(filepath)
    
    if not path.exists():
        print(f"Error: {filepath} does not exist")
        return False
    
    try:
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        fixed_content = fix_blank_lines(content)
        
        # Only write if changed
        if fixed_content != content:
            # Create backup
            backup_path = path.with_suffix(path.suffix + '.bak')
            with open(backup_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            # Write fixed content
            with open(path, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
            
            print(f"✓ Fixed {filepath} (backup: {backup_path})")
            return True
        else:
            print(f"✓ {filepath} - no changes needed")
            return False
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        print("Usage: python fix_blank_lines.py <file1> [file2] ...")
        print("   or: python fix_blank_lines.py *.cpp")
        sys.exit(1)
    
    from glob import glob
    files = sys.argv[1:]
    fixed_count = 0
    all_files = []
    
    for file_pattern in files:
        # Handle wildcards
        matching_files = glob(file_pattern) if '*' in file_pattern else [file_pattern]
        all_files.extend(matching_files)
    
    # Remove duplicates and sort
    all_files = sorted(set(all_files))
    
    for filepath in all_files:
        if process_file(filepath):
            fixed_count += 1
    
    print(f"\n✓ Fixed {fixed_count} file(s)")

if __name__ == "__main__":
    main()